#include "oqs_cpp.hpp"
void f(){oqs::Signature s{"ML-DSA-65"};s.sign("PRIVATE080_CLI_KEY_CANARY_abcdefghijklmnopqrstuvwxyz0123456789");oqs::KeyEncapsulation k("ML-KEM-512");k.encap_secret(data);}