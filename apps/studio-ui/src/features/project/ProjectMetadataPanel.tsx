import React, { useState } from 'react';
import { FileText, Save } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { FilmProjectMetadataView } from './types';

interface ProjectMetadataPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const ProjectMetadataPanel: React.FC<ProjectMetadataPanelProps> = ({ lang = 'ta-IN' }) => {
  const [metadata, setMetadata] = useState<FilmProjectMetadataView>({
    project_id: 'prj-siragugal-01',
    title: { 'ta-IN': 'சிறகுகள்', 'en-US': 'Wings of Freedom' },
    synopsis: { 'ta-IN': 'ஒரு சுதந்திரப் போராட்டக் காவியம்.', 'en-US': 'An epic saga of cinema.' },
    director_name: 'Director AG',
    production_house: 'Siragugal Studios',
    target_aspect_ratio: '2.39:1',
    target_fps: 24,
    created_at: '2026-08-04T00:00:00Z',
  });

  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleUpdateMetadata = async () => {
    const response = await StudioIpcService.executeEngineCommand('project_update_metadata', metadata);
    if (response.success) {
      setStatusMessage(lang === 'ta-IN' ? 'திட்ட விபரம் புதுப்பிக்கப்பட்டது' : 'Project Metadata Updated via IPC');
    }
  };

  const titleText = lang === 'ta-IN' ? 'திரைப்பட விபரம் (Film Metadata)' : 'Film Project Metadata';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <FileText className="w-4 h-4 text-blue-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <button
          onClick={handleUpdateMetadata}
          className="p-1 text-slate-400 hover:text-slate-100 transition-colors"
        >
          <Save className="w-4 h-4" />
        </button>
      </div>

      <div className="space-y-3 flex-1 overflow-y-auto">
        <div>
          <label className="text-xs text-slate-400 block mb-1">Title (Tamil)</label>
          <input
            type="text"
            value={metadata.title['ta-IN']}
            onChange={(e) => setMetadata({ ...metadata, title: { ...metadata.title, 'ta-IN': e.target.value } })}
            className="w-full bg-surface border border-border/60 rounded p-2 text-xs font-semibold text-slate-200"
          />
        </div>

        <div>
          <label className="text-xs text-slate-400 block mb-1">Director</label>
          <input
            type="text"
            value={metadata.director_name}
            onChange={(e) => setMetadata({ ...metadata, director_name: e.target.value })}
            className="w-full bg-surface border border-border/60 rounded p-2 text-xs text-slate-200"
          />
        </div>

        <div className="grid grid-cols-2 gap-2">
          <div>
            <label className="text-xs text-slate-400 block mb-1">Aspect Ratio</label>
            <input
              type="text"
              value={metadata.target_aspect_ratio}
              onChange={(e) => setMetadata({ ...metadata, target_aspect_ratio: e.target.value })}
              className="w-full bg-surface border border-border/60 rounded p-2 text-xs font-mono text-slate-200"
            />
          </div>

          <div>
            <label className="text-xs text-slate-400 block mb-1">Target FPS</label>
            <input
              type="number"
              value={metadata.target_fps}
              onChange={(e) => setMetadata({ ...metadata, target_fps: parseInt(e.target.value, 10) })}
              className="w-full bg-surface border border-border/60 rounded p-2 text-xs font-mono text-slate-200"
            />
          </div>
        </div>

        {statusMessage && (
          <div className="p-2.5 bg-blue-500/10 border border-blue-500/20 rounded-lg text-[11px] text-blue-400 font-mono">
            {statusMessage}
          </div>
        )}
      </div>
    </div>
  );
};
