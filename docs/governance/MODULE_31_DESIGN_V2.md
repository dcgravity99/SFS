# MODULE 31 DESIGN SPECIFICATION v2.0: STUDIO UI FRAMEWORK & DESKTOP SHELL INTEGRATION
**Siragugal Film Studio**  
**Document Version**: 2.0.0  
**Status**: REVISED DESIGN SPECIFICATION FOR REVIEW  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Executive Summary

Module 31 establishes the frontend presentation architecture (`apps/studio-ui/`) and desktop shell IPC integration (`packages/sira-studio-app`) for **Siragugal Film Studio**. Version 2.0 refines the design specification with formal versioned IPC envelopes, strict Content Security Policy (CSP), Zustand immutable state slice design, WCAG 2.2 AA accessibility, virtualized rendering performance budgets, and comprehensive testing strategies without adding unapproved application code.

---

## 2. Versioned IPC Contract Architecture

All IPC communications between React 19 and Rust backend sub-engines use strongly typed, versioned envelopes:

```typescript
// IPC Envelope Contract (v1.0.0)
export interface IpcRequestEnvelope<T = unknown> {
  request_id: string;      // UUIDv7
  correlation_id: string;  // UUIDv7
  schema_version: string;  // "1.0.0"
  timestamp_ms: number;
  command: string;         // Whitelisted command identifier
  payload: T;
}

export interface IpcResponseEnvelope<T = unknown> {
  request_id: string;
  correlation_id: string;
  schema_version: string;
  timestamp_ms: number;
  success: boolean;
  data?: T;
  error?: StandardizedIpcError;
}

export interface StandardizedIpcError {
  code: string;           // e.g. "SIRA-6004", "SIRA-4002"
  error_name: string;
  category: string;
  message: string;
  is_recoverable: boolean;
}

export interface ExperienceEventV1 {
  event_id: string;        // UUIDv7
  event_version: string;   // "1.0.0"
  timestamp_ms: number;
  correlation_id: string;
  source_module: string;
  severity: "Info" | "Success" | "Warning" | "Error" | "Critical";
  event_category: string;
  payload_json: string;
}
```

---

## 3. Security Architecture & OWASP ASVS Level 2 Mapping

- **Content Security Policy (CSP)**: `default-src 'self'`; zero `unsafe-inline` or `unsafe-eval`; script injection disabled.
- **Zod Payload Validation**: Every incoming and outgoing IPC message validated against Zod runtime schemas before dispatch.
- **IPC Command Allowlist**: Whitelist enforcement rejecting unauthorized IPC invocation strings.
- **Zero Direct Filesystem Access**: React UI has zero direct disk access; all file references operate through `asset_db` and `sira_engine_asset` `AssetId` handles.
- **OWASP ASVS Mapping**: Complies with V5 (Validation, Sanitization & Encoding), V8 (Data Protection), V13 (API Security).

---

## 4. Frontend State Architecture

Organized into 6 isolated Zustand state slices with immutable Immer updates:
1. **App Store**: Application lifecycle state, active project session, health status.
2. **Project Store**: Current project metadata, asset catalog, screenplay AST references.
3. **Workspace Store**: Active studio workspace mode (Story, Character, Actor, Scene, Director, Cinematography, Audio, Timeline, Render).
4. **Timeline Store**: Playhead timecode (`SiraTimecode`), active track locks, clip selection.
5. **Preferences Store**: User UI settings, keyboard shortcut bindings, theme configuration.
6. **Layout Store**: Multi-dock panel coordinates, split pane ratios, panel visibility.

---

## 5. UI Architecture & Design System

- **Design Tokens**: CSS variables defining primary accents (`hsl(220, 90%, 56%)`), surface dark modes (`#0d0f12`, `#14181f`), glassmorphism backdrop blurs (`backdrop-filter: blur(16px)`), and border glows.
- **Typography System**: Inter for UI controls; Outfit for studio headers; Fira Code for script/prompts.
- **Responsive Dock Layout**: Resizable split panels supporting multi-monitor display configurations.

---

## 6. Accessibility & WCAG 2.2 AA Compliance

- **Keyboard Navigation**: Full tab index order, visible focus rings (`ring-2 ring-primary`), keyboard shortcut overlays (`Ctrl+Shift+P`).
- **Screen Reader Support**: ARIA live regions (`aria-live="polite"`) for toast notifications and progress updates.
- **Accessibility Flags**: Support High Contrast Mode toggle and Reduced Motion CSS (`prefers-reduced-motion: reduce`).

---

## 7. Performance Targets & Budgets

- **Initial Render Latency**: `< 50 ms`
- **IPC Round-Trip Overhead**: `< 1.0 ms`
- **Animation Target**: Continuous 60 FPS
- **Virtual Scrolling**: Mandatory virtualized list rendering (TanStack Virtual) for timelines, asset catalogs, and script beat sheets.
- **Code Splitting**: Dynamic React `lazy()` chunk loading per workspace mode.

---

## 8. Error Handling & Recovery Workflows

- **Global Error Boundary**: Catch unhandled UI exceptions; display user-friendly diagnostic dialog with correlation IDs.
- **IPC Timeout Policy**: 10-second default timeout on non-blocking requests; automatic retry with exponential backoff on transient errors.

---

## 9. Testing Strategy

- **Vitest & React Testing Library**: Unit test UI state stores and isolated components.
- **Playwright**: End-to-end integration tests validating Tauri IPC bridge workflows.
- **Axe-core Accessibility Audits**: Automated WCAG 2.2 AA accessibility regression testing.
