# MODULE 45 DESIGN SPECIFICATION: STUDIO SETTINGS & SYSTEM CONFIGURATION UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 45 establishes the **Studio Settings & System Configuration UI** (`apps/studio-ui/src/features/settings/`) for **Siragugal Film Studio**. As the final module in Phase 3 Presentation Infrastructure, it implements global application system settings, Tamil/English language preference toggles (`ta-IN` / `en-US`), dark/light theme options, GPU hardware acceleration preferences, auto-save interval configurations, security audit logging inspectors, and live IPC integration with desktop shell settings services following the Tamil-first (`ta-IN`) globalization architecture.

---

## 2. Module Responsibilities & Core Features

1. **Studio Settings Workspace**: Master configuration hub for application preferences, hardware acceleration, and enterprise security policies.
2. **Globalization & Locale Settings Panel**: Interactive language toggle switching primary application UI between Tamil (`ta-IN`) and English (`en-US`) dynamically.
3. **Hardware Acceleration & GPU Inspector**: GPU driver detection, CUDA / Vulkan backend selectors, and VRAM memory limit configuration sliders.
4. **Studio Preferences & Auto-Save Controls**: Project auto-save interval settings (`1 min`, `5 mins`, `15 mins`), undo history depth (`50` to `500` steps), and UI density controls.
5. **Enterprise Security & Audit Log Panel**: Security audit trail viewer displaying compliance checks (OWASP ASVS L2, NIST SSDF), active IPC version envelopes, and system health certificates.
6. **Sub-Engine IPC Integration**: Save studio configuration to desktop shell via `StudioIpcService.executeEngineCommand('settings_update_config', ...)` and `settings_get_system_info`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 30 (`sira_studio_app`), Module 08 (`sira_core`), Module 01 (`sira_types`), React 19, Zustand stores (`usePreferencesStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 44 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_44_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 45 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/settings/types.ts)
export interface StudioConfigView {
  primary_locale: 'ta-IN' | 'en-US';
  theme_mode: 'Dark' | 'Light' | 'System';
  auto_save_interval_mins: number;
  undo_history_depth: number;
  gpu_acceleration_enabled: boolean;
  vram_limit_mb: number;
}

export interface SecurityAuditEntryView {
  audit_id: string; // Machine-readable UUIDv7
  standard_name: string; // e.g. "OWASP ASVS L2"
  status: 'Compliant' | 'Warning' | 'NonCompliant';
  verified_at: string;
}

// React Feature Components
export declare const SettingsWorkspace: React.FC;
export declare const SystemPreferencesPanel: React.FC;
export declare const HardwareAccelerationPanel: React.FC;
export declare const LocaleThemeSelector: React.FC;
export declare const SecurityAuditPanel: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 45 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── settings.json
            │       └── en-US/
            │           └── settings.json
            └── features/
                └── settings/       # Studio Settings feature package
                    ├── types.ts    # Settings & configuration UI models
                    ├── SettingsWorkspace.tsx         # Master settings workspace
                    ├── SystemPreferencesPanel.tsx    # Auto-save & undo depth controls
                    ├── HardwareAccelerationPanel.tsx # GPU & VRAM limit settings
                    ├── LocaleThemeSelector.tsx       # Tamil/English & theme toggle
                    └── SecurityAuditPanel.tsx        # OWASP audit & IPC health viewer
```

---

## 6. Testing & Validation Strategy

1. **Locale Toggle Test**: Switch primary language to `en-US`; verify UI labels re-render in English dynamically.
2. **GPU Settings Update IPC Test**: Change VRAM limit slider; verify IPC payload emits `settings_update_config` command cleanly.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`அமைப்புகள் (Studio Settings)`).

---

## 7. Acceptance Criteria

Module 45 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Studio Settings components render preference controls, locale selectors, and security panels cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all setting controls.
4. Phase 3 Master Presentation Infrastructure (Modules 31–45) is 100% complete.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 45: Studio Settings & System Configuration UI**.
> 2. Upon your explicit approval, I will execute Module 45 implementation (`apps/studio-ui/src/features/settings/`).
