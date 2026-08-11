import React from 'react';
import { UserCheck, BarChart2 } from 'lucide-react';
import { CharacterDialogueStat } from './types';

interface SceneBreakdownPanelProps {
  stats?: CharacterDialogueStat[];
}

const defaultStats: CharacterDialogueStat[] = [
  { character_name: 'DIRECTOR', line_count: 12, word_count: 145 },
  { character_name: 'LEAD ACTOR', line_count: 18, word_count: 230 },
  { character_name: 'CINEMATOGRAPHER', line_count: 6, word_count: 82 },
];

export const SceneBreakdownPanel: React.FC<SceneBreakdownPanelProps> = ({ stats = defaultStats }) => {
  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <BarChart2 className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">Character Dialogue Breakdown</h2>
      </div>

      <div className="space-y-4 overflow-y-auto pr-1">
        {stats.map((stat) => (
          <div key={stat.character_name} className="bg-surface/60 border border-border/40 rounded-lg p-3">
            <div className="flex items-center justify-between mb-2">
              <span className="text-xs font-semibold text-slate-200 flex items-center gap-1.5">
                <UserCheck className="w-3.5 h-3.5 text-blue-400" />
                {stat.character_name}
              </span>
              <span className="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded">
                {stat.word_count} words
              </span>
            </div>
            <div className="w-full bg-surface-hover rounded-full h-1.5 overflow-hidden">
              <div
                className="bg-primary h-full rounded-full"
                style={{ width: `${Math.min((stat.word_count / 300) * 100, 100)}%` }}
              />
            </div>
            <div className="mt-2 text-[10px] text-slate-400 flex justify-between">
              <span>Lines: {stat.line_count}</span>
              <span>Distribution: {Math.round((stat.word_count / 457) * 100)}%</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
