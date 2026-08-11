import React from 'react';
import { CheckCircle2, Clock } from 'lucide-react';
import { ProductionMilestoneView } from './types';

interface ProductionTrackerProps {
  milestones?: ProductionMilestoneView[];
  lang?: 'ta-IN' | 'en-US';
}

const defaultMilestones: ProductionMilestoneView[] = [
  { milestone_id: 'm1', name: { 'ta-IN': 'திரைக்கதை வளர்ச்சி', 'en-US': 'Script Development' }, progress_percent: 100, is_completed: true },
  { milestone_id: 'm2', name: { 'ta-IN': 'கதாபாத்திர அமைப்பு', 'en-US': 'Character Setup' }, progress_percent: 100, is_completed: true },
  { milestone_id: 'm3', name: { 'ta-IN': 'காட்சி அமைப்பு', 'en-US': 'Scene Assembly' }, progress_percent: 80, is_completed: false },
  { milestone_id: 'm4', name: { 'ta-IN': 'ஒளிப்பதிவு', 'en-US': 'Cinematography' }, progress_percent: 60, is_completed: false },
  { milestone_id: 'm5', name: { 'ta-IN': 'இறுதி தயாரிப்பு', 'en-US': 'Final Master Render' }, progress_percent: 0, is_completed: false },
];

export const ProductionTracker: React.FC<ProductionTrackerProps> = ({
  milestones = defaultMilestones,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'தயாரிப்பு நிலை (Production Tracker)' : 'Production Milestone Tracker';

  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Clock className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3">
        {milestones.map((m) => {
          const mName = m.name[lang] || m.name['en-US'];
          return (
            <div key={m.milestone_id} className="space-y-1">
              <div className="flex justify-between items-center text-xs">
                <span className="text-slate-300 flex items-center gap-1.5">
                  {m.is_completed ? (
                    <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" />
                  ) : (
                    <Clock className="w-3.5 h-3.5 text-slate-400" />
                  )}
                  {mName}
                </span>
                <span className="font-mono text-emerald-400">{m.progress_percent}%</span>
              </div>
              <div className="w-full bg-surface-hover h-1.5 rounded-full overflow-hidden">
                <div className="bg-emerald-400 h-full transition-all" style={{ width: `${m.progress_percent}%` }} />
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
