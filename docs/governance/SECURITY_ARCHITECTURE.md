# SECURITY BY DESIGN ARCHITECTURE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: MANDATORY SECURITY GOVERNANCE SPECIFICATION  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Executive Summary & Standards Alignment

This document establishes the **Security by Design Architecture** for **Siragugal Film Studio**. The architecture strictly complies with international security engineering standards:

- **OWASP ASVS Level 2** (Application Security Verification Standard)
- **OWASP Top 10** & **OWASP Proactive Controls**
- **CWE Top 25 Most Dangerous Software Weaknesses**
- **NIST SSDF SP 800-218** (Secure Software Development Framework)
- **SLSA Level 3** (Supply-Chain Levels for Software Artifacts)
- **CIS Secure Development Guidelines**

---

## 2. Zero Trust Architecture & Defense-in-Depth

Siragugal Film Studio operates under a strict **Zero Trust Architecture**. Every subsystem, IPC boundary, plugin sandbox, database query, and external API request assumes zero implicit trust.

```
[ User Input / GUI ] ──► (Input Validation Gate) ──► [ Experience Layer ]
                                                            │ (Permission Gate)
                                                            ▼
[ WASM Plugin Sandbox ] ◄── (10-Tier Permission SIRA-6004) ──┤ SIRA Core Runtime
                                                            │ (Keychain Auth)
                                                            ▼
[ AI Providers ] ◄────── (TLS 1.3 + SHA-256 Checksum) ──────┘
```

---

## 3. 20 Security Core Architecture Controls

1. **Secure Coding**: Prohibits `unsafe` Rust blocks unless explicitly justified; enforces TypeScript `strict` mode (`noImplicitAny`); mandates modern C++20 RAII.
2. **Input Validation**: Enforces canonicalization and schema validation on all project files (`.sfsp`), workflows (`.sfsw`), IPC messages, CLI arguments, and file paths.
3. **Authentication**: OAuth2 / OpenID Connect support for cloud APIs; OS Keychain credential storage (macOS Keychain / Windows Credential Manager).
4. **Authorization**: Default-deny 10-tier permission model (`sira.permission.*`).
5. **Approved Cryptography**: AES-256-GCM, Ed25519, SHA-256, SHA-512, Argon2id, TLS 1.3. Obsolete ciphers (MD5, SHA1, DES, RC4) strictly prohibited.
6. **Secrets Management**: Zero secrets in source code, config files, git repositories, or logs.
7. **IPC Security**: gRPC over Unix Domain Sockets (`0600` permissions) with schema validation and replay protection.
8. **Plugin Sandboxing**: Wasmtime WASI WebAssembly sandbox trapping unauthorized I/O.
9. **AI Provider Security**: SHA-256 model weight checksum verification (`SIRA-3008`), rate limiting, and fallback chains.
10. **File Security**: Canonical path validation preventing Zip Slip, Symlink attacks, and Path Traversal (`../`).
11. **Database Security**: 100% parameterized SQL queries; SQLite WAL mode integrity & foreign key enforcement.
12. **Log Redaction**: Automatic regex-based redaction engine stripping API keys (`sk-...`) and tokens before writing log streams.
13. **Memory Security**: Sensitive memory zeroization (`zeroize` crate), bounded memory allocations, and VRAM OOM protection.
14. **Supply Chain Security**: SBOM generation (`spdx`), pinned lockfiles (`Cargo.lock`, `pnpm-lock.yaml`), and dependency vulnerability scans (`cargo audit`, `npm audit`).
15. **Static Analysis CI Gates**: Mandatory CI failure on High or Critical vulnerabilities (`cargo clippy`, `cargo deny`, `CodeQL`, `Semgrep`, `Trivy`).
16. **Security Testing**: Fuzz testing (`cargo fuzz`), unit security tests, property testing, and negative tests.
17. **STRIDE Threat Modeling**: Maintained threat models for all 10 core operational domains.
18. **PR Security Review Checklist**: Security checklists mandatory for every pull request.
19. **Release Gates**: Zero Critical / Zero High vulnerability release blocker rule.
20. **Security Incident Response**: Standardized incident response plan (`SECURITY_INCIDENT_RESPONSE.md`).
