void sample(void) { EVP_PKEY_CTX *k = EVP_PKEY_CTX_new_from_name(NULL,"SLH-DSA-SHAKE-192f",NULL); EVP_PKEY_keygen(k, arg); }
