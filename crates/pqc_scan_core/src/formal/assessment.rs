use super::*;

#[kani::proof]
#[kani::unwind(3)]
fn short_unknown_identifiers_stay_unknown() {
    let byte: u8 = kani::any();
    kani::assume(byte.is_ascii());
    let bytes = [b'?', byte];
    let id = std::str::from_utf8(&bytes).unwrap();
    let a = assess_algorithm(id, &BTreeMap::new());
    assert_eq!(a.quantum, QuantumExposure::Unknown);
    assert!(!a.parameters_known);
}

// Separate harnesses avoid multiplying the allocator state across catalog entries.
macro_rules! classical {
    ($name:ident, $id:literal) => {
        #[kani::proof]
        #[kani::unwind(8)]
        fn $name() {
            let a = assess_algorithm($id, &BTreeMap::new());
            assert_eq!(a.quantum, QuantumExposure::Vulnerable);
        }
    };
}
classical!(rsa_vulnerable, "RSA");
classical!(ec_vulnerable, "EC");
classical!(dh_vulnerable, "DH");
classical!(dsa_vulnerable, "DSA");
classical!(ecdh_vulnerable, "ECDH");
classical!(ecdsa_vulnerable, "ECDSA");
classical!(ed25519_vulnerable, "Ed25519");
classical!(x25519_vulnerable, "X25519");

macro_rules! generic_pqc {
    ($name:ident, $id:literal) => {
        #[kani::proof]
        #[kani::unwind(11)]
        fn $name() {
            let a = assess_algorithm($id, &BTreeMap::new());
            assert_eq!(a.quantum, QuantumExposure::Unknown);
            assert!(!a.parameters_known);
        }
    };
}
generic_pqc!(ml_kem_needs_parameters, "ML-KEM");
generic_pqc!(ml_dsa_needs_parameters, "ML-DSA");
generic_pqc!(slh_dsa_needs_parameters, "SLH-DSA");
