import React, { useState } from 'react';
import { Scissors, Play } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';

interface TrimmingToolsPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const TrimmingToolsPanel: React.FC<TrimmingToolsPanelProps> = ({ lang = 'ta-IN' }) => {
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleRazorSplit = async () => {
    const response = await StudioIpcService.executeEngineCommand('timeline_split_clip', {
      clip_id: 'clip-v1-01',
      split_frame: 48,
    });

    if (response.success) {
      setStatusMessage(lang === 'ta-IN' ? 'துண்டு வெட்டப்பட்டது (Split Complete)' : 'Razor Split Dispatched via IPC');
    }
  };

  const handleTrimIn = async () => {
    const response = await StudioIpcService.executeEngineCommand('timeline_trim_clip', {
      clip_id: 'clip-v1-01',
      trim_in_frame: 12,
    });

    if (response.success) {
      setStatusMessage(lang === 'ta-IN' ? 'தொடக்க ஒழுங்கு செய்யப்பட்டது (Trim In Complete)' : 'Trim In Dispatched via IPC');
    }
  };

  const titleText = lang === 'ta-IN' ? 'ஒழுங்கு கருவிகள் (Trimming Tools)' : 'Razor & Trimming Tools';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Scissors className="w-4 h-4 text-blue-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <button
          onClick={handleRazorSplit}
          aria-label={lang === 'ta-IN' ? 'வெட்டு' : 'Razor Split'}
          className="w-full flex items-center justify-center gap-2 py-2.5 px-4 bg-primary hover:bg-blue-600 text-white font-medium text-xs rounded-lg transition-colors shadow-lg shadow-blue-500/20"
        >
          <Scissors className="w-4 h-4" />
          {lang === 'ta-IN' ? 'பிளவு / வெட்டு (Razor Split)' : 'Razor Split at Playhead (IPC)'}
        </button>

        <div className="grid grid-cols-2 gap-2">
          <button
            onClick={handleTrimIn}
            aria-label={lang === 'ta-IN' ? 'தொடக்க ஒழுங்கு' : 'Trim In'}
            className="flex items-center justify-center gap-1.5 py-2 px-3 bg-surface hover:bg-surface-hover text-slate-200 border border-border/60 font-medium text-xs rounded-lg transition-colors"
          >
            <Scissors className="w-3.5 h-3.5 text-emerald-400" />
            {lang === 'ta-IN' ? 'தொடக்க ஒழுங்கு' : 'Trim In'}
          </button>
          <button
            onClick={handleTrimIn}
            aria-label={lang === 'ta-IN' ? 'முடிவு ஒழுங்கு' : 'Trim Out'}
            className="flex items-center justify-center gap-1.5 py-2 px-3 bg-surface hover:bg-surface-hover text-slate-200 border border-border/60 font-medium text-xs rounded-lg transition-colors"
          >
            <Play className="w-3.5 h-3.5 text-purple-400" />
            {lang === 'ta-IN' ? 'முடிவு ஒழுங்கு' : 'Trim Out'}
          </button>
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
