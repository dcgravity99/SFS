# DEPENDENCY SECURITY & SUPPLY CHAIN POLICY
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: MANDATORY SUPPLY CHAIN SPECIFICATION  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Executive Summary

This document establishes the supply chain security and dependency governance policy for **Siragugal Film Studio** adhering to **SLSA Level 3** (Supply-Chain Levels for Software Artifacts) and **NIST SSDF SP 800-218**.

---

## 2. Dependency Verification Rules

1. **License Compliance**: All third-party dependencies MUST be licensed under Apache-2.0, MIT, BSD-2-Clause, BSD-3-Clause, or MPL-2.0. GPL/AGPL copyleft licenses are strictly forbidden.
2. **Lockfile Pinning**: All build lockfiles (`Cargo.lock`, `pnpm-lock.yaml`) MUST be committed to version control and pinned to exact versions.
3. **Automated Vulnerability Scanning**: CI pipelines MUST execute `cargo audit`, `cargo deny check`, `npm audit`, and `trivy` on every commit. Build pipelines WILL FAIL on any High or Critical vulnerability.
4. **Software Bill of Materials (SBOM)**: Every release build MUST automatically generate an SPDX 2.3 JSON Software Bill of Materials (`spdx.json`).
