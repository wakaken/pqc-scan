import { createVerify } from "node:crypto";
import jwt from "jsonwebtoken";
import { SignJWT } from "jose";
import { buildLegacyProfile } from "./services/tokenService";

const profile = buildLegacyProfile();
const verifier = createVerify("RSA-SHA512");

const token = jwt.sign(
  { sub: "ops-user", scope: "billing:read", kid: "legacy-rsa-key" },
  "dev-secret",
  { algorithm: "PS512", keyid: "legacy-rsa-key" },
);

async function mintJoseToken() {
  return new SignJWT({ tenant: "acme" })
    .setProtectedHeader({ alg: "RS256", kid: "legacy-rsa-key" })
    .setIssuedAt()
    .setIssuer("https://legacy-idp.internal")
    .sign(new TextEncoder().encode("not-a-real-key"));
}

mintJoseToken().catch(() => undefined);
console.log(profile, verifier, token.slice(0, 16));
