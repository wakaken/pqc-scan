use ml_kem::*;
use ::kem::{Decapsulate, Encapsulate};
let mut rng = rand::thread_rng();

// Generate a (decapsulation key, encapsulation key) pair
let (dk, ek) = MlKem768::generate(&mut rng);

// Encapsulate a shared key to the holder of the decapsulation key, receive the shared
// secret `k_send` and the encapsulated form `ct`.
let (ct, k_send) = ek.encapsulate(&mut rng).unwrap();

// Decapsulate the shared key and verify that it was faithfully received.
let k_recv = dk.decapsulate(&ct).unwrap();
assert_eq!(k_send, k_recv);
