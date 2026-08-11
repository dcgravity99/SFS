import React from 'react';
import { Play, CheckCircle, AlertTriangle, Clock } from 'lucide-react';
import { RenderJobView } from './types';

interface RenderJobCardProps {
  job: RenderJobView;
  onCancel?: (jobId: string) => void;
  lang?: 'ta-IN' | 'en-US';
}

export const RenderJobCard: React.FC<RenderJobCardProps> = ({ job, lang = 'ta-IN' }) => {
  const name = job.display_name[lang] || job.display_name['en-US'];

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'Rendering':
        return <span className="text-[10px] font-mono text-blue-400 bg-blue-500/10 px-2 py-0.5 rounded flex items-center gap-1"><Play className="w-2.5 h-2.5" /> Rendering</span>;
      case 'Completed':
        return <span className="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded flex items-center gap-1"><CheckCircle className="w-2.5 h-2.5" /> Completed</span>;
      case 'Failed':
        return <span className="text-[10px] font-mono text-red-400 bg-red-500/10 px-2 py-0.5 rounded flex items-center gap-1"><AlertTriangle className="w-2.5 h-2.5" /> Failed</span>;
      default:
        return <span className="text-[10px] font-mono text-slate-400 bg-surface px-2 py-0.5 rounded flex items-center gap-1"><Clock className="w-2.5 h-2.5" /> Queued</span>;
    }
  };

  return (
    <div className="bg-surface/80 border border-border/60 rounded-lg p-3 space-y-2">
      <div className="flex items-center justify-between">
        <span className="text-xs font-semibold text-slate-200">{name}</span>
        {getStatusBadge(job.status)}
      </div>

      <div className="text-[10px] font-mono text-slate-400 flex justify-between">
        <span>Format: {job.output_format}</span>
        <span>Frame: {job.current_frame}/{job.total_frames}</span>
      </div>

      {job.status === 'Rendering' && (
        <div className="w-full bg-surface-hover h-1.5 rounded-full overflow-hidden">
          <div className="bg-blue-400 h-full transition-all" style={{ width: `${Math.round((job.current_frame / job.total_frames) * 100)}%` }} />
        </div>
      )}
    </div>
  );
};
