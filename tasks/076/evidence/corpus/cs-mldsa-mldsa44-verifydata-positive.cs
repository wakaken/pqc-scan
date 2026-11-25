class Sample { void Run() { var k = System.Security.Cryptography.MLDsa.GenerateKey(System.Security.Cryptography.MLDsaAlgorithm.MLDsa44); k.VerifyData(data); } }
