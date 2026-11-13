use pqc_scan_probe_ssh::{
    encode_kexinit, encode_packet, parse_identification, parse_kexinit, parse_packet,
};
use serde_json::{json, Value};
fn inspect(bytes: &[u8]) -> Value {
    let identification = parse_identification(bytes);
    if let Ok(text) = &identification {
        assert!(text.starts_with("SSH-"));
        assert!(!text.contains('\n'));
    }
    let packet = parse_packet(bytes);
    if let Ok(payload) = packet {
        assert!(!payload.is_empty());
        let padding = vec![0; bytes[4] as usize];
        let encoded = encode_packet(payload, &padding).unwrap();
        assert_eq!(parse_packet(&encoded).unwrap(), payload);
    }
    let kex = parse_kexinit(bytes);
    json!({"identification":if identification.is_ok(){"accepted"}else{"rejected"},"packet":if packet.is_ok(){"accepted"}else{"rejected"},"kex":if kex.is_ok(){"accepted"}else{"rejected"}})
}
#[test]
fn positive_and_negative_protocol_controls() {
    assert_eq!(inspect(b"SSH-2.0-public\r\n")["identification"], "accepted");
    assert_eq!(inspect(b"garbage")["packet"], "rejected");
    assert_eq!(inspect(&encode_kexinit([0; 16]))["kex"], "accepted");
}
#[test]
#[ignore = "invoked per input by the bounded TASK-047 supervisor"]
fn quality047_worker() {
    let result = std::panic::catch_unwind(|| {
        let bytes = std::fs::read(std::env::var("PQC_FUZZ_INPUT").unwrap()).unwrap();
        assert!(bytes.len() <= 1024 * 1024);
        inspect(&bytes)
    });
    match result {
        Ok(value) => println!("PQC047:{value}"),
        Err(_) => std::process::exit(42),
    }
}
