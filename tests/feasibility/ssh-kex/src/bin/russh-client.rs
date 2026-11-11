//! Standalone feasibility client. No userauth/channel API is called.
use russh::{
    client,
    keys::{PublicKey, PublicKeyOrCertificate},
    Names,
};
use serde_json::json;
use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
    time::Duration,
};
#[derive(Default)]
struct Facts {
    pin_checked: bool,
    pin_matched: bool,
    kex: Option<String>,
}
struct Handler {
    expected: Option<PublicKey>,
    facts: Arc<Mutex<Facts>>,
}
impl client::Handler for Handler {
    type Error = russh::Error;
    async fn check_server_key(
        &mut self,
        key: &PublicKeyOrCertificate,
    ) -> Result<bool, Self::Error> {
        // Certificates deliberately unsupported: never trust their embedded key alone.
        let matched = match (key, &self.expected) {
            (PublicKeyOrCertificate::PublicKey { key, .. }, Some(expected)) => {
                key.key_data() == expected.key_data()
            }
            _ => false,
        };
        let mut f = self.facts.lock().unwrap();
        f.pin_checked = true;
        f.pin_matched = matched;
        Ok(matched)
    }
    async fn kex_done(
        &mut self,
        _secret: Option<&[u8]>,
        names: &Names,
        _session: &mut client::Session,
    ) -> Result<(), Self::Error> {
        // Never inspect, copy, log or retain the shared secret.
        self.facts.lock().unwrap().kex = Some(names.kex.as_ref().to_owned());
        Ok(())
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 4);
    let port: u16 = args[1].parse().unwrap();
    let expected = if args[2] == "unknown" {
        None
    } else {
        Some(PublicKey::from_openssh(&std::fs::read_to_string(&args[2]).unwrap()).unwrap())
    };
    let algorithm = match args[3].as_str() {
        "hybrid" => russh::kex::MLKEM768X25519_SHA256,
        "classical" => russh::kex::CURVE25519,
        _ => panic!("unsupported fixed study profile"),
    };
    let facts = Arc::new(Mutex::new(Facts::default()));
    let h = Handler {
        expected,
        facts: facts.clone(),
    };
    let mut config = client::Config::default();
    config.preferred.kex = Cow::Owned(vec![algorithm]);
    config.inactivity_timeout = Some(Duration::from_secs(3));
    config.keepalive_interval = None;
    let outcome = tokio::time::timeout(Duration::from_secs(5), async {
        // Loopback numeric destination only. Production policy integration is out of scope.
        let stream = tokio::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port))
            .await
            .map_err(russh::Error::IO)?;
        let session = client::connect_stream(Arc::new(config), stream, h).await?;
        session
            .disconnect(russh::Disconnect::ByApplication, "study complete", "")
            .await?;
        // connect_stream returns only after initial verified NEWKEYS. A subsequent
        // disconnect/EOF is cleanup, not a failed key exchange. Bound that wait.
        let _ = tokio::time::timeout(Duration::from_secs(1), session).await;
        Ok::<(), russh::Error>(())
    })
    .await;
    let (completed, error) = match outcome {
        Ok(Ok(())) => (true, "none".to_owned()),
        Ok(Err(e)) => (
            false,
            match e {
                russh::Error::UnknownKey => "untrusted_host",
                russh::Error::WrongServerSig => "invalid_signature",
                russh::Error::NoCommonAlgo { .. } => "no_overlap",
                _ => "transport_error",
            }
            .to_owned(),
        ),
        Err(_) => (false, "timeout".to_owned()),
    };
    let f = facts.lock().unwrap();
    println!(
        "{}",
        json!({"completed":completed,"pin_checked":f.pin_checked,"pin_matched":f.pin_matched,"selected_kex":if completed { f.kex.as_deref() } else {None},"error_class":error})
    );
    if !completed {
        std::process::exit(2);
    }
}
