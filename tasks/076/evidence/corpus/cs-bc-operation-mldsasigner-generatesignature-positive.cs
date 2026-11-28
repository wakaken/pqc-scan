class Sample { void Run() { var k = new Org.BouncyCastle.Crypto.Signers.MLDsaSigner(Org.BouncyCastle.Crypto.Parameters.MLDsaParameters.ml_dsa_65,false); k.GenerateSignature(data); } }
