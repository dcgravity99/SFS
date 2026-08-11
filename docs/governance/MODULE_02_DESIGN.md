# MODULE 02 DESIGN SPECIFICATION: BUILD SYSTEM & TOOLCHAIN
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 02 establishes the multi-language build system, Protobuf code compilation pipeline, native C++/Rust cross-compilation toolchain, multiple build profiles, code generation policies, installer signing architecture, and release artifact bundler for **Siragugal Film Studio**.

---

## 2. Module Responsibilities & Toolchain Features

1. **Protobuf Schema Versioning & Pipeline**: Automate compilation of `.proto` (proto3) schemas into TypeScript, Rust, and C++ stubs. Enforce backward-compatible field numbering and zero field tag reuse.
2. **Build Profile Management**: Provide 6 distinct build profiles: `Debug`, `Development`, `Release`, `Benchmark`, `CI`, and `Sanitizer` (AddressSanitizer & ThreadSanitizer).
3. **Native FFI Boundary Rules**: Standardize C ABI (`extern "C"`), memory ownership (caller-allocates or explicit `free` handles), thread safety, panic catching (`catch_unwind`), and error code returns (`SIRA-*`).
4. **Build Caching Strategy**: Implement caching layers across Cargo (`target/`), pnpm (`.pnpm-store`), C++ (`sccache`), and CI (GitHub Actions cache).
5. **Compiler Warning Policy**: Enforce zero-warning objective (`#[deny(warnings)]` in Rust, `/WX` or `-Werror` in C++, ESLint errors only).
6. **Installer Signing Architecture**: Support Apple Notarization (`xcrun notarytool`) on macOS and Microsoft Authenticode (`signtool.exe`) on Windows.
7. **Cross-Compilation Matrix**: Support building macOS (`aarch64-apple-darwin`, `x86_64-apple-darwin`) and Windows (`x86_64-pc-windows-msvc`).
8. **Reproducible Build Enforcement**: Lock dependencies (`pnpm-lock.yaml`, `Cargo.lock`) and enforce deterministic compilation outputs.

---

## 3. Build Profile Matrix

| Build Profile | Optimization Level | Debug Symbols | LTO | Sanitizers | Target Use Case |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Debug** | `opt-level=0` | Full (`gdb/lldb`) | Off | None | Local iteration & interactive debugging |
| **Development** | `opt-level=1` | Limited | Off | None | Fast incremental compilation for developers |
| **Release** | `opt-level=3` | Stripped | Thin LTO | None | Official production desktop releases |
| **Benchmark** | `opt-level=3` | Full | Fat LTO | None | Performance budget & VRAM profiling |
| **CI** | `opt-level=2` | Line tables | Off | None | Fast, deterministic integration testing |
| **Sanitizer** | `opt-level=1` | Full | Off | ASan / TSan | Memory leak & data-race verification |

---

## 4. Native FFI Boundary Rules

All C++/Rust FFI boundaries must adhere to the following rules:
1. **C ABI Stability**: All exported functions must use `extern "C"` linkage.
2. **Memory Ownership**: Explicit allocation and free pairs (`sira_buffer_alloc` and `sira_buffer_free`). Pointer ownership must never cross language boundaries implicitly.
3. **Panic Safety**: Rust FFI callbacks must wrap execution in `std::panic::catch_unwind`. Unhandled panics crossing FFI boundaries are forbidden.
4. **Error Propagation**: Return numeric error codes (`SIRA-1000` to `SIRA-7999`) with out-parameter error message buffers.

---

## 5. Installer Signing & Notarization Architecture

```
+-------------------------------------------------------------------------+
|                    RELEASE COMPILATION (LTO = FAT)                      |
+-------------------------------------------------------------------------+
                                    │
                                    ▼
+-------------------------------------------------------------------------+
|                      BINARY SIGNING & STAMPING                          |
|  - macOS: codesign --options runtime --timestamp (Developer ID App)     |
|  - Windows: signtool.exe sign /tr timestamp_server (Authenticode)       |
+-------------------------------------------------------------------------+
                                    │
                                    ▼
+-------------------------------------------------------------------------+
|                     INSTALLER PACKAGING & NOTARIZATION                  |
|  - macOS: xcrun notarytool submit MyInstaller.dmg --wait               |
|  - Windows: Wix Toolchain -> MyInstaller.msi                            |
+-------------------------------------------------------------------------+
```

---

## 6. Public Interfaces & Build CLI Commands

```bash
# Module 02 CLI Commands
pnpm build:proto         # Compile Protobuf schemas to TS/Rust stubs
pnpm build:native        # Compile native C++/Rust FFI bindings
pnpm build:release       # Execute Release profile build with LTO
pnpm build:sanitizer     # Run build with ASan/TSan memory checks
pnpm package:installer   # Package signed macOS DMG / Windows MSI installer
```

---

## 7. Internal File Blueprint

Module 02 implements the following toolchain components:

```
D:\SiragugalFilmStudio\
├── docs/
│   └── schemas/
│       └── sira_common.proto       # Core Protobuf schema definition (proto3)
└── tools/
    └── build/
        ├── compile_proto.js        # Protobuf compiler script (TS & Rust output)
        ├── build_native.js         # Native C++/Rust FFI builder & FFI wrapper
        ├── package_app.js          # Installer packaging & signing coordinator
        └── build_config.json       # Profile parameters, compiler flags & cache rules
```

---

## 8. Acceptance Criteria

Module 02 is accepted when:
1. `tools/build/compile_proto.js` compiles `sira_common.proto` into valid TypeScript and Rust stubs without errors.
2. `tools/build/build_native.js` successfully configures native C++/Rust FFI build wrappers across profiles.
3. `tools/build/package_app.js` runs installer dry-runs cleanly.
4. Compiler warning checks enforce zero compiler warnings across TS, Rust, and C++.
5. A documented clean build succeeds cleanly on a supported machine.
6. Zero application or creative feature code is present.
