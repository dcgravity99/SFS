# SECURE CODING STANDARD
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: MANDATORY DEVELOPER SPECIFICATION  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Rust Secure Coding Rules

1. **Prohibition of `unsafe`**: `unsafe` blocks are strictly forbidden unless accompanied by an explicit security audit comment signed off by the Chief Software Architect.
2. **Bounds Checking & Safe Conversions**: Prefer `try_into()` and checked arithmetic over raw casts (`as`).
3. **Panic Isolation**: All thread bounds and WASM host imports MUST use `std::panic::catch_unwind` to prevent process crashes.
4. **Error Handling**: Never discard errors using `.unwrap()` or `.expect()` in production code. Use `SiraResult<T>` and structured error mapping (`SIRA-1000` to `SIRA-7999`).
5. **Sensitive Memory Cleanup**: Structs handling API keys, tokens, or encryption keys MUST implement `zeroize::ZeroizeOnDrop`.

---

## 2. TypeScript Secure Coding Rules

1. **Strict Compiler Mode**: `tsconfig.json` MUST enforce `"strict": true` and `"noImplicitAny": true`.
2. **Forbidden Type Bypasses**: The `any` type and `@ts-ignore` directives are strictly prohibited.
3. **Input Validation**: All external API payloads and IPC inputs MUST be validated using Zod / JSON schema validators before processing.
4. **Context Isolation**: Webview contexts MUST restrict Content Security Policy (CSP) headers and disable raw Node.js integration.

---

## 3. C++20 Secure Coding Rules

1. **Modern RAII & Smart Pointers**: Use `std::unique_ptr` and `std::shared_ptr`. Raw pointers and `malloc`/`free` are strictly forbidden.
2. **Compiler Sanitizers**: All C++ native build targets MUST compile cleanly under AddressSanitizer (`-fsanitize=address`) and UndefinedBehaviorSanitizer (`-fsanitize=undefined`).
3. **Bounds Checking**: Mandatory usage of `std::span` and `std::array` with bounds checking instead of C-style array pointers.
