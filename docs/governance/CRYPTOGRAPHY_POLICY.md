# CRYPTOGRAPHY & DATA PROTECTION POLICY
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: MANDATORY CRYPTOGRAPHIC SPECIFICATION  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Approved Cryptographic Algorithms

Siragugal Film Studio mandates the usage of modern, NIST-approved cryptographic primitives exclusively:

| Cryptographic Purpose | Approved Primitive / Standard | Implementation Library |
| :--- | :--- | :--- |
| **Symmetric Encryption** | AES-256-GCM (256-bit Key, 96-bit Nonce) | `aes-gcm` Rust crate |
| **Asymmetric Signatures** | Ed25519 (EdDSA Curve25519) | `ed25519-dalek` Rust crate |
| **Cryptographic Hashing** | SHA-256 / SHA-512 | `sha2` Rust crate |
| **Password / Key Derivation** | Argon2id (Memory: 64MB, Iterations: 3) | `argon2` Rust crate |
| **Transport Encryption** | TLS 1.3 (Cipher Suite: AES-256-GCM-SHA384) | `rustls` |

---

## 2. Forbidden Cryptographic Algorithms

> [!CAUTION]
> The following legacy or weak primitives are strictly prohibited in the codebase:
> - **MD5, SHA-1** (Collision vulnerable)
> - **DES, 3DES, RC4, Blowfish** (Weak ciphers)
> - **AES-ECB Mode** (Pattern leak vulnerability)
> - **RSA < 3072-bit** (Insufficient security margin)
