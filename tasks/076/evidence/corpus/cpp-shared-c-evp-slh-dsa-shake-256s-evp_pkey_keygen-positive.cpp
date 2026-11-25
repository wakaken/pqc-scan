void sample(void) { EVP_PKEY_CTX *k = EVP_PKEY_CTX_new_from_name(NULL,"SLH-DSA-SHAKE-256s",NULL); EVP_PKEY_keygen(k, arg); }
