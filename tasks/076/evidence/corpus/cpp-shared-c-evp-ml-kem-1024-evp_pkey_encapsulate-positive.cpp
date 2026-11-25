void sample(void) { EVP_PKEY_CTX *k = EVP_PKEY_CTX_new_from_name(NULL,"ML-KEM-1024",NULL); EVP_PKEY_encapsulate(k, arg); }
