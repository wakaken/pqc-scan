# OpenForge.Cryptography.LibOqs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![.NET 8.0](https://img.shields.io/badge/.NET-8.0-512BD4)](https://dotnet.microsoft.com/download)
[![.NET 9.0](https://img.shields.io/badge/.NET-9.0-512BD4)](https://dotnet.microsoft.com/download)

A comprehensive .NET wrapper for the [liboqs](https://openquantumsafe.org/) quantum-safe cryptography library, providing post-quantum cryptographic algorithms for key encapsulation (KEM) and digital signatures. This library offers production-ready implementations of NIST-standardized post-quantum algorithms to protect against both classical and quantum computing threats.

## 🎯 Key Features

- **NIST-Standardized Algorithms**: Full support for ML-KEM (Kyber) and ML-DSA (Dilithium)
- **Comprehensive Algorithm Support**: Over 50 quantum-safe algorithms including BIKE, Classic McEliece, HQC, Falcon, SPHINCS+, and more
- **Modular Architecture**: Choose only the components you need to minimize dependencies
- **Cross-Platform**: Native binaries for Windows, Linux (including musl), and macOS on x64 and ARM64 architectures
- **Automatic Memory Safety**: Secure memory handling with automatic cleanup - no manual memory management required
- **High Performance**: Optimized native implementations via liboqs
- **Type-Safe API**: Strongly typed interfaces with comprehensive XML documentation

## 📦 Package Structure

The library follows a modular design, allowing you to install only what you need:

### Core Package
- **`OpenForge.Cryptography.LibOqs.Core`**: Shared P/Invoke definitions, common structures, and base functionality

### Functional Packages
Choose based on your cryptographic needs:
- **`OpenForge.Cryptography.LibOqs`**: Complete package with all algorithms (KEM + Signatures)
- **`OpenForge.Cryptography.LibOqs.KEM`**: Key Encapsulation Mechanisms only
- **`OpenForge.Cryptography.LibOqs.SIG`**: Digital Signatures only

### Native Binary Packages
Automatically included as dependencies:
- **`OpenForge.Cryptography.LibOqs.Native.Full`**: Complete native binaries (all algorithms)
- **`OpenForge.Cryptography.LibOqs.Native.KEM`**: KEM-only native binaries (smaller size)
- **`OpenForge.Cryptography.LibOqs.Native.Sig`**: Signature-only native binaries

## 🚀 Installation

Install the package that matches your requirements:

```bash
# For all quantum-safe algorithms
dotnet add package OpenForge.Cryptography.LibOqs

# For key exchange only (smaller footprint)
dotnet add package OpenForge.Cryptography.LibOqs.KEM

# For digital signatures only
dotnet add package OpenForge.Cryptography.LibOqs.SIG
```

## 📋 Requirements

- **.NET Runtime**: .NET 8.0 or .NET 9.0
- **Operating Systems**:
  - Windows 10+ (x64, ARM64)
  - Linux (x64, ARM64)
  - Linux with musl libc (Alpine Linux, x64, ARM64)
  - macOS 11+ (ARM64/Apple Silicon)
- **Memory**: Varies by algorithm (typically 1–100 MB for operations)

## 💻 Usage Examples

> **🔒 Memory Safety Note**: All examples use automatic memory management. Secret keys, shared secrets, and signatures are automatically cleared from memory when objects are disposed. No manual cleanup is required.

### Key Encapsulation (KEM) - Quantum-Safe Key Exchange

```csharp
using OpenForge.Cryptography.LibOqs.KEM;

// Initialize LibOQS before use
OpenForge.Cryptography.LibOqs.Core.OqsCore.Initialize();

// Use NIST-standardized ML-KEM (formerly Kyber)
using var kem = new Kem("ML-KEM-768");

// Generate key pair for the receiver
var (publicKey, secretKey) = kem.GenerateKeyPair();

// Sender: Encapsulate a shared secret using receiver's public key
var (ciphertext, sharedSecretSender) = kem.Encapsulate(publicKey);

// Receiver: Decapsulate to retrieve the same shared secret
var sharedSecretReceiver = kem.Decapsulate(ciphertext, secretKey);

// Both parties now have the same shared secret for symmetric encryption
Console.WriteLine($"Secrets match: {sharedSecretSender.SequenceEqual(sharedSecretReceiver)}");
```

### Digital Signatures - Quantum-Safe Authentication

```csharp
using OpenForge.Cryptography.LibOqs.SIG;
using System.Text;

// Initialize LibOQS before use
OpenForge.Cryptography.LibOqs.Core.OqsCore.Initialize();

// Use NIST-standardized ML-DSA (formerly Dilithium)
using var sig = new Sig("ML-DSA-65");

// Generate signing key pair
var (publicKey, secretKey) = sig.GenerateKeyPair();

// Sign a message
var message = Encoding.UTF8.GetBytes("Authenticate this message");
var signature = sig.Sign(message, secretKey);

// Verify the signature
bool isValid = sig.Verify(message, signature, publicKey);
Console.WriteLine($"Signature valid: {isValid}");

// Attempting to verify a tampered message will fail
message[0] ^= 0xFF; // Tamper with the message
bool isTampered = sig.Verify(message, signature, publicKey);
Console.WriteLine($"Tampered message detected: {!isTampered}");
```

### Hybrid Cryptography Example

```csharp
using OpenForge.Cryptography.LibOqs.KEM;
using OpenForge.Cryptography.LibOqs.Core;
using System.Security.Cryptography;

// Combine post-quantum KEM with classical AES for hybrid security
public static class HybridCrypto
{
    static HybridCrypto()
    {
        OqsCore.Initialize();
    }
    
    public static byte[] HybridEncrypt(byte[] data, byte[] kemPublicKey)
    {
        using var kem = new Kem("ML-KEM-768");
        
        // Generate post-quantum shared secret
        var (ciphertext, sharedSecret) = kem.Encapsulate(kemPublicKey);
        
        // Derive AES key from shared secret
        using var aes = Aes.Create();
        aes.Key = sharedSecret[..32]; // Use first 256 bits for AES-256
        aes.GenerateIV();
        
        // Encrypt data with AES
        using var encryptor = aes.CreateEncryptor();
        var encrypted = encryptor.TransformFinalBlock(data, 0, data.Length);
        
        // Return KEM ciphertext + AES IV + encrypted data
        return ciphertext.Concat(aes.IV).Concat(encrypted).ToArray();
    }
}
```

## 🔒 Supported Algorithms

### NIST-Standardized (Recommended for Production)

#### Key Encapsulation
- **ML-KEM** (FIPS 203): ML-KEM-512, ML-KEM-768, ML-KEM-1024
  - Formerly known as CRYSTALS-Kyber
  - Security levels 1, 3, and 5

#### Digital Signatures
- **ML-DSA** (FIPS 204): ML-DSA-44, ML-DSA-65, ML-DSA-87
  - Formerly known as CRYSTALS-Dilithium
  - Security levels 2, 3, and 5

### Additional Algorithms

#### Key Encapsulation
- **BIKE**: Round 5 candidate with small key sizes
- **Classic McEliece**: Conservative choice based on coding theory
- **HQC**: Hamming Quasi-Cyclic based
- **FrodoKEM**: Learning With Errors based
- **NTRU & NTRU Prime**: Lattice-based with long history
- **Saber**: Learning With Errors-based (LightSaber, Saber, FireSaber variants)

#### Digital Signatures
- **Falcon**: NIST alternate with very small signatures (including padded variants)
- **SPHINCS+**: Hash-based signatures with SHA2 and SHAKE variants
- **CROSS**: Code-based signatures with multiple parameter sets
- **MAYO**: Oil-and-vinegar based signatures
- **SNOVA**: Multivariate signatures with various configurations
- **UOV**: Unbalanced Oil and Vinegar signatures

#### Stateful Signatures (Special Use)
- **LMS**: Leighton-Micali Signatures (RFC 8554)
- **XMSS**: eXtended Merkle Signature Scheme (RFC 8391)
- **XMSS-MT**: Multi-tree variant of XMSS

*Note: Stateful signatures are available through the Core library's algorithm constants but require special handling for state management.*

## 🏗️ Architecture

The library is designed with modularity and safety in mind:

1. **Native Layer**: Platform-specific liboqs binaries with automatic runtime detection
2. **P/Invoke Layer**: Low-level bindings in Core package with secure native library loading
3. **Provider Layer**: Internal factories (`KemProvider`, `SigProvider`) managing native instances
4. **Managed Wrapper**: Type-safe, disposable C# classes (`Kem`, `Sig`) for end users  
5. **Algorithm Constants**: Strongly typed algorithm identifiers with deprecation warnings
6. **Security Utilities**: Constant-time operations, secure memory management, and validation

### Key Design Features
- **Zero Memory Management**: All cryptographic operations automatically handle secure memory allocation and cleanup
- **Automatic Initialization**: Native library resolver automatically handles platform-specific binaries  
- **Secure by Default**: All sensitive key material is automatically cleared from memory when disposed
- **Thread Safety**: Core initialization and library loading are thread-safe
- **Defensive Programming**: Comprehensive input validation and error handling

### Memory Management Philosophy
This library follows a **"secure by default"** approach to memory management:
- **No manual cleanup required**: All `Kem`, `Sig`, `KeyPair`, and result objects automatically clear sensitive data
- **RAII pattern**: Use `using` statements or `Dispose()` to ensure cleanup (automatic via finalizers if forgotten)
- **Memory pressure optimization**: Built-in hints for garbage collection when working with large-key algorithms
- **Expert APIs available**: Advanced users can access `OqsCore` methods for custom memory management scenarios
- **Performance optimized**: Memory operations are optimized but safety is never compromised

### Memory Pressure Management
For applications with specific memory constraints or when working with large-key algorithms:

```csharp
// Check memory requirements before choosing an algorithm
var memoryInfo = OqsCore.GetMemoryUsageInfo("Classic-McEliece-8192128");
Console.WriteLine($"Peak usage: {memoryInfo?.UsageDescription}");

// For batch operations or memory-constrained environments
if (memoryInfo?.RecommendMemoryPressureHints == true)
{
    // Hint to GC after large operations
    OqsCore.HintMemoryPressure(memoryInfo.Value.EstimatedPeakUsage);
}
```

## 🧪 Testing

The library includes comprehensive test coverage using:
- **xUnit** for the testing framework
- **FluentAssertions** for readable test assertions
- **NSubstitute** for mocking
- **Coverlet** for code coverage

Run tests with:
```bash
dotnet test
```

## 📖 Documentation

Full API documentation is generated from XML comments using DocFX. View the documentation:

```bash
# Build documentation
./build-docs.sh

# Serve documentation locally
./serve-docs.sh
```

## 🛡️ Security Considerations

1. **Algorithm Selection**: Use NIST-standardized algorithms (ML-KEM, ML-DSA) for production
2. **Key Storage**: Store secret keys securely, never in plain text
3. **Stateful Signatures**: Require careful state management to prevent reuse
4. **Hybrid Approach**: Consider combining with classical algorithms during transition
5. **Side Channels**: Native implementations include countermeasures but assess your threat model
6. **Deprecated Algorithms**: The library includes deprecated algorithms that should not be used in production:
   - **Rainbow**: Cryptographically broken - avoid entirely
   - **SIDH/SIKE**: Cryptographically broken - avoid entirely
   - Use `AlgorithmConstants.IsDeprecated()` to check algorithm status programmatically

## 🤝 Contributing

Contributions are welcome! Please ensure:
- All tests pass
- Code follows existing style conventions
- XML documentation is added for public APIs
- Security implications are considered

## 📄 License

This project is licensed under the MIT License—see the LICENSE file for details.

## 🙏 Acknowledgments

- [Open Quantum Safe](https://openquantumsafe.org/) project for liboqs
- NIST Post-Quantum Cryptography Standardization process
- The cryptographic research community

## ⚠️ Disclaimer

While this library implements cryptographic algorithms believed to be quantum-resistant, the field of post-quantum cryptography is evolving. Always consult with cryptographic experts for production deployments and stay updated with NIST recommendations.