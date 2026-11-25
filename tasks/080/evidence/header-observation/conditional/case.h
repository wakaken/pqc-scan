void f(){EVP_PKEY_CTX *ctx;
#if FEATURE
ctx=EVP_PKEY_CTX_new_from_name(NULL,"ML-KEM-768",NULL);
#else
ctx=EVP_PKEY_CTX_new_from_name(NULL,"ML-KEM-1024",NULL);
#endif
EVP_PKEY_encapsulate(ctx,data);}