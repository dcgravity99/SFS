# PULL REQUEST SECURITY REVIEW CHECKLIST
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: MANDATORY CODE REVIEW SPECIFICATION  
**Author**: AG (Permanent Chief Software Architect)  

---

## Mandatory PR Security Verification Items

Every Pull Request MUST complete and verify the following 10 security items before merging:

- [ ] **1. Input Validation**: All public functions canonicalize file paths and validate external parameters.
- [ ] **2. Zero Secret Leaks**: Code passes secret scanning (`trufflehog` / `gitleaks`); zero API keys or credentials in code/logs.
- [ ] **3. Safe Error Handling**: Fallible operations return `SiraResult<T>`; zero `.unwrap()` calls in production code.
- [ ] **4. Memory Safety**: Zero un-audited `unsafe` Rust blocks; C++ code builds cleanly under AddressSanitizer.
- [ ] **5. Least Privilege Permissions**: New host APIs enforce explicit 10-tier permission boundary checks (`SIRA-6004`).
- [ ] **6. SQL Parameterization**: Database queries use bound parameters; zero string concatenation SQL.
- [ ] **7. Log Sanitization**: New log statements verify sensitive data redaction (`sk-...` stripped).
- [ ] **8. Dependency Verification**: `Cargo.lock` and `pnpm-lock.yaml` pass `cargo audit` and `npm audit` with 0 High/Critical findings.
- [ ] **9. Cryptography Audit**: Uses only AES-256-GCM, Ed25519, SHA-256, or TLS 1.3.
- [ ] **10. Automated Tests**: Security negative tests and boundary fuzz tests added for new features.
