const https = require('https');
const crypto = require('crypto');
const jwt = require('jsonwebtoken');
const { importPKCS8, SignJWT } = require('jose');
const { buildTokenService } = require('./services/tokenService');

const service = buildTokenService();

const payload = { sub: 'alice', aud: 'payments-api', kid: 'legacy-rsa-key' };
const token = jwt.sign(payload, 'dev-secret', {
  algorithm: 'RS256',
  keyid: 'legacy-rsa-key',
});

const signer = crypto.createSign('RSA-SHA256');
signer.update(JSON.stringify(payload));

const tlsAgent = new https.Agent({
  minVersion: 'TLSv1.0',
  ciphers: 'TLS_RSA_WITH_AES_128_CBC_SHA',
});

async function issueJoseToken() {
  const pkcs8 = `-----BEGIN PRIVATE KEY-----\nMIIBVQIBADANBgkqhkiG9w0BAQEFAASCAT8wggE7AgEAAkEA\n-----END PRIVATE KEY-----`;
  const key = await importPKCS8(pkcs8, 'RS256');
  return new SignJWT({ role: 'admin' })
    .setProtectedHeader({ alg: 'RS256', kid: 'legacy-rsa-key' })
    .setIssuedAt()
    .sign(key);
}

issueJoseToken().catch(() => undefined);
console.log(service.legacyProfile(), token, signer, tlsAgent.options.minVersion);
