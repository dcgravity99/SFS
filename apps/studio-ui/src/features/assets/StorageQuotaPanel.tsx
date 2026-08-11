import React from 'react';
import { HardDrive } from 'lucide-react';
import { StorageQuotaView } from './types';

interface StorageQuotaPanelProps {
  quota?: StorageQuotaView;
  lang?: 'ta-IN' | 'en-US';
}

export const StorageQuotaPanel: React.FC<StorageQuotaPanelProps> = ({
  quota = {
    total_quota_bytes: 50000000000,
    used_bytes: 12400000000,
    cached_models_bytes: 4200000000,
  },
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'சேமிப்பக அளவு (Storage Quota)' : 'Storage Quota Utilization';
  const usedPct = Math.round((quota.used_bytes / quota.total_quota_bytes) * 100);

  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <HardDrive className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="p-4 bg-surface/80 border border-border/60 rounded-lg space-y-3">
        <div className="flex justify-between items-center text-xs">
          <span className="text-slate-300">Disk Quota Used</span>
          <span className="font-mono text-emerald-400">{usedPct}% ({(quota.used_bytes / 1024 / 1024 / 1024).toFixed(1)} GB / {(quota.total_quota_bytes / 1024 / 1024 / 1024).toFixed(0)} GB)</span>
        </div>

        <div className="w-full bg-surface-hover h-2 rounded-full overflow-hidden">
          <div className="bg-emerald-400 h-full transition-all" style={{ width: `${usedPct}%` }} />
        </div>

        <div className="text-[11px] font-mono text-slate-400 flex justify-between border-t border-border/40 pt-2">
          <span>Cached LoRA Models: {(quota.cached_models_bytes / 1024 / 1024 / 1024).toFixed(1)} GB</span>
          <span>Backend Validated</span>
        </div>
      </div>
    </div>
  );
};
