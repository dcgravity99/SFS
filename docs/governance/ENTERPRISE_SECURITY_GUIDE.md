# ENTERPRISE SECURITY & COMPLIANCE GOVERNANCE GUIDE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED & PUBLISHED  
**Author**: AG (Chief Software Architect)  

---

## 1. Overview

This document defines the Role-Based Access Control (RBAC) authorization matrix, security policy rules, cryptographic key management procedures, and compliance standards for **Siragugal Film Studio**.

---

## 2. Role-Based Access Control (RBAC) Matrix

| Role | Story / Screenplay | Shot Approval | Render Submit | Project Settings | System Admin |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Administrator** | ✅ Full | ✅ Full | ✅ Full | ✅ Full | ✅ Full |
| **Director** | ✅ Full | ✅ Full | ✅ Full | ✅ Full | ❌ Restricted |
| **Producer** | 👁️ Read | ✅ Full | ✅ Full | ✅ Full | ❌ Restricted |
| **Cinematographer** | 👁️ Read | 📝 Annotate | ✅ High Priority | ❌ Read Only | ❌ Restricted |
| **Animator** | 👁️ Read | 📝 Annotate | ✅ Normal Priority | ❌ Read Only | ❌ Restricted |
| **Audio Engineer** | 👁️ Read | 📝 Annotate | ✅ Normal Priority | ❌ Read Only | ❌ Restricted |
| **Editor** | 👁️ Read | 📝 Annotate | ✅ Normal Priority | ❌ Read Only | ❌ Restricted |
| **Viewer** | 👁️ Read | ❌ Denied | ❌ Denied | ❌ Read Only | ❌ Restricted |

---

## 3. Cryptographic Key Lifecycle

- **Key Generation**: AES-256-GCM keys created via hardware security module (HSM) bindings.
- **Key Rotation**: Automated rotation schedule every 90 days.
- **Zero Raw Secret Exposure**: Keys exposed via opaque handle handles only (`KeyId`).
