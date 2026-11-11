use pqc_scan_core::target_policy::{
    Cancellation, ProbeBudget, ProbeError, ProbeHost, ProbeLimits, Resolver, TargetPolicy,
    TargetRequest,
};
use std::net::IpAddr;
use std::time::Instant;

struct FakeResolver {
    answers: Vec<IpAddr>,
    calls: usize,
}
impl Resolver for FakeResolver {
    fn resolve(
        &mut self,
        _: &str,
        _: Instant,
        _: &Cancellation,
    ) -> Result<Vec<IpAddr>, ProbeError> {
        self.calls += 1;
        Ok(self.answers.clone())
    }
}
fn budget() -> ProbeBudget {
    ProbeBudget::new(ProbeLimits::default(), Cancellation::new()).unwrap()
}
fn ip(s: &str) -> IpAddr {
    s.parse().unwrap()
}
fn fake(answers: &[&str]) -> FakeResolver {
    FakeResolver {
        answers: answers.iter().map(|s| ip(s)).collect(),
        calls: 0,
    }
}
#[test]
fn rejects_url_control_wildcard_and_scope_syntax_before_resolution() {
    for host in [
        "https://example.com",
        "user@example.com",
        "x/y",
        "x\\y",
        "*.example.com",
        "x\n",
        " example.com",
        "fe80::1%en0",
        "[::1]:443",
        "127.1",
        "2130706433",
        "0x7f000001",
        "",
    ] {
        assert!(TargetRequest::new(host, 443).is_err(), "accepted {host:?}");
    }
    assert!(TargetRequest::new("example.com", 0).is_err());
}
#[test]
fn strict_idna_and_ipv6_have_separate_destination_identity() {
    let r = TargetRequest::new("BÜCHER.example", 443).unwrap();
    assert_eq!(r.host(), &ProbeHost::Dns("xn--bcher-kva.example".into()));
    assert_eq!(
        TargetRequest::new("[::ffff:127.0.0.1]", 22).unwrap().host(),
        &ProbeHost::Ip(ip("127.0.0.1"))
    );
}
#[test]
fn validates_every_answer_and_keeps_host_separate_from_pinned_ip() {
    let p = TargetPolicy::new(&[]).unwrap();
    let mut r = fake(&["8.8.8.8", "1.1.1.1"]);
    let t = p
        .resolve(
            TargetRequest::new("example.com", 443).unwrap(),
            &mut r,
            &budget(),
        )
        .unwrap();
    assert_eq!(t.socket_addr().to_string(), "8.8.8.8:443");
    assert_eq!(t.host(), &ProbeHost::Dns("example.com".into()));
    assert_eq!(t.answers().len(), 2);
    assert_eq!(r.calls, 1);
    assert_eq!(
        t.check_peer("1.1.1.1:443".parse().unwrap()),
        Err(ProbeError::PeerMismatch)
    );
    assert!(t.check_peer("8.8.8.8:443".parse().unwrap()).is_ok());
}
#[test]
fn rejects_entire_mixed_answer_set_not_just_selected_address() {
    let mut r = fake(&["8.8.8.8", "::ffff:127.0.0.1"]);
    assert!(matches!(
        TargetPolicy::new(&[]).unwrap().resolve(
            TargetRequest::new("example.com", 443).unwrap(),
            &mut r,
            &budget()
        ),
        Err(ProbeError::AddressDenied)
    ));
}
#[test]
fn duplicate_answers_consume_limit_and_empty_resolution_fails() {
    let p = TargetPolicy::new(&[]).unwrap();
    for answers in [vec![], vec![ip("8.8.8.8"); 9]] {
        let mut r = FakeResolver { answers, calls: 0 };
        assert!(p
            .resolve(
                TargetRequest::new("example.com", 443).unwrap(),
                &mut r,
                &budget()
            )
            .is_err());
    }
    let mut r = FakeResolver {
        answers: vec![ip("8.8.8.8"); 8],
        calls: 0,
    };
    assert_eq!(
        p.resolve(
            TargetRequest::new("example.com", 443).unwrap(),
            &mut r,
            &budget()
        )
        .unwrap()
        .answers()
        .len(),
        1
    );
}
#[test]
fn literal_ip_bypasses_dns_but_never_policy() {
    let p = TargetPolicy::new(&[]).unwrap();
    let mut r = fake(&["1.1.1.1"]);
    assert!(p
        .resolve(
            TargetRequest::new("127.0.0.1", 22).unwrap(),
            &mut r,
            &budget()
        )
        .is_err());
    assert!(p
        .resolve(
            TargetRequest::new("8.8.8.8", 22).unwrap(),
            &mut r,
            &budget()
        )
        .is_ok());
    assert_eq!(r.calls, 0);
}
#[test]
fn nonpublic_and_tunnel_ranges_require_explicit_authorization() {
    for host in [
        "10.0.0.1",
        "100.64.0.1",
        "127.0.0.1",
        "169.254.169.254",
        "172.31.255.255",
        "192.168.1.2",
        "192.0.0.8",
        "192.0.2.1",
        "192.88.99.1",
        "198.18.0.1",
        "198.51.100.1",
        "203.0.113.1",
        "::1",
        "fc00::1",
        "fe80::1",
        "2001:db8::1",
        "64:ff9b::a00:1",
        "2002:a00:1::1",
        "3fff::1",
    ] {
        let mut r = fake(&[]);
        assert!(
            TargetPolicy::new(&[])
                .unwrap()
                .resolve(TargetRequest::new(host, 22).unwrap(), &mut r, &budget())
                .is_err(),
            "accepted {host}"
        );
        let cidr = format!("{host}/{}", if host.contains(':') { 128 } else { 32 });
        assert!(
            TargetPolicy::new(&[cidr])
                .unwrap()
                .resolve(TargetRequest::new(host, 22).unwrap(), &mut r, &budget())
                .is_ok(),
            "denied explicit {host}"
        );
    }
}
#[test]
fn broad_allowlist_never_permits_nonunicast_or_unspecified() {
    let p = TargetPolicy::new(&["0.0.0.0/0".into(), "::/0".into()]).unwrap();
    for host in [
        "0.0.0.0",
        "224.0.0.1",
        "239.255.255.255",
        "255.255.255.255",
        "::",
        "ff02::1",
        "::ffff:224.0.0.1",
    ] {
        assert!(
            matches!(
                p.resolve(
                    TargetRequest::new(host, 22).unwrap(),
                    &mut fake(&[]),
                    &budget()
                ),
                Err(ProbeError::AddressDenied)
            ),
            "accepted {host}"
        );
    }
}
#[test]
fn mapped_cidr_is_canonicalized_and_cannot_authorize_wrong_family() {
    let p = TargetPolicy::new(&["::ffff:127.0.0.0/104".into()]).unwrap();
    assert!(p
        .resolve(
            TargetRequest::new("127.2.3.4", 22).unwrap(),
            &mut fake(&[]),
            &budget()
        )
        .is_ok());
    for cidr in [
        "::ffff:127.0.0.0/80",
        "127.0.0.0/33",
        "::1/129",
        "x/8",
        "127.0.0.0",
        "127.0.0.0/+8",
    ] {
        assert!(
            TargetPolicy::new(&[cidr.into()]).is_err(),
            "accepted {cidr}"
        );
    }
}
#[test]
fn reduced_limits_only_and_cancelled_resolution_never_calls_resolver() {
    let mut limits = ProbeLimits::default();
    limits.max_received_bytes += 1;
    assert!(ProbeBudget::new(limits, Cancellation::new()).is_err());
    let cancellation = Cancellation::new();
    cancellation.cancel();
    let budget = ProbeBudget::new(ProbeLimits::default(), cancellation).unwrap();
    let mut r = fake(&["8.8.8.8"]);
    assert!(matches!(
        TargetPolicy::new(&[]).unwrap().resolve(
            TargetRequest::new("example.com", 443).unwrap(),
            &mut r,
            &budget
        ),
        Err(ProbeError::Cancelled)
    ));
    assert_eq!(r.calls, 0);
}
#[test]
fn one_budget_cannot_resolve_twice_and_deadline_overrides_resolver_result() {
    let p = TargetPolicy::new(&[]).unwrap();
    let b = budget();
    let mut r = fake(&["8.8.8.8"]);
    p.resolve(TargetRequest::new("example.com", 443).unwrap(), &mut r, &b)
        .unwrap();
    assert!(matches!(
        p.resolve(TargetRequest::new("example.com", 443).unwrap(), &mut r, &b),
        Err(ProbeError::AttemptLimit)
    ));
    assert_eq!(r.calls, 1);
    struct Late;
    impl Resolver for Late {
        fn resolve(
            &mut self,
            _: &str,
            _: Instant,
            _: &Cancellation,
        ) -> Result<Vec<IpAddr>, ProbeError> {
            std::thread::sleep(std::time::Duration::from_millis(15));
            Ok(vec![ip("8.8.8.8")])
        }
    }
    let b = ProbeBudget::new(
        ProbeLimits {
            dns_timeout: std::time::Duration::from_millis(5),
            ..ProbeLimits::default()
        },
        Cancellation::new(),
    )
    .unwrap();
    assert!(matches!(
        p.resolve(
            TargetRequest::new("example.com", 443).unwrap(),
            &mut Late,
            &b
        ),
        Err(ProbeError::DeadlineExceeded)
    ));
}

#[test]
fn limit_validation_covers_zero_and_every_ceiling() {
    let defaults = ProbeLimits::default();
    for select in 0..7 {
        let mut limits = defaults.clone();
        match select {
            0 => limits.max_dns_answers = 9,
            1 => limits.dns_timeout += std::time::Duration::from_nanos(1),
            2 => limits.tcp_timeout += std::time::Duration::from_nanos(1),
            3 => limits.protocol_timeout += std::time::Duration::from_nanos(1),
            4 => limits.overall_timeout += std::time::Duration::from_nanos(1),
            5 => limits.max_received_bytes += 1,
            _ => limits.max_sent_bytes += 1,
        }
        assert!(ProbeBudget::new(limits, Cancellation::new()).is_err());
    }
    assert!(ProbeBudget::new(
        ProbeLimits {
            dns_timeout: std::time::Duration::ZERO,
            ..defaults.clone()
        },
        Cancellation::new()
    )
    .is_err());
    assert!(ProbeBudget::new(
        ProbeLimits {
            max_dns_answers: 0,
            ..defaults.clone()
        },
        Cancellation::new()
    )
    .is_err());
    assert!(ProbeBudget::new(
        ProbeLimits {
            max_sent_bytes: 0,
            ..defaults
        },
        Cancellation::new()
    )
    .is_err());
}

#[test]
fn elapsed_deadline_takes_precedence_over_late_dns_error() {
    struct LateFailure;
    impl Resolver for LateFailure {
        fn resolve(
            &mut self,
            _: &str,
            _: Instant,
            _: &Cancellation,
        ) -> Result<Vec<IpAddr>, ProbeError> {
            std::thread::sleep(std::time::Duration::from_millis(15));
            Err(ProbeError::DnsFailed)
        }
    }
    let budget = ProbeBudget::new(
        ProbeLimits {
            dns_timeout: std::time::Duration::from_millis(5),
            ..ProbeLimits::default()
        },
        Cancellation::new(),
    )
    .unwrap();
    assert!(matches!(
        TargetPolicy::new(&[]).unwrap().resolve(
            TargetRequest::new("example.com", 443).unwrap(),
            &mut LateFailure,
            &budget
        ),
        Err(ProbeError::DeadlineExceeded)
    ));
}
