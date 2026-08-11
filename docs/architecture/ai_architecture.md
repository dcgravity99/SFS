# AI Core & Model Registry Architecture Specification
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED  
**Author**: AG (Chief Software Architect)  

---

## 1. SIRA AI Core Sub-Engine Suite

SIRA AI Core operates as an isolated execution runtime housing 11 sub-engines:

```
+-------------------------------------------------------------------------+
|                            SIRA AI CORE                                 |
| +------------------------+ +-------------------+ +--------------------+ |
| | 1. AI Orchestrator     | | 2. Workflow Engine| | 3. Prompt Engine   | |
| +------------------------+ +-------------------+ +--------------------+ |
| | 4. Model Manager       | | 5. Plugin Manager | | 6. Knowledge Engine| |
| +------------------------+ +-------------------+ +--------------------+ |
| | 7. Story Intelligence  | | 8. Character Intel| | 9. Scene Intel     | |
| +------------------------+ +-------------------+ +--------------------+ |
| | 10. Director Intel     | | 11. Render Intel  |                        |
| +------------------------+ +-------------------+                        |
+-------------------------------------------------------------------------+
```

---

## 2. Enterprise AI Model Registry Schema

Every supported AI model is tracked in a centralized, extensible JSON/Protobuf Model Registry:

```json
{
  "model_id": "sira-diffusers-sdxl-v1",
  "name": "Stable Diffusion XL 1.0 Base",
  "family": "StableDiffusion",
  "version": "1.0.0",
  "license": "CreativeML OpenRAIL-M",
  "provider": "Local (Diffusers)",
  "is_local": true,
  "supported_tasks": ["Image Generation", "Storyboard Synthesis"],
  "vram_requirement_mb": 8192,
  "ram_requirement_mb": 16384,
  "quantization_support": ["fp16", "int8", "int4"],
  "context_length": 77,
  "performance_rating": "4.8/5.0",
  "hardware_compatibility": ["CUDA", "MPS", "DirectML", "ROCm"],
  "download_source": "https://huggingface.co/stabilityai/stable-diffusion-xl-base-1.0",
  "checksum_sha256": "31e35cde5e003f1d0541e21...89a",
  "dependencies": ["python>=3.10", "torch>=2.1.0"],
  "status": "Installed",
  "benchmark_results": { "tokens_per_sec": 0, "seconds_per_image": 3.4 }
}
```

### Supported Model Categories:
1. Large Language Models (LLMs)
2. Vision Models (VLM)
3. Image Generation
4. Video Generation
5. Audio Generation
6. Speech Recognition (STT)
7. Text-to-Speech (TTS)
8. Music Generation
9. Video Enhancement (Upscalers/Frame Interpolation)
10. Image Enhancement
11. Embedding Models (RAG Vectorizers)

---

## 3. Local AI Package Manager Architecture

The Package Manager provides automated lifecycle control for local models:
- **Download & Verification**: Multi-threaded chunked downloader with SHA-256 integrity verification.
- **Quantization Controller**: Converts weights dynamically to GGUF, EXL2, or AWQ based on available GPU VRAM.
- **Dependency & Rollback Engine**: Resolves backend dependencies and maintains single-click version rollback.
- **Offline Importer**: Supports importing local `.safetensors`, `.gguf`, or `.bin` model packages without internet.

---

## 4. Workflow Graph Engine

Every creative workflow is represented as a Directed Acyclic Graph (DAG) of processing nodes:

```
[Voice Input] → [STT Node] → [Story Analysis Node] → [Scene Breakout Node]
                                                          │
                  ┌───────────────────────────────────────┴───────────────────────────────────────┐
                  ▼                                                                               ▼
     [Storyboard Generator Node]                                                     [Dialogue Audio TTS Node]
                  │                                                                               │
                  ▼                                                                               ▼
     [Video Generation Node]                                                         [Foley Sound Synthesis Node]
                  │                                                                               │
                  └───────────────────────────────────────┬───────────────────────────────────────┘
                                                          ▼
                                            [Timeline Video Assembler]
                                                          │
                                                          ▼
                                             [Final Render & Export]
```
