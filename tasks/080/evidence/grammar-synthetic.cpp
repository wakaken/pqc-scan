#include <oqs/oqs.h>
#include <openssl/evp.h>
void f(EVP_PKEY *incoming){ EVP_PKEY_CTX *ctx=EVP_PKEY_CTX_new_from_name(NULL,"ML-KEM-768",NULL); EVP_PKEY *key=NULL; EVP_PKEY_keygen(ctx,&key); EVP_DigestSignInit(md,NULL,NULL,NULL,key); oqs::KeyEncapsulation k("ML-KEM-768"); k.encap_secret(data); auto q=oqs::Signature("ML-DSA-65"); q.sign(data); ctx=other; }
#define RSA_new fake
