import React from 'react';
import { Film, Layers } from 'lucide-react';

interface ProjectWorkspaceProps {
  lang?: 'ta-IN' | 'en-US';
}

export const ProjectWorkspace: React.FC<ProjectWorkspaceProps> = ({ lang = 'ta-IN' }) => {
  const titleText = lang === 'ta-IN' ? 'திரைப்படத் திட்டம் (Project Workspace)' : 'Film Project Workspace';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Film className="w-4 h-4 text-blue-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-blue-400 bg-blue-500/10 px-2 py-0.5 rounded border border-blue-500/20">
          Core Engine: sira_core
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 flex-1">
        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Film className="w-3.5 h-3.5 text-purple-400" /> Production Overview
            </span>
            <div className="mt-3 text-xl font-bold font-mono text-slate-100">Feature Film: Wings of Freedom</div>
            <p className="text-xs text-slate-400 mt-2">
              Primary Tamil (`ta-IN`) film orchestration with English fallback.
            </p>
          </div>
          <div className="text-[10px] font-mono text-blue-400 border-t border-border/40 pt-2 flex items-center gap-1">
            <Layers className="w-3 h-3" /> Sub-Engine: sira_core
          </div>
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Film className="w-3.5 h-3.5 text-emerald-400" /> Project Milestones Status
            </span>
            <div className="mt-3 text-lg font-semibold text-emerald-400">Cinematography Active</div>
            <p className="text-xs text-slate-400 mt-2">
              65% Total Production Progress.
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
