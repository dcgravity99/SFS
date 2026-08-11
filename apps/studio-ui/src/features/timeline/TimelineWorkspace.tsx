import React from 'react';
import { Sliders, Layers } from 'lucide-react';

interface TimelineWorkspaceProps {
  lang?: 'ta-IN' | 'en-US';
}

export const TimelineWorkspace: React.FC<TimelineWorkspaceProps> = ({ lang = 'ta-IN' }) => {
  const titleText = lang === 'ta-IN' ? 'நேரக்கோடு திருத்தி (NLE Timeline Editor)' : 'NLE Timeline Editor Workspace';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Sliders className="w-4 h-4 text-blue-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-blue-400 bg-blue-500/10 px-2 py-0.5 rounded border border-blue-500/20">
          Tracks V1-V4, A1-A4, T1 Active
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 flex-1">
        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300">Multi-Track NLE Layout</span>
            <div className="mt-3 text-xl font-bold font-mono text-slate-100">9 Active Tracks</div>
            <p className="text-xs text-slate-400 mt-2">
              Non-linear video, audio stems, and subtitle timelines synchronized to 24 FPS.
            </p>
          </div>
          <div className="text-[10px] font-mono text-blue-400 border-t border-border/40 pt-2 flex items-center gap-1">
            <Layers className="w-3 h-3" /> Sub-Engine: sira_engine_timeline
          </div>
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300">Editing Tools & Controls</span>
            <div className="mt-3 text-lg font-semibold text-blue-400">Razor Split & Trimming</div>
            <p className="text-xs text-slate-400 mt-2">
              Frame-accurate In/Out trimming & ripple split commands.
            </p>
          </div>
          <div className="text-[10px] font-mono text-emerald-400 border-t border-border/40 pt-2">
            IPC Sync: Active
          </div>
        </div>
      </div>
    </div>
  );
};
