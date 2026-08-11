# Development, Security & Quality Control Workflow
**Siragugal Film Studio**  
**Document Version**: 1.2.0  
**Status**: APPROVED & FROZEN  
**Author**: AG (Chief Software Architect)  

---

## 1. Governance & Quality Assurance Pipeline

Every contribution to Siragugal Film Studio must strictly adhere to the 11-step lifecycle outlined in Constitution v1.2.0:

1. **Analyze**: Define business value, creative impact, and non-functional requirements.
2. **Research**: Survey existing codebase utilities and open-source benchmarks.
3. **Design**: Draft ADR in `docs/adr/` if architectural boundaries change.
4. **Review**: Architectural approval by Chief Software Architect (AG).
5. **Optimize**: Performance profiling (VRAM, memory, CPU/GPU footprint).
6. **Validate**: Security review & offline-first verification.
7. **Document**: Update technical specifications under `docs/`.
8. **Implement**: Code implementation matching repository standards.
9. **Test**: 80%+ unit coverage, integration tests, cross-platform validation (macOS & Windows).
10. **Review Again**: Peer code review and final architectural sign-off.

---

## 2. Security, Observability & Architecture Freeze Governance

- **Pre-Implementation Freeze Mandate**: The technical architecture is frozen at v1.2.0. No architectural modifications are permitted without a formal Architecture Decision Record (ADR).
- **Zero-Trust Key Management**: Cloud API credentials (OpenAI, Anthropic, Gemini, ElevenLabs, Runway) must be stored in platform-native secure keychains (macOS Keychain / Windows Credential Manager).
- **Plugin Signing & Sandboxing**: All plugins must be digitally signed and run inside WASM or process RPC sandboxes with manifest permission control.
- **Observability**: Structured JSON logging, real-time GPU/VRAM telemetry, and opt-in diagnostics.

---

## 3. Versioning & Branching Strategy

- **Semantic Versioning 2.0.0**: `MAJOR.MINOR.PATCH`
- **Git Branching Policy**: `main` (Production), `develop` (Integration), `feature/*` (Isolated ADR features), `hotfix/*` (Emergency patches).
