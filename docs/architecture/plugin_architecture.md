# Plugin SDK & Sandboxing Architecture Specification
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED  
**Author**: AG (Chief Software Architect)  

---

## 1. Overview & Plugin Categories

Siragugal Film Studio provides a modular, sandboxed Plugin SDK across 10 distinct extension categories:

```
                          +-------------------------+
                          |   PLUGIN SDK ARCHITECTURE|
                          +-------------------------+
                                       │
     ┌──────────┬──────────┬───────────┼───────────┬──────────┬──────────┐
     ▼          ▼          ▼           ▼           ▼          ▼          ▼
[AI Provider][Workflow] [Timeline] [Generator] [Enhancer] [Exporter] [Node Plugins]
     │          │          │           │           │          │          │
     └──────────┴──────────┴───────────┼───────────┴──────────┴──────────┘
                                       ▼
                       [Importer / Template / Theme Plugins]
```

---

## 2. Sandboxing & Isolation Architecture

All third-party plugins execute within strict memory-isolated sandboxes to prevent malicious code execution, crashes, or unauthorized file/network access:

- **WebAssembly (WASM) Sandbox**: For lightweight data transformations, custom node logic, script parsers, and custom UI panels.
- **Process-Isolated RPC Sandbox**: For heavy native AI backends running in dedicated child processes communicating via gRPC over IPC with restricted process privileges.

---

## 3. Plugin Permission & Security Model

Plugins must declare explicit manifest permissions (`plugin.json`):

```json
{
  "plugin_id": "com.community.elevenlabs-v2",
  "name": "ElevenLabs Voice Provider",
  "version": "2.0.0",
  "category": "AI Provider",
  "sandbox": "WASM",
  "permissions": {
    "network": ["api.elevenlabs.io"],
    "filesystem": "none",
    "gpu_access": false,
    "keychain_read": ["ELEVENLABS_API_KEY"]
  }
}
```

- **Cryptographic Code Signing**: Mandatory digital signatures for verified community plugins.
- **Permission Dialog**: Users are prompted before granting network or filesystem access to third-party plugins.
