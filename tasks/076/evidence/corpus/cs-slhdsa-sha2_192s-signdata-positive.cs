class Sample { void Run() { var k = System.Security.Cryptography.SlhDsa.GenerateKey(System.Security.Cryptography.SlhDsaAlgorithm.SlhDsaSha2_192s); k.SignData(data); } }
