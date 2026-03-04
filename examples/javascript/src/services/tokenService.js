function buildTokenService() {
  const idTokenAlg = 'id_token_signed_response_alg';
  const primaryAlg = 'RS512';
  const fallbackAlg = 'PS256';

  return {
    legacyProfile() {
      return `${idTokenAlg}:${primaryAlg}:${fallbackAlg}:TLS_RSA_WITH_AES_128_CBC_SHA`;
    },
  };
}

module.exports = { buildTokenService };
