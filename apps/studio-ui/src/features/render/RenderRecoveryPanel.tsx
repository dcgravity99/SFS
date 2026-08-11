import React, { useState } from 'react';
import { AlertTriangle, RefreshCw } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { RenderRecoveryView } from './types';

interface RenderRecoveryPanelProps {
  recovery?: RenderRecoveryView;
  lang?: 'ta-IN' | 'en-US';
}

export const RenderRecoveryPanel: React.FC<RenderRecoveryPanelProps> = ({
  recovery = {
    job_id: 'job-render-failed-01',
    last_checkpoint_frame: 94,
    error_message: {
      'ta-IN': 'VRAM ஒதுக்கீடு தாண்டிவிட்ட காரணத்தால் பணி நின்றது.',
      'en-US': 'Render interrupted due to VRAM allocation limit exceeded.',
    },
  },
  lang = 'ta-IN',
}) => {
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleRetryJob = async () => {
    const response = await StudioIpcService.executeEngineCommand('render_retry_job', {
      job_id: recovery.job_id,
      resume_from_frame: recovery.last_checkpoint_frame,
    });

    if (response.success) {
      setStatusMessage(
        lang === 'ta-IN'
          ? `ரெண்டர் பணி பிரேம் ${recovery.last_checkpoint_frame}-லிருந்து தொடரப்பட்டது`
          : `Render Job Resumed from Frame ${recovery.last_checkpoint_frame} via IPC`
      );
    }
  };

  const titleText = lang === 'ta-IN' ? 'தோல்வி மீட்பு (Job Recovery)' : 'Render Job Failure Recovery';
  const errMsg = recovery.error_message[lang] || recovery.error_message['en-US'];

  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <AlertTriangle className="w-4 h-4 text-red-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="p-4 bg-surface/80 border border-border/60 rounded-lg space-y-3">
        <div className="text-xs font-semibold text-red-400 flex items-center gap-1.5">
          <AlertTriangle className="w-3.5 h-3.5" /> {errMsg}
        </div>

        <div className="text-[11px] font-mono text-slate-300">
          Last Valid Saved Checkpoint Frame: <span className="text-emerald-400 font-bold">{recovery.last_checkpoint_frame}</span>
        </div>

        <button
          onClick={handleRetryJob}
          className="inline-flex items-center gap-1.5 px-3 py-1.5 bg-red-600 hover:bg-red-500 text-white font-medium text-xs rounded-lg transition-colors shadow-lg shadow-red-500/20"
        >
          <RefreshCw className="w-3.5 h-3.5" />
          {lang === 'ta-IN' ? 'மீண்டும் முயற்சி (Resume Render)' : 'Resume Render from Checkpoint'}
        </button>

        {statusMessage && (
          <div className="p-2 bg-emerald-500/10 border border-emerald-500/20 rounded text-[10px] font-mono text-emerald-400">
            {statusMessage}
          </div>
        )}
      </div>
    </div>
  );
};
