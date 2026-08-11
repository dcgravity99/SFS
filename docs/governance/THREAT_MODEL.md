# STRIDE THREAT MODEL & SECURITY ARCHITECTURE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED SECURITY ARCHITECTURE AUDIT  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Executive Summary

This document presents the formal **STRIDE Threat Model** for **Siragugal Film Studio**. It evaluates security threats across 10 primary operational domains and establishes defense-in-depth mitigations.

---

## 2. STRIDE Matrix across 10 Target Domains

| Domain | Threat Classification | Threat Scenario Description | Risk Level | Defense-in-Depth Mitigation |
| :--- | :--- | :--- | :--- | :--- |
| **1. Project Files (`.sfsp`)** | **Tampering** | Malicious alteration of `manifest.json` or `project.db`. | **HIGH** | Mandatory SHA-256 integrity verification (`compute_sha256`); atomic file replacement prevents partial writes. |
| **2. Plugins (WASM)** | **Elevation of Privilege** | WASM plugin attempts host filesystem or process memory access. | **CRITICAL** | Wasmtime WASI sandbox traps unauthorized I/O; 10-tier permission boundary validator (`SIRA-6004`). |
| **3. AI Providers** | **Information Disclosure** | Cloud provider requests log plain-text user API keys. | **HIGH** | Zero plain-text API keys in configs; loaded from OS Keychain; automatic regex log redaction (`sira_diagnostics`). |
| **4. IPC Channels** | **Spoofing / Tampering** | Unauthorized process connects to SIRA Core gRPC socket. | **HIGH** | Unix Domain Sockets / Named Pipes restricted to local process owner permissions (`0600`). |
| **5. Supply Chain** | **Tampering** | Compromised third-party NPM or Cargo dependency. | **HIGH** | Pinned toolchains, SBOM generation, vulnerability scanning, cargo lockfile auditing. |
| **6. Credentials** | **Information Disclosure** | User API keys stored in plain text on disk. | **CRITICAL** | API keys stored in macOS Keychain or Windows Credential Manager; excluded from project bundles. |
| **7. Model Downloads** | **Tampering** | Man-in-the-middle attack injects corrupted model weights. | **HIGH** | Mandatory TLS 1.3 encryption and SHA-256 weight checksum verification (`SIRA-3008`). |
| **8. Workflow Marketplace (`.sfsw`)** | **Spoofing / Tampering** | Untrusted marketplace workflow contains malicious payload. | **HIGH** | Mandatory Ed25519 digital signature verification (`DigitalSignatureVerifier`) & trust level enforcement. |
| **9. Cloud Providers** | **Information Disclosure** | Sensitive script text transmitted over unencrypted HTTP. | **MEDIUM** | Enforce TLS 1.3 HTTPS endpoint validation on all cloud provider requests. |
| **10. Shared Memory** | **Tampering** | Unprivileged local process modifies frame buffer ring buffers. | **MEDIUM** | Operating system memory mapping access rights restricted strictly to child sub-engine processes. |
