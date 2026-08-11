import React from 'react';
import { Users, MessageSquare, Layers } from 'lucide-react';

interface CollaborationWorkspaceProps {
  lang?: 'ta-IN' | 'en-US';
}

export const CollaborationWorkspace: React.FC<CollaborationWorkspaceProps> = ({ lang = 'ta-IN' }) => {
  const titleText = lang === 'ta-IN' ? 'குழுப் பணித் துறை (Collaboration Studio Workspace)' : 'Team Collaboration Workspace';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Users className="w-4 h-4 text-emerald-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20">
          Team Online: 5 Artists
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 flex-1">
        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Users className="w-3.5 h-3.5 text-blue-400" /> Multi-User Team Roster
            </span>
            <div className="mt-3 text-xl font-bold font-mono text-slate-100">5 Active Production Roles</div>
            <p className="text-xs text-slate-400 mt-2">
              Director, Cinematographer, Lead Animator, Audio Engineer, and Editor connected.
            </p>
          </div>
          <div className="text-[10px] font-mono text-blue-400 border-t border-border/40 pt-2 flex items-center gap-1">
            <Layers className="w-3 h-3" /> Sub-Engine: sira_core_collaboration
          </div>
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <MessageSquare className="w-3.5 h-3.5 text-purple-400" /> Timecode Shot Review
            </span>
            <div className="mt-3 text-lg font-semibold text-purple-400">12 Review Annotations</div>
            <p className="text-xs text-slate-400 mt-2">
              Frame-accurate review comments linked to timeline SMPTE markers.
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
