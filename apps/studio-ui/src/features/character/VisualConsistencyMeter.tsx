import React from 'react';
import { ShieldCheck } from 'lucide-react';

interface VisualConsistencyMeterProps {
  score: number; // 0.0 to 1.0
  label?: string;
}

export const VisualConsistencyMeter: React.FC<VisualConsistencyMeterProps> = ({
  score,
  label = 'Visual Consistency Score',
}) => {
  const percentage = Math.round(Math.min(Math.max(score, 0), 1) * 100);

  const getStatusColor = (val: number) => {
    if (val >= 85) return 'bg-emerald-500 text-emerald-400';
    if (val >= 70) return 'bg-amber-500 text-amber-400';
    return 'bg-red-500 text-red-400';
  };

  const statusColorClass = getStatusColor(percentage);

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between text-xs">
        <span className="text-slate-300 font-medium flex items-center gap-1.5">
          <ShieldCheck className="w-3.5 h-3.5 text-blue-400" />
          {label}
        </span>
        <span className="font-mono font-semibold text-slate-200">{percentage}%</span>
      </div>

      <div
        className="w-full bg-surface-hover rounded-full h-2 overflow-hidden border border-border/40"
        role="progressbar"
        aria-label={label}
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={percentage}
      >
        <div
          className={`h-full rounded-full transition-all duration-500 ${statusColorClass.split(' ')[0]}`}
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  );
};
