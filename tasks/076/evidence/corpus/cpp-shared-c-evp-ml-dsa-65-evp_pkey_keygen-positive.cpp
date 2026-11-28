void sample(void) { EVP_PKEY_CTX *k = EVP_PKEY_CTX_new_from_name(NULL,"ML-DSA-65",NULL); EVP_PKEY_keygen(k, arg); }
