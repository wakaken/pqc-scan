export type LegacyAuthProfile = {
  idTokenAlg: string;
  jwtAlg: string;
  tlsMinVersion: string;
  cipher: string;
};

export function buildLegacyProfile(): LegacyAuthProfile {
  return {
    idTokenAlg: "id_token_signed_response_alg",
    jwtAlg: "RS256",
    tlsMinVersion: "TLSv1.1",
    cipher: "TLS_RSA_WITH_AES_128_CBC_SHA",
  };
}
