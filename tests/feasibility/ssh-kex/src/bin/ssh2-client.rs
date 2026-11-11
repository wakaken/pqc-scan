//! Negative candidate: Session::handshake may itself send a service request.
use serde_json::json;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut session = ssh2::Session::new().unwrap();
    let supported = session.supported_algs(ssh2::MethodType::Kex).unwrap();
    if args.len() == 1 {
        println!(
            "{}",
            json!({"local_supported_kex":supported,"hybrid_available":supported.iter().any(|s|s.contains("mlkem")||s.contains("sntrup"))})
        );
        return;
    }
    let port: u16 = args[1].parse().unwrap();
    session.set_timeout(3000);
    session
        .method_pref(
            ssh2::MethodType::Kex,
            if args[3] == "classical" {
                "curve25519-sha256"
            } else {
                "mlkem768x25519-sha256"
            },
        )
        .unwrap();
    let stream = std::net::TcpStream::connect_timeout(
        &format!("127.0.0.1:{port}").parse().unwrap(),
        std::time::Duration::from_secs(3),
    )
    .unwrap();
    session.set_tcp_stream(stream);
    let handshake = session.handshake().is_ok();
    let expected =
        russh::keys::PublicKey::from_openssh(&std::fs::read_to_string(&args[2]).unwrap())
            .unwrap()
            .to_bytes()
            .unwrap();
    let matched = handshake && session.host_key().is_some_and(|(key, _)| key == expected);
    let kex = session.methods(ssh2::MethodType::Kex).map(str::to_owned);
    println!(
        "{}",
        json!({"handshake":handshake,"pin_matched":matched,"selected_kex":if matched {kex} else {None},"authenticated":session.authenticated()})
    );
    let _ = session.disconnect(None, "study complete", None);
}
