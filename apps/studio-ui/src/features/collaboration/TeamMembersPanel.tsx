import React from 'react';
import { Users, UserCheck } from 'lucide-react';
import { TeamMemberView } from './types';

interface TeamMembersPanelProps {
  members?: TeamMemberView[];
  lang?: 'ta-IN' | 'en-US';
}

const defaultMembers: TeamMemberView[] = [
  { artist_id: 'artist-dir-ag', display_name: 'AG (Director)', role: 'Director', is_online: true },
  { artist_id: 'artist-dp-selva', display_name: 'Selva (Cinematographer)', role: 'Cinematographer', is_online: true },
  { artist_id: 'artist-sound-ar', display_name: 'AR (Audio Lead)', role: 'AudioEngineer', is_online: false },
];

export const TeamMembersPanel: React.FC<TeamMembersPanelProps> = ({
  members = defaultMembers,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'குழு உறுப்பினர்கள் (Team Members)' : 'Team Roster & Roles';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Users className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {members.map((m) => (
          <div key={m.artist_id} className="bg-surface/80 border border-border/60 rounded-lg p-3">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <UserCheck className={`w-3.5 h-3.5 ${m.is_online ? 'text-emerald-400' : 'text-slate-400'}`} />
                <span className="text-xs font-semibold text-slate-200">{m.display_name}</span>
              </div>
              <span className="text-[10px] font-mono text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded">
                {m.role}
              </span>
            </div>
            <div className="mt-2 text-[10px] font-mono text-slate-400 border-t border-border/40 pt-2 flex justify-between">
              <span>{m.artist_id}</span>
              <span>{m.is_online ? 'Online' : 'Offline'}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
