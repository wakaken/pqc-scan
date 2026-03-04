require "openssl"
require "jwt"
require_relative "services/auth_service"

service = Services::AuthService.new
payload = { sub: "user-123", aud: "example-service", kid: "legacy-rsa-key" }

private_key = OpenSSL::PKey::RSA.new(service.private_key_pem)
token = JWT.encode(payload, private_key, "RS256", kid: "legacy-rsa-key")

ctx = OpenSSL::SSL::SSLContext.new
ctx.min_version = OpenSSL::SSL::TLS1_VERSION
ctx.ciphers = "TLS_RSA_WITH_AES_256_CBC_SHA"

puts [token[0, 18], service.id_token_signed_response_alg, ctx.min_version].join("|")
