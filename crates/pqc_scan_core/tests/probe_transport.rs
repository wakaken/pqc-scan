#![cfg(feature = "probe-transport")]
use pqc_scan_core::{
    probe_transport::BoundedTcpStream,
    target_policy::{
        AuthorizedTarget, Cancellation, ProbeBudget, ProbeError, ProbeLimits, Resolver,
        TargetPolicy, TargetRequest,
    },
};
use std::{
    io::{Read, Write},
    net::{IpAddr, TcpListener},
    time::{Duration, Instant},
};
struct NoResolver;
impl Resolver for NoResolver {
    fn resolve(
        &mut self,
        _: &str,
        _: Instant,
        _: &Cancellation,
    ) -> Result<Vec<IpAddr>, ProbeError> {
        panic!("literal IP must not resolve")
    }
}
fn setup(
    limits: ProbeLimits,
    cancellation: Cancellation,
) -> (TcpListener, AuthorizedTarget, ProbeBudget) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let budget = ProbeBudget::new(limits, cancellation).unwrap();
    let target = TargetPolicy::new(&["127.0.0.0/8".into()])
        .unwrap()
        .resolve(
            TargetRequest::new("127.0.0.1", listener.local_addr().unwrap().port()).unwrap(),
            &mut NoResolver,
            &budget,
        )
        .unwrap();
    (listener, target, budget)
}
#[test]
fn exact_socket_bytes_are_counted_and_caller_buffers_clipped() {
    let limits = ProbeLimits {
        max_received_bytes: 4,
        max_sent_bytes: 3,
        ..ProbeLimits::default()
    };
    let (listener, target, budget) = setup(limits, Cancellation::new());
    let peer = std::thread::spawn(move || {
        let (mut peer, _) = listener.accept().unwrap();
        peer.write_all(b"abcdef").unwrap();
        let mut bytes = Vec::new();
        peer.read_to_end(&mut bytes).unwrap();
        bytes
    });
    let mut stream = BoundedTcpStream::connect(target, budget).unwrap();
    assert_eq!(stream.write_probe(b"012345").unwrap(), 3);
    let mut bytes = [0; 64];
    let n = stream.read_probe(&mut bytes).unwrap();
    assert!(n > 0 && n <= 4);
    let mut read = n;
    while read < 4 {
        read += stream.read_probe(&mut bytes[read..]).unwrap();
    }
    assert_eq!(stream.budget().received_bytes(), 4);
    assert_eq!(stream.budget().sent_bytes(), 3);
    assert_eq!(&bytes[..4], b"abcd");
    assert_eq!(stream.read_probe(&mut bytes), Err(ProbeError::TrafficLimit));
    drop(stream);
    assert_eq!(peer.join().unwrap(), b"012");
}
#[test]
fn cancellation_interrupts_stalled_read_and_closes_socket() {
    let cancel = Cancellation::new();
    let cancel_from_thread = cancel.clone();
    let (listener, target, budget) = setup(ProbeLimits::default(), cancel);
    let peer = std::thread::spawn(move || {
        let (mut peer, _) = listener.accept().unwrap();
        let mut byte = [0];
        peer.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
        peer.read(&mut byte).unwrap()
    });
    let mut stream = BoundedTcpStream::connect(target, budget).unwrap();
    let trigger = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(30));
        cancel_from_thread.cancel()
    });
    let started = Instant::now();
    assert_eq!(stream.read_probe(&mut [0]), Err(ProbeError::Cancelled));
    assert!(started.elapsed() < Duration::from_secs(1));
    trigger.join().unwrap();
    assert_eq!(peer.join().unwrap(), 0);
}
#[test]
fn protocol_deadline_does_not_extend_on_successful_reads() {
    let limits = ProbeLimits {
        protocol_timeout: Duration::from_millis(50),
        ..ProbeLimits::default()
    };
    let (listener, target, budget) = setup(limits, Cancellation::new());
    let peer = std::thread::spawn(move || {
        let (mut peer, _) = listener.accept().unwrap();
        for _ in 0..50 {
            if peer.write_all(b"x").is_err() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    });
    let mut stream = BoundedTcpStream::connect(target, budget).unwrap();
    let started = Instant::now();
    let mut received = 0;
    loop {
        match stream.read_probe(&mut [0]) {
            Ok(1) => received += 1,
            Err(ProbeError::DeadlineExceeded) => break,
            other => panic!("unexpected {other:?}"),
        }
    }
    assert!(received > 0);
    assert!(started.elapsed() < Duration::from_millis(400));
    peer.join().unwrap();
}
#[test]
fn cancelled_connect_sends_no_connection() {
    let cancel = Cancellation::new();
    let (listener, target, budget) = setup(ProbeLimits::default(), cancel.clone());
    cancel.cancel();
    assert!(matches!(
        BoundedTcpStream::connect(target, budget),
        Err(ProbeError::Cancelled)
    ));
    listener.set_nonblocking(true).unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
#[test]
fn trait_io_preserves_typed_failure_and_drop_closes_peer() {
    let (listener, target, budget) = setup(
        ProbeLimits {
            max_sent_bytes: 1,
            ..ProbeLimits::default()
        },
        Cancellation::new(),
    );
    let peer = std::thread::spawn(move || {
        let (mut peer, _) = listener.accept().unwrap();
        let mut bytes = Vec::new();
        peer.read_to_end(&mut bytes).unwrap();
        bytes
    });
    let mut stream = BoundedTcpStream::connect(target, budget).unwrap();
    stream.write_all(b"x").unwrap();
    let error = stream.write_all(b"y").unwrap_err();
    assert_eq!(
        error.get_ref().unwrap().downcast_ref::<ProbeError>(),
        Some(&ProbeError::TrafficLimit)
    );
    drop(stream);
    assert_eq!(peer.join().unwrap(), b"x");
}
#[test]
fn target_cannot_be_attached_to_a_fresh_budget() {
    let (listener, target, _old_budget) = setup(ProbeLimits::default(), Cancellation::new());
    let fresh = ProbeBudget::new(ProbeLimits::default(), Cancellation::new()).unwrap();
    assert!(matches!(
        BoundedTcpStream::connect(target, fresh),
        Err(ProbeError::InvalidTarget)
    ));
    listener.set_nonblocking(true).unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
#[test]
fn cancelling_idle_connection_shuts_peer_without_another_io_call() {
    let cancel = Cancellation::new();
    let (listener, target, budget) = setup(ProbeLimits::default(), cancel.clone());
    let stream = BoundedTcpStream::connect(target, budget).unwrap();
    let (mut peer, _) = listener.accept().unwrap();
    peer.set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();
    cancel.cancel();
    assert_eq!(peer.read(&mut [0]).unwrap(), 0);
    drop(stream);
}

#[test]
fn aggregate_deadline_wins_over_protocol_deadline() {
    let limits = ProbeLimits {
        overall_timeout: Duration::from_millis(30),
        protocol_timeout: Duration::from_secs(1),
        ..ProbeLimits::default()
    };
    let (listener, target, budget) = setup(limits, Cancellation::new());
    let mut stream = BoundedTcpStream::connect(target, budget).unwrap();
    let (mut peer, _) = listener.accept().unwrap();
    let start = Instant::now();
    assert_eq!(
        stream.read_probe(&mut [0]),
        Err(ProbeError::DeadlineExceeded)
    );
    assert!(start.elapsed() < Duration::from_millis(400));
    peer.set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();
    assert_eq!(peer.read(&mut [0]).unwrap(), 0);
}

#[test]
fn failed_flush_closes_socket_at_absolute_deadline() {
    let (listener, target, budget) = setup(
        ProbeLimits {
            protocol_timeout: Duration::from_millis(5),
            ..ProbeLimits::default()
        },
        Cancellation::new(),
    );
    let mut stream = BoundedTcpStream::connect(target, budget).unwrap();
    let (mut peer, _) = listener.accept().unwrap();
    std::thread::sleep(Duration::from_millis(15));
    assert!(stream.flush().is_err());
    peer.set_read_timeout(Some(Duration::from_millis(50)))
        .unwrap();
    assert_eq!(peer.read(&mut [0]).unwrap(), 0);
}

#[test]
fn idle_socket_closes_at_deadline_without_protocol_io() {
    let (listener, target, budget) = setup(
        ProbeLimits {
            protocol_timeout: Duration::from_millis(30),
            ..ProbeLimits::default()
        },
        Cancellation::new(),
    );
    let stream = BoundedTcpStream::connect(target, budget).unwrap();
    let (mut peer, _) = listener.accept().unwrap();
    peer.set_read_timeout(Some(Duration::from_millis(300)))
        .unwrap();
    assert_eq!(peer.read(&mut [0]).unwrap(), 0);
    drop(stream);
}

#[test]
fn dropping_stream_wakes_and_joins_deadline_watchdog_promptly() {
    let (listener, target, budget) = setup(ProbeLimits::default(), Cancellation::new());
    let stream = BoundedTcpStream::connect(target, budget).unwrap();
    let (_peer, _) = listener.accept().unwrap();
    let start = Instant::now();
    drop(stream);
    assert!(start.elapsed() < Duration::from_millis(300));
}
