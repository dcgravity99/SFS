import React from 'react';
import { Play } from 'lucide-react';
import { RenderProgressView } from './types';

interface RenderProgressMonitorProps {
  progress?: RenderProgressView;
  lang?: 'ta-IN' | 'en-US';
}

export const RenderProgressMonitor: React.FC<RenderProgressMonitorProps> = ({
  progress = {
    job_id: 'job-render-scene1-master',
    current_frame: 142,
    total_frames: 360,
    current_pass: 'Path Tracing Sample 256/256',
    eta_seconds: 48,
  },
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'முன்னேற்றக் கண்காணிப்பு (Progress Monitor)' : 'Frame Progress Monitor';
  const pct = Math.round((progress.current_frame / progress.total_frames) * 100);

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Play className="w-4 h-4 text-blue-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div className="bg-surface/80 border border-border/60 rounded-lg p-4 text-center">
          <div className="text-3xl font-bold font-mono text-blue-400">{pct}%</div>
          <div className="text-xs text-slate-300 mt-1">Frame {progress.current_frame} of {progress.total_frames}</div>
          <div className="text-[10px] font-mono text-slate-400 mt-2">{progress.current_pass}</div>
        </div>

        <div className="w-full bg-surface-hover h-2.5 rounded-full overflow-hidden">
          <div className="bg-blue-400 h-full transition-all" style={{ width: `${pct}%` }} />
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-3 text-xs font-mono text-slate-300 flex justify-between">
          <span>Estimated Time Remaining (ETA):</span>
          <span className="text-emerald-400 font-bold">{progress.eta_seconds}s</span>
        </div>
      </div>
    </div>
  );
};
