import React from 'react';
import { Video, Layers } from 'lucide-react';

interface DirectorWorkspaceProps {
  shotsCount?: number;
  lang?: 'ta-IN' | 'en-US';
}

export const DirectorWorkspace: React.FC<DirectorWorkspaceProps> = ({
  shotsCount = 12,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'இயக்குனர் திட்டம் (Director Workspace)' : 'Director Shot Workspace';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Video className="w-4 h-4 text-purple-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded border border-purple-500/20">
          Scene #1 Mapped
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 flex-1">
        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300">Shot Sequence Status</span>
            <div className="mt-3 text-2xl font-bold font-mono text-slate-100">{shotsCount} Shots Planned</div>
            <p className="text-xs text-slate-400 mt-2">
              Coverage established across establishing, master, and close-up angles.
            </p>
          </div>
          <div className="text-[10px] font-mono text-purple-400 flex items-center gap-1 border-t border-border/40 pt-2">
            <Layers className="w-3 h-3" /> Sub-Engine: sira_engine_director
          </div>
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300">Pacing & Intent Profile</span>
            <div className="mt-3 text-lg font-semibold text-purple-400">Dramatic Tension Pivot</div>
            <p className="text-xs text-slate-400 mt-2">
              Pacing slows down gradually into Scene 1 climax.
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
