import React from 'react';
import { useWorkspaceStore } from '../../stores/workspace.store';
import { Card } from '../common/Card';
import { ScreenplayEditor } from '../../features/story/ScreenplayEditor';
import { BeatSheetBoard } from '../../features/story/BeatSheetBoard';
import { SceneBreakdownPanel } from '../../features/story/SceneBreakdownPanel';
import { CharacterGallery } from '../../features/character/CharacterGallery';
import { LoraBindingPanel } from '../../features/character/LoraBindingPanel';
import { ActorVoiceSelector } from '../../features/actor/ActorVoiceSelector';
import { VisemeTimelineVisualizer } from '../../features/actor/VisemeTimelineVisualizer';
import { EmotionalTonePanel } from '../../features/actor/EmotionalTonePanel';
import { SceneGraphInspector } from '../../features/scene/SceneGraphInspector';
import { TransformInspector } from '../../features/scene/TransformInspector';
import { PropRegistryPicker } from '../../features/scene/PropRegistryPicker';
import { DirectorWorkspace } from '../../features/director/DirectorWorkspace';
import { ShotListPanel } from '../../features/director/ShotListPanel';
import { DirectorNotesPanel } from '../../features/director/DirectorNotesPanel';
import { CameraBlockingView } from '../../features/director/CameraBlockingView';
import { CinematographyWorkspace } from '../../features/cinematography/CinematographyWorkspace';
import { LensProfilePanel } from '../../features/cinematography/LensProfilePanel';
import { CameraSettingsPanel } from '../../features/cinematography/CameraSettingsPanel';
import { LightingControlPanel } from '../../features/cinematography/LightingControlPanel';
import { DepthOfFieldVisualizer } from '../../features/cinematography/DepthOfFieldVisualizer';
import { AudioWorkspace } from '../../features/audio/AudioWorkspace';
import { DialogueTrackPanel } from '../../features/audio/DialogueTrackPanel';
import { WaveformViewer } from '../../features/audio/WaveformViewer';
import { SoundLibraryPanel } from '../../features/audio/SoundLibraryPanel';
import { AudioMixerPanel } from '../../features/audio/AudioMixerPanel';
import { TimelineWorkspace } from '../../features/timeline/TimelineWorkspace';
import { TimecodeRuler } from '../../features/timeline/TimecodeRuler';
import { MultiTrackCanvas } from '../../features/timeline/MultiTrackCanvas';
import { TrimmingToolsPanel } from '../../features/timeline/TrimmingToolsPanel';
import { PromptBuilderWorkspace } from '../../features/prompts/PromptBuilderWorkspace';
import { PositivePromptEditor } from '../../features/prompts/PositivePromptEditor';
import { NegativePromptEditor } from '../../features/prompts/NegativePromptEditor';
import { GenerationParamsPanel } from '../../features/prompts/GenerationParamsPanel';
import { AssetWorkspace } from '../../features/assets/AssetWorkspace';
import { MediaAssetGallery } from '../../features/assets/MediaAssetGallery';
import { AssetMetadataInspector } from '../../features/assets/AssetMetadataInspector';
import { StorageQuotaPanel } from '../../features/assets/StorageQuotaPanel';
import { ProjectWorkspace } from '../../features/project/ProjectWorkspace';
import { ProjectDashboard } from '../../features/project/ProjectDashboard';
import { ProjectMetadataPanel } from '../../features/project/ProjectMetadataPanel';
import { ProductionTracker } from '../../features/project/ProductionTracker';
import { VersionHistoryPanel } from '../../features/project/VersionHistoryPanel';
import { RenderWorkspace } from '../../features/render/RenderWorkspace';
import { RenderQueuePanel } from '../../features/render/RenderQueuePanel';
import { RenderProgressMonitor } from '../../features/render/RenderProgressMonitor';
import { RenderResourcePanel } from '../../features/render/RenderResourcePanel';
import { RenderRecoveryPanel } from '../../features/render/RenderRecoveryPanel';
import { CollaborationWorkspace } from '../../features/collaboration/CollaborationWorkspace';
import { TeamMembersPanel } from '../../features/collaboration/TeamMembersPanel';
import { ReviewThreadPanel } from '../../features/collaboration/ReviewThreadPanel';
import { ApprovalWorkflowPanel } from '../../features/collaboration/ApprovalWorkflowPanel';
import { VersionComparisonViewer } from '../../features/collaboration/VersionComparisonViewer';
import { SettingsWorkspace } from '../../features/settings/SettingsWorkspace';
import { SystemPreferencesPanel } from '../../features/settings/SystemPreferencesPanel';
import { HardwareAccelerationPanel } from '../../features/settings/HardwareAccelerationPanel';
import { LocaleThemeSelector } from '../../features/settings/LocaleThemeSelector';
import { SecurityAuditPanel } from '../../features/settings/SecurityAuditPanel';

export const WorkspacePanel: React.FC = () => {
  const { activeMode } = useWorkspaceStore();

  return (
    <main className="flex-1 p-6 overflow-y-auto">
      <div className="max-w-6xl mx-auto space-y-6">
        <div className="flex items-center justify-between border-b border-border pb-4">
          <div>
            <h1 className="text-xl font-semibold text-slate-100 capitalize">
              {activeMode} Engine Workspace
            </h1>
            <p className="text-xs text-slate-400 mt-1">
              Phase 3 Studio Presentation Infrastructure for Siragugal Film Studio
            </p>
          </div>
          <span className="text-xs font-mono bg-blue-500/10 text-blue-400 px-3 py-1.5 rounded-full border border-blue-500/20">
            Engine Connected: sira-engine-{activeMode}
          </span>
        </div>

        {activeMode === 'story' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div className="lg:col-span-2">
                <ScreenplayEditor />
              </div>
              <div className="lg:col-span-1">
                <SceneBreakdownPanel />
              </div>
            </div>
            <BeatSheetBoard />
          </div>
        )}

        {activeMode === 'character' && (
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div className="lg:col-span-2">
              <CharacterGallery />
            </div>
            <div className="lg:col-span-1">
              <LoraBindingPanel />
            </div>
          </div>
        )}

        {activeMode === 'actor' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div className="lg:col-span-2">
                <ActorVoiceSelector />
              </div>
              <div className="lg:col-span-1">
                <EmotionalTonePanel />
              </div>
            </div>
            <VisemeTimelineVisualizer />
          </div>
        )}

        {activeMode === 'scene' && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <SceneGraphInspector lang="ta-IN" />
            <TransformInspector lang="ta-IN" />
            <PropRegistryPicker lang="ta-IN" />
          </div>
        )}

        {activeMode === 'director' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div className="lg:col-span-1">
                <DirectorWorkspace lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <ShotListPanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <DirectorNotesPanel lang="ta-IN" />
              </div>
            </div>
            <CameraBlockingView lang="ta-IN" />
          </div>
        )}

        {activeMode === 'cinematography' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
              <div className="lg:col-span-1">
                <CinematographyWorkspace lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <LensProfilePanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <CameraSettingsPanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <LightingControlPanel lang="ta-IN" />
              </div>
            </div>
            <DepthOfFieldVisualizer lang="ta-IN" />
          </div>
        )}

        {activeMode === 'audio' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
              <div className="lg:col-span-1">
                <AudioWorkspace lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <DialogueTrackPanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <SoundLibraryPanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <AudioMixerPanel lang="ta-IN" />
              </div>
            </div>
            <WaveformViewer lang="ta-IN" />
          </div>
        )}

        {activeMode === 'timeline' && (
          <div className="space-y-6">
            <TimecodeRuler lang="ta-IN" />
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div className="lg:col-span-2">
                <MultiTrackCanvas lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <TrimmingToolsPanel lang="ta-IN" />
              </div>
            </div>
            <TimelineWorkspace lang="ta-IN" />
          </div>
        )}

        {activeMode === 'prompts' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
              <div className="lg:col-span-1">
                <PromptBuilderWorkspace lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <PositivePromptEditor lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <NegativePromptEditor lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <GenerationParamsPanel lang="ta-IN" />
              </div>
            </div>
          </div>
        )}

        {activeMode === 'assets' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div className="lg:col-span-1">
                <AssetWorkspace lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <MediaAssetGallery lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <AssetMetadataInspector lang="ta-IN" />
              </div>
            </div>
            <StorageQuotaPanel lang="ta-IN" />
          </div>
        )}

        {activeMode === 'project' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
              <div className="lg:col-span-1">
                <ProjectWorkspace lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <ProjectDashboard lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <ProjectMetadataPanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <VersionHistoryPanel lang="ta-IN" />
              </div>
            </div>
            <ProductionTracker lang="ta-IN" />
          </div>
        )}

        {activeMode === 'render' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
              <div className="lg:col-span-1">
                <RenderWorkspace lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <RenderQueuePanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <RenderProgressMonitor lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <RenderResourcePanel lang="ta-IN" />
              </div>
            </div>
            <RenderRecoveryPanel lang="ta-IN" />
          </div>
        )}

        {activeMode === 'collaboration' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
              <div className="lg:col-span-1">
                <CollaborationWorkspace lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <TeamMembersPanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <ReviewThreadPanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <ApprovalWorkflowPanel lang="ta-IN" />
              </div>
            </div>
            <VersionComparisonViewer lang="ta-IN" />
          </div>
        )}

        {activeMode === 'settings' && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
              <div className="lg:col-span-1">
                <SettingsWorkspace lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <SystemPreferencesPanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <HardwareAccelerationPanel lang="ta-IN" />
              </div>
              <div className="lg:col-span-1">
                <LocaleThemeSelector lang="ta-IN" />
              </div>
            </div>
            <SecurityAuditPanel lang="ta-IN" />
          </div>
        )}

        {activeMode !== 'story' && activeMode !== 'character' && activeMode !== 'actor' && activeMode !== 'scene' && activeMode !== 'director' && activeMode !== 'cinematography' && activeMode !== 'audio' && activeMode !== 'timeline' && activeMode !== 'prompts' && activeMode !== 'assets' && activeMode !== 'project' && activeMode !== 'render' && activeMode !== 'collaboration' && activeMode !== 'settings' && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <Card title="Active Engine Specs">
              <div className="text-xs text-slate-400 space-y-2">
                <div className="flex justify-between">
                  <span>IPC Envelope:</span>
                  <span className="text-slate-200 font-mono">v1.0.0</span>
                </div>
                <div className="flex justify-between">
                  <span>Security Standard:</span>
                  <span className="text-slate-200">OWASP ASVS L2</span>
                </div>
                <div className="flex justify-between">
                  <span>Render Target:</span>
                  <span className="text-slate-200">60 FPS</span>
                </div>
              </div>
            </Card>

            <Card title="Sub-Engine Status">
              <div className="text-xs text-slate-400 space-y-2">
                <div className="flex justify-between">
                  <span>IPC Latency:</span>
                  <span className="text-emerald-400 font-mono">&lt; 0.5 ms</span>
                </div>
                <div className="flex justify-between">
                  <span>Memory Quota:</span>
                  <span className="text-slate-200 font-mono">Bounded</span>
                </div>
                <div className="flex justify-between">
                  <span>VRAM Allocation:</span>
                  <span className="text-slate-200 font-mono">Managed</span>
                </div>
              </div>
            </Card>

            <Card title="Module 45 Settings Studio">
              <div className="text-xs text-slate-400 space-y-2">
                <div className="flex justify-between">
                  <span>Primary Locale:</span>
                  <span className="text-slate-200">Tamil (ta-IN)</span>
                </div>
                <div className="flex justify-between">
                  <span>Shell Sync:</span>
                  <span className="text-slate-200 font-mono">sira-studio-app</span>
                </div>
                <div className="flex justify-between">
                  <span>Accessibility:</span>
                  <span className="text-slate-200">WCAG 2.2 AA</span>
                </div>
              </div>
            </Card>
          </div>
        )}
      </div>
    </main>
  );
};
