//! Test-only record ingestion through the same verified rustls config as TlsProbe.
use super::*;
fn inspect(bytes: &[u8]) -> &'static str {
    let probe = TlsProbe::new(&TlsOptions {
        verification_name: "localhost".into(),
        ca_file: None,
    })
    .unwrap();
    let mut connection = rustls::ClientConnection::new(probe.config, probe.name).unwrap();
    let mut reader = std::io::Cursor::new(bytes);
    match connection.read_tls(&mut reader) {
        Ok(_) => match connection.process_new_packets() {
            Ok(_) => {
                assert!(connection.is_handshaking());
                "incomplete"
            }
            Err(error) => {
                let _ = tls_diagnostic(error);
                "rejected"
            }
        },
        Err(_) => "io_rejected",
    }
}
#[test]
fn malformed_record_is_rejected_and_empty_record_stays_incomplete() {
    assert_eq!(inspect(&[]), "incomplete");
    assert_eq!(inspect(b"not TLS"), "rejected");
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
        Ok(state) => println!("PQC047:{{\"tls\":\"{state}\"}}"),
        Err(_) => std::process::exit(42),
    }
}
