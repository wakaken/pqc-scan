class Sample { void Run() { var k = new Org.BouncyCastle.Crypto.Signers.SlhDsaSigner(Org.BouncyCastle.Crypto.Parameters.SlhDsaParameters.slh_dsa_sha2_128s,false); k.GenerateSignature(data); } }
