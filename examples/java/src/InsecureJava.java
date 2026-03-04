import com.example.auth.JwtSignerService;
import com.example.security.LegacyCryptoPolicy;

public class InsecureJava {
  public static void main(String[] args) throws Exception {
    JwtSignerService signerService = new JwtSignerService();
    LegacyCryptoPolicy policy = new LegacyCryptoPolicy();

    String token = signerService.issueToken("user-123");
    String tlsSuite = policy.preferredCipherSuite();

    System.out.println(token.substring(0, 12) + "..." + tlsSuite);
  }
}
