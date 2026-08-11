import React from 'react';
import { Film, Play, Pause, Layers } from 'lucide-react';
import { useProjectStore } from '../../stores/project.store';
import { useTimelineStore } from '../../stores/timeline.store';

export const Header: React.FC = () => {
  const { projectName } = useProjectStore();
  const { isPlaying, togglePlay, currentFrame } = useTimelineStore();

  return (
    <header className="h-14 glass-panel border-b border-border px-4 flex items-center justify-between select-none">
      <div className="flex items-center gap-3">
        <Film className="w-6 h-6 text-primary" />
        <span className="font-semibold text-slate-100 text-sm tracking-wide">
          Siragugal Film Studio
        </span>
        <span className="text-xs text-slate-400 bg-surface px-2.5 py-1 rounded-full border border-border">
          {projectName}
        </span>
      </div>

      <div className="flex items-center gap-4">
        <button
          onClick={togglePlay}
          className="p-2 rounded-full bg-primary hover:bg-blue-600 text-white shadow-lg transition-transform active:scale-95"
        >
          {isPlaying ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4 ml-0.5" />}
        </button>
        <span className="font-mono text-xs text-slate-300 bg-surface px-3 py-1 rounded border border-border">
          Frame: {currentFrame}
        </span>
      </div>

      <div className="flex items-center gap-2">
        <span className="text-xs text-slate-400 flex items-center gap-1.5">
          <Layers className="w-3.5 h-3.5 text-blue-400" /> Phase 3 Studio Architecture
        </span>
      </div>
    </header>
  );
};
