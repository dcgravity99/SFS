import React, { useState } from 'react';
import { CheckCircle2, AlertCircle, Clock } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { ApprovalStatus } from './types';

interface ApprovalWorkflowPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const ApprovalWorkflowPanel: React.FC<ApprovalWorkflowPanelProps> = ({ lang = 'ta-IN' }) => {
  const [status, setStatus] = useState<ApprovalStatus>('PendingReview');
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleUpdateApproval = async (newStatus: ApprovalStatus) => {
    setStatus(newStatus);

    const response = await StudioIpcService.executeEngineCommand('collaboration_update_approval', {
      shot_id: 'shot-01-wide',
      approval_status: newStatus,
    });

    if (response.success) {
      setStatusMessage(
        lang === 'ta-IN' ? `ஒப்புதல் நிலை: ${newStatus}` : `Approval Status updated to ${newStatus} via IPC`
      );
    }
  };

  const titleText = lang === 'ta-IN' ? 'ஒப்புதல் நிலை (Approval Workflow)' : 'Shot Approval Workflow';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <CheckCircle2 className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div className="p-4 bg-surface/80 border border-border/60 rounded-lg text-center">
          <div className="text-xs text-slate-400 mb-1">Current Shot Approval Status</div>
          <div className="text-lg font-bold font-mono text-slate-100 flex items-center justify-center gap-2">
            {status === 'Approved' && <CheckCircle2 className="w-5 h-5 text-emerald-400" />}
            {status === 'RevisionsRequested' && <AlertCircle className="w-5 h-5 text-amber-400" />}
            {status === 'PendingReview' && <Clock className="w-5 h-5 text-blue-400" />}
            {status}
          </div>
        </div>

        <div className="space-y-2">
          <button
            onClick={() => handleUpdateApproval('Approved')}
            className={`w-full py-2.5 px-4 font-semibold text-xs rounded-lg border transition-colors flex items-center justify-center gap-2 ${
              status === 'Approved'
                ? 'bg-emerald-500/20 text-emerald-300 border-emerald-500/40'
                : 'bg-surface text-slate-300 border-border/60 hover:bg-surface-hover'
            }`}
          >
            <CheckCircle2 className="w-4 h-4" /> {lang === 'ta-IN' ? 'ஒப்புதலளிக்கப்பட்டது' : 'Approve Shot'}
          </button>

          <button
            onClick={() => handleUpdateApproval('RevisionsRequested')}
            className={`w-full py-2.5 px-4 font-semibold text-xs rounded-lg border transition-colors flex items-center justify-center gap-2 ${
              status === 'RevisionsRequested'
                ? 'bg-amber-500/20 text-amber-300 border-amber-500/40'
                : 'bg-surface text-slate-300 border-border/60 hover:bg-surface-hover'
            }`}
          >
            <AlertCircle className="w-4 h-4" /> {lang === 'ta-IN' ? 'திருத்தம் தேவை' : 'Request Revisions'}
          </button>
        </div>

        {statusMessage && (
          <div className="p-2 bg-emerald-500/10 border border-emerald-500/20 rounded text-[11px] font-mono text-emerald-400">
            {statusMessage}
          </div>
        )}
      </div>
    </div>
  );
};
