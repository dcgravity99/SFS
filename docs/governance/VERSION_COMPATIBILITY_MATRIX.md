# VERSION COMPATIBILITY MATRIX
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED VERSION COMPATIBILITY SPECIFICATION  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Project Format (.sfsp) Compatibility Matrix

| SFSP Format Version | Min Studio Version | Forward Compatibility Policy | Migration Strategy |
| :--- | :--- | :--- | :--- |
| **v1.0.0** (Current) | `v0.1.0-alpha` | 1.x series fully readable by future 1.y releases. | Automatic SQLite schema migration (`V1__init.sql`). |
| **v1.1.0** (Future) | `v0.2.0-alpha` | Reject opening in versions `< v0.2.0` with `SIRA-4002`. | Automated backup creation (`manifest.json.v1.bak`). |

---

## 2. Workflow Format (.sfsw) Compatibility Matrix

| SFSW Schema Version | Node Contract Version | Compatibility Guarantee |
| :--- | :--- | :--- |
| **v1.0.0** | `NodeContract v1.0.0` | 100% portable across macOS and Windows. |
| **v1.1.0** | `NodeContract v1.1.0` | Fall back to un-cached node re-execution if version mismatch occurs. |

---

## 3. Plugin SDK & Host API Matrix

| Plugin SDK Version | Target Studio Version | Host API Version | WASI Runtime |
| :--- | :--- | :--- | :--- |
| **v1.0.0** | `v0.1.0-alpha` | `HostApiModuleGroup v1` | Wasmtime `v18.0` |
| **v1.1.0** | `v0.2.0-alpha` | `HostApiModuleGroup v1` | Wasmtime `v18.0` |

---

## 4. HAL Backend & Hardware Capability Matrix

| HAL Backend | Minimum OS / SDK | Supported Compute Data Types | Fallback Policy |
| :--- | :--- | :--- | :--- |
| **Apple Metal / MPS** | macOS 13.0+ (Metal 3) | FP16, BF16, INT8, Unified Memory | Automatic CPU Fallback |
| **NVIDIA CUDA** | Windows 11 (CUDA 12.2+) | FP16, BF16, INT8, Tensor Cores | Fallback to DirectML or CPU |
| **AMD ROCm** | Windows 11 / Linux (ROCm 5.7+) | FP16, BF16, INT8 | Fallback to Vulkan or CPU |
| **DirectML** | Windows 11 (DirectX 12) | FP16, INT8 | Fallback to CPU |
| **CPU Engine** | Any (x86_64 / ARM64) | FP32, FP16 (AVX-512 / NEON) | Baseline Universal Target |

---

## 5. Public API & IPC Compatibility Matrix

| Component | Protocol / Binding | Version | Stability Level |
| :--- | :--- | :--- | :--- |
| **Core Types** | Rust & TypeScript | `0.1.0` | **STABLE** |
| **IPC Transport** | gRPC / Unix Domain Sockets | Proto3 (`sira_common.proto`) | **STABLE** |
| **IPC Frame Buffer** | Shared Memory Ring Buffer | Native IPC v1 | **STABLE** |
| **Database Schema** | SQLite 3 WAL Mode | `v1` (`assets`, `asset_relationships`) | **STABLE** |
