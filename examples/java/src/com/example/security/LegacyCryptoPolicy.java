package com.example.security;

import javax.net.ssl.SSLContext;

public class LegacyCryptoPolicy {
  public String preferredCipherSuite() throws Exception {
    SSLContext context = SSLContext.getInstance("TLSv1.1");
    context.init(null, null, null);

    String rsaSuite = "TLS_RSA_WITH_AES_128_CBC_SHA";
    String ecdsaSuite = "ECDSA";
    String pkiAlgorithm = "sha256WithRSAEncryption";

    return rsaSuite + "|" + ecdsaSuite + "|" + pkiAlgorithm;
  }
}
