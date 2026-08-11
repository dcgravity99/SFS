import React from 'react';
import { GitBranch, History } from 'lucide-react';
import { ProjectCheckpointView } from './types';

interface VersionHistoryPanelProps {
  checkpoints?: ProjectCheckpointView[];
  lang?: 'ta-IN' | 'en-US';
}

const defaultCheckpoints: ProjectCheckpointView[] = [
  {
    checkpoint_id: 'chk-v1.1.0',
    version_tag: 'v1.1.0-alpha',
    description: { 'ta-IN': 'காட்சி 1 ஒளிப்பதிவு அமைப்பு', 'en-US': 'Scene 1 Cinematography Setup' },
    timestamp: '2026-08-04T08:15:00Z',
  },
  {
    checkpoint_id: 'chk-v1.0.0',
    version_tag: 'v1.0.0-release',
    description: { 'ta-IN': 'திரைக்கதை & ஒலித் தொகுதி நிறைவு', 'en-US': 'Story & Audio Module Integration' },
    timestamp: '2026-08-04T07:00:00Z',
  },
];

export const VersionHistoryPanel: React.FC<VersionHistoryPanelProps> = ({
  checkpoints = defaultCheckpoints,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'பதிப்பு வரலாறு (Checkpoints)' : 'Version History & Checkpoints';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <History className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {checkpoints.map((c) => {
          const desc = c.description[lang] || c.description['en-US'];
          return (
            <div key={c.checkpoint_id} className="bg-surface/80 border border-border/60 rounded-lg p-3">
              <div className="flex items-center justify-between">
                <span className="text-xs font-bold text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded flex items-center gap-1">
                  <GitBranch className="w-3 h-3" /> {c.version_tag}
                </span>
                <span className="text-[10px] font-mono text-slate-400">{new Date(c.timestamp).toLocaleTimeString()}</span>
              </div>
              <p className="text-xs text-slate-300 mt-2">{desc}</p>
              <div className="mt-2 text-[10px] font-mono text-slate-400 border-t border-border/40 pt-2">
                {c.checkpoint_id}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
