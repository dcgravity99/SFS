# MODULE 44 DESIGN SPECIFICATION: COLLABORATION & TEAM REVIEW WORKSPACE UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 44 establishes the **Collaboration & Team Review Workspace UI** (`apps/studio-ui/src/features/collaboration/`) for **Siragugal Film Studio**. It implements multi-user production collaboration dashboards, team artist role assignment management (Director, Cinematographer, Audio Engineer, Animator), timecode-bound shot review annotation threads, shot approval workflows (`Approved`, `Revisions Requested`, `Pending Review`), split-screen version comparison viewers, and live IPC integration with collaboration sub-services following the Tamil-first (`ta-IN`) globalization architecture.

---

## 2. Module Responsibilities & Core Features

1. **Collaboration Master Workspace**: Multi-user team production hub coordinating scene reviews, artist task assignments, and approval state tracking.
2. **Team Members & Role Panel**: Artist roster panel displaying active team members (`ArtistId`), roles, online status, and current task assignments.
3. **Timecode Review Thread & Annotations Panel**: Timecode-synchronized review comment thread displaying feedback, frame marker references (`Timecode`), and localized text comments.
4. **Shot Approval Workflow Inspector**: Official review status manager for approving shots, requesting visual revisions, or flagging issues.
5. **Version Comparison Viewer**: Side-by-side split visualizer comparing current shot iteration against previous review checkpoints (`v1` vs `v2`).
6. **Globalization & Localization Engine**: Tamil-first i18n string externalization (`ta-IN` primary, `en-US` secondary) for all collaboration controls.
7. **Sub-Engine IPC Integration**: Send review feedback to backend via `StudioIpcService.executeEngineCommand('collaboration_create_review', ...)` and `collaboration_update_approval`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 08 (`sira_core`), Module 05 (`sira_asset_db`), Module 24 (`sira_engine_timeline`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 43 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_43_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 44 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/collaboration/types.ts)
export interface TeamMemberView {
  artist_id: string; // Machine-readable UUIDv7
  display_name: string;
  role: 'Director' | 'Cinematographer' | 'AudioEngineer' | 'Animator' | 'Editor';
  avatar_asset_id?: string;
  is_online: boolean;
}

export interface ReviewCommentView {
  comment_id: string;
  artist_id: string;
  timecode_frame: number;
  content: LocalizedTextMap;
  created_at: string;
}

export interface ShotApprovalStateView {
  shot_id: string;
  approval_status: 'Approved' | 'RevisionsRequested' | 'PendingReview';
  approved_by_artist_id?: string;
}

// React Feature Components
export declare const CollaborationWorkspace: React.FC;
export declare const TeamMembersPanel: React.FC;
export declare const ReviewThreadPanel: React.FC;
export declare const ApprovalWorkflowPanel: React.FC;
export declare const VersionComparisonViewer: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 44 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── collaboration.json
            │       └── en-US/
            │           └── collaboration.json
            └── features/
                └── collaboration/  # Collaboration & Team Review feature package
                    ├── types.ts    # Team & review UI models
                    ├── CollaborationWorkspace.tsx   # Master collaboration workspace
                    ├── TeamMembersPanel.tsx         # Artist roster & roles panel
                    ├── ReviewThreadPanel.tsx        # Timecode comment thread
                    ├── ApprovalWorkflowPanel.tsx    # Shot approval status inspector
                    └── VersionComparisonViewer.tsx  # Side-by-side version viewer
```

---

## 6. Testing & Validation Strategy

1. **Review Thread Post Test**: Submit review note; verify IPC payload emits `collaboration_create_review` command cleanly.
2. **Shot Approval State Change Test**: Change approval status to `Approved`; verify badge updates color.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`குழுப் பணித் துறை (Collaboration Studio)`).

---

## 7. Acceptance Criteria

Module 44 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Collaboration Studio components render team rosters, review threads, and approval panels cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all collaboration controls.
4. Zero unapproved external network chat/synthesis logic is present in React UI.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 44: Collaboration & Team Review Workspace UI**.
> 2. Upon your explicit approval, I will execute Module 44 implementation (`apps/studio-ui/src/features/collaboration/`).
