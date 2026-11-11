use pqc_scan_core::probe::ProbeDiagnostic;
use pqc_scan_probe_ssh::{parse_identification, parse_kexinit, parse_packet, SshProbe};

fn payload(lists: [&str; 10], follows: u8, reserved: u32) -> Vec<u8> {
    let mut p = vec![20];
    p.extend_from_slice(&[0; 16]);
    for list in lists {
        p.extend_from_slice(&(list.len() as u32).to_be_bytes());
        p.extend_from_slice(list.as_bytes());
    }
    p.push(follows);
    p.extend_from_slice(&reserved.to_be_bytes());
    p
}
fn standard() -> [&'static str; 10] {
    [
        "curve25519-sha256",
        "ssh-ed25519",
        "aes128-ctr",
        "aes128-ctr",
        "hmac-sha2-256",
        "hmac-sha2-256",
        "none",
        "none",
        "",
        "",
    ]
}
fn frame(p: &[u8]) -> Vec<u8> {
    let mut pad = 8 - ((4 + 1 + p.len()) % 8);
    if pad < 4 {
        pad += 8;
    }
    let mut b = Vec::new();
    b.extend_from_slice(&((1 + p.len() + pad) as u32).to_be_bytes());
    b.push(pad as u8);
    b.extend_from_slice(p);
    b.resize(b.len() + pad, 0);
    b
}

#[test]
fn ordered_unknown_names_and_extension_markers_remain_distinct_advertisements() {
    let mut lists = standard();
    lists[0]="ext-info-s,mlkem768x25519-sha256,future@vendor.test,mlkem768x25519-sha256,kex-strict-s-v00@openssh.com";
    let (offers, follows) = parse_kexinit(&payload(lists, 2, 0)).unwrap();
    assert!(follows);
    assert_eq!(offers.kex_algorithms.len(), 5);
    assert_eq!(
        offers.extension_markers(),
        ["ext-info-s", "kex-strict-s-v00@openssh.com"]
    );
    let candidates = offers.kex_candidates();
    assert_eq!(candidates.len(), 3);
    assert_eq!(candidates[0].construction, "hybrid");
    assert_eq!(candidates[1].construction, "unknown");
    assert_eq!(candidates[0].name, candidates[2].name);
    assert_eq!(
        SshProbe::new().profile().purpose,
        "diagnostic_proposal_only"
    );
}

#[test]
fn identification_and_frame_boundaries_are_checked_before_payload_access() {
    assert_eq!(
        parse_identification(b"SSH-2.0-test\r\n").unwrap(),
        "SSH-2.0-test"
    );
    assert_eq!(
        parse_identification("SSH-2.0-test 日本語\r\n".as_bytes()).unwrap(),
        "SSH-2.0-test 日本語"
    );
    assert!(parse_identification("SSH-2.0-日本語 comment\r\n".as_bytes()).is_err());
    assert!(parse_identification(b"SSH-2.0-test bad\x00comment\r\n").is_err());
    assert!(parse_identification(b"SSH-2.0-test bad\x1bcomment\r\n").is_err());
    assert!(parse_identification(b"SSH-1.5-old\r\n").is_err());
    assert!(parse_identification(b"SSH-2.0-test\n").is_err());
    assert!(parse_identification(&[b'A'; 256]).is_err());
    let p = payload(standard(), 0, 0);
    let packet = frame(&p);
    assert_eq!(parse_packet(&packet).unwrap(), p);
    for n in 0..packet.len() {
        assert!(parse_packet(&packet[..n]).is_err());
    }
    let mut bad = packet.clone();
    bad[4] = 3;
    assert!(parse_packet(&bad).is_err());
    assert_eq!(
        parse_packet(&[255, 255, 255, 255, 4]),
        Err(ProbeDiagnostic::LocalResourceLimit)
    );
}

#[test]
fn malformed_and_excessive_name_lists_are_rejected_without_truncation() {
    for name in ["", ",bad", "bad,", "a,,b", "a b", "a\u{7f}b", "a@b@c", "é"] {
        let mut lists = standard();
        lists[0] = name;
        assert!(parse_kexinit(&payload(lists, 0, 0)).is_err(), "{name}");
    }
    let long = "a".repeat(65);
    let mut lists = standard();
    lists[0] = &long;
    assert_eq!(
        parse_kexinit(&payload(lists, 0, 0)),
        Err(ProbeDiagnostic::LocalResourceLimit)
    );
    let many = vec!["a"; 257].join(",");
    lists[0] = &many;
    assert_eq!(
        parse_kexinit(&payload(lists, 0, 0)),
        Err(ProbeDiagnostic::LocalResourceLimit)
    );
    assert!(parse_kexinit(&payload(standard(), 0, 1)).is_err());
    let mut extra = payload(standard(), 0, 0);
    extra.push(0);
    assert!(parse_kexinit(&extra).is_err());
}

#[test]
fn exact_limits_and_ssh199_compatibility_are_supported() {
    let mut line = b"SSH-2.0-".to_vec();
    line.extend(vec![b'A'; 245]);
    line.extend_from_slice(b"\r\n");
    assert_eq!(line.len(), 255);
    assert!(parse_identification(&line).is_ok());
    assert!(parse_identification(b"SSH-1.99-old_compatible\r\n").is_ok());
    assert!(parse_identification(b"SSH-1.99-old_compatible\n").is_ok());
    assert!(parse_identification(b"SSH-2.0-wrong-name\r\n").is_err());
    let exact = vec!["a".repeat(64); 256].join(",");
    let mut lists = standard();
    lists[0] = &exact;
    assert_eq!(
        parse_kexinit(&payload(lists, 255, 0))
            .unwrap()
            .0
            .kex_algorithms
            .len(),
        256
    );
    let p = vec![20; pqc_scan_probe_ssh::MAX_FRAME_BYTES - 9];
    let frame = pqc_scan_probe_ssh::encode_packet(&p, &[0; 4]).unwrap();
    assert_eq!(frame.len(), pqc_scan_probe_ssh::MAX_FRAME_BYTES);
    assert_eq!(parse_packet(&frame).unwrap().len(), p.len());
}

#[test]
fn deterministic_malformed_byte_mutations_never_panic_or_lose_successful_bounds() {
    let valid = payload(standard(), 0, 0);
    let packet = frame(&valid);
    for index in 0..valid.len() {
        for byte in [0, 1, 31, 127, 255] {
            let mut value = valid.clone();
            value[index] = byte;
            let _ = parse_kexinit(&value);
        }
    }
    for index in 0..packet.len() {
        for byte in [0, 1, 31, 127, 255] {
            let mut value = packet.clone();
            value[index] = byte;
            if let Ok(p) = parse_packet(&value) {
                assert!(!p.is_empty());
                assert!(p.len() < pqc_scan_probe_ssh::MAX_FRAME_BYTES);
            }
        }
    }
    let mut state = 0x0334253u64;
    for len in 0..1024 {
        let mut input = vec![0; len];
        for b in &mut input {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            *b = state as u8;
        }
        let _ = parse_packet(&input);
        let _ = parse_kexinit(&input);
    }
}
