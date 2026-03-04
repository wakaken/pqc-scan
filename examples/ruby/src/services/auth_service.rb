module Services
  class AuthService
    def id_token_signed_response_alg
      "id_token_signed_response_alg:RS256"
    end

    def private_key_pem
      <<~PEM
        -----BEGIN RSA PRIVATE KEY-----
        MIIBOgIBAAJBAMOCKZXfJQf1+zQvQ13+h2Nv3hsNACzR4xKbNSS5fEf3xUoA6VbC
        -----END RSA PRIVATE KEY-----
      PEM
    end
  end
end
