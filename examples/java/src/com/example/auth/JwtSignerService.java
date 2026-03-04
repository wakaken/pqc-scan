package com.example.auth;

import java.nio.charset.StandardCharsets;
import java.security.KeyFactory;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.Signature;
import java.util.Base64;

import javax.crypto.Cipher;

public class JwtSignerService {
  private final String algorithm = "RS256";

  public String issueToken(String subject) throws Exception {
    KeyPairGenerator generator = KeyPairGenerator.getInstance("RSA");
    generator.initialize(2048);
    KeyPair pair = generator.generateKeyPair();

    Signature signature = Signature.getInstance("SHA256withRSA");
    signature.initSign(pair.getPrivate());

    Cipher cipher = Cipher.getInstance("RSA/ECB/OAEPWithSHA-256AndMGF1Padding");
    cipher.init(Cipher.ENCRYPT_MODE, pair.getPublic());

    KeyFactory.getInstance("RSA");

    String header = Base64.getUrlEncoder().withoutPadding()
        .encodeToString("{\"alg\":\"RS256\",\"kid\":\"legacy-rsa-key\"}".getBytes(StandardCharsets.UTF_8));
    String payload = Base64.getUrlEncoder().withoutPadding()
        .encodeToString(("{\"sub\":\"" + subject + "\"}").getBytes(StandardCharsets.UTF_8));

    signature.update((header + "." + payload).getBytes(StandardCharsets.UTF_8));
    String sig = Base64.getUrlEncoder().withoutPadding().encodeToString(signature.sign());

    return header + "." + payload + "." + sig + "." + algorithm;
  }
}
