# MODULE 11 DESIGN SPECIFICATION: AI PROVIDER INTERFACE
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 11 establishes the plugin-based AI Provider Interface (`sira-ai-provider`) for **Siragugal Film Studio**. It implements capability-driven dispatching, Provider Manifests, universal request/response contracts (`AIRequest`, `AIResponse`, `AIStreamChunk`), separate Provider and Model Registries, policy-driven routing, standardized streaming interfaces, security isolation, offline-first fallback chains, provider benchmarking frameworks, and 100% decoupling from SIRA Core without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Capability-Driven Dispatching**: Runtime requests `AICapability` (TextGen, VideoGen, SpeechToText) without vendor-specific operations.
2. **Provider Manifest**: Describes provider metadata: `provider_id`, `vendor_name`, `supported_capabilities`, `models`, `supports_streaming`, `auth_type`, `license`, `is_offline_capable`.
3. **Universal Request/Response Contracts**: Isolates vendor payloads using `AIRequest`, `AIResponse`, `AIStreamChunk`, `AIUsage`, `AIError`, and `AIModelInfo`.
4. **Decoupled Registries**: `ProviderRegistry` manages provider plugins; `ModelRegistry` tracks independently versioned models and weights (GGUF, Safetensors, ONNX) with SHA-256 checksum verification (`SIRA-3008`).
5. **Policy-Driven `ProviderRouter`**: Evaluates 8 routing dimensions: Latency (ms), USD Cost, VRAM required vs available, Context Window limits, Queue Depth, Provider Health, User Preference, and Offline Mode flag.
6. **Standardized Streaming Interface**: Handles streaming chunks (`AIStreamChunk`) for text tokens, audio PCM frames, image tile progress, video frame progress, and tool calls.
7. **Security & Sandbox Isolation**: Zero plain-text API keys in configs; keys loaded from OS keychains; mandatory TLS 1.3 validation; SHA-256 weight verification.
8. **Offline-First Fallback Chain**: Default preference order: `Local Model` (llama.cpp/Diffusers) → `Enterprise Local Server` → `Cloud Provider` (OpenAI/Anthropic/Runway) → `Fallback Mock`.
9. **Provider Benchmark Framework**: Measures TTFT (Time to First Token), throughput (tokens/sec or frames/sec), VRAM footprint, and USD cost per job.

---

## 3. Provider Manifest & Universal Contracts Schema

```json
{
  "provider_id": "provider-local-llm",
  "vendor_name": "Siragugal Llama.cpp Local Engine",
  "supported_capabilities": ["TextGeneration"],
  "models": [
    {
      "model_id": "llama-3-8b-instruct.gguf",
      "display_name": "Llama 3 8B Instruct (GGUF Q4_K_M)",
      "context_window_tokens": 8192,
      "vram_required_mb": 5600,
      "checksum_sha256": "4bf92f3577b34da6a3ce929d0e0e4736"
    }
  ],
  "supports_streaming": true,
  "auth_type": "None",
  "license": "Apache-2.0",
  "is_offline_capable": true
}
```

---

## 4. File Blueprint

Module 11 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-ai-provider/           # Rust AI Provider abstraction crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root
            ├── manifest.rs         # ProviderManifest & AIModelInfo structs
            ├── contracts.rs        # AIRequest, AIResponse, AIStreamChunk, AIUsage
            ├── provider_trait.rs   # AiProvider async trait
            ├── provider_registry.rs# ProviderRegistry plugin manager
            ├── model_registry.rs   # ModelRegistry & SHA-256 checksum verifier
            ├── router.rs           # Policy-driven ProviderRouter & offline-first fallback chain
            ├── security.rs         # OS keychain API key isolation & TLS validation
            ├── benchmark.rs        # ProviderBenchmark framework (TTFT, throughput, VRAM)
            └── mock_provider.rs    # Verification MockProvider for unit testing
```

---

## 5. Acceptance Criteria

Module 11 is accepted when:
1. `packages/sira-ai-provider` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. New AI providers integrate 100% by implementing the `AiProvider` trait alone, with zero changes to SIRA Core, HAL, Workflow Engine, or UI.
3. Policy-driven routing and offline-first fallback chains pass 100% of integration tests.
4. SHA-256 model weight checksum verification detects corrupted weights emitting `SIRA-3008`.
5. Zero application or creative feature code is present.
