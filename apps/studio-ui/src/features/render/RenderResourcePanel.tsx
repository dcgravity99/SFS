import React from 'react';
import { Cpu } from 'lucide-react';
import { GpuResourceTelemetryView } from './types';

interface RenderResourcePanelProps {
  telemetry?: GpuResourceTelemetryView;
  lang?: 'ta-IN' | 'en-US';
}

export const RenderResourcePanel: React.FC<RenderResourcePanelProps> = ({
  telemetry = {
    gpu_name: 'NVIDIA GeForce RTX 4090',
    vram_used_bytes: 19756843008,
    vram_total_bytes: 25769803776,
    gpu_utilization_percent: 88,
  },
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'GPU பயன்பாடு (GPU & VRAM Telemetry)' : 'Hardware Resource Telemetry';
  const vramUsedGb = (telemetry.vram_used_bytes / 1024 / 1024 / 1024).toFixed(1);
  const vramTotalGb = (telemetry.vram_total_bytes / 1024 / 1024 / 1024).toFixed(0);

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Cpu className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div className="bg-surface/80 border border-border/60 rounded-lg p-3">
          <div className="text-xs font-semibold text-slate-200">{telemetry.gpu_name}</div>
          <div className="text-[10px] font-mono text-emerald-400 mt-1">PCIe Gen4 x16 Active</div>
        </div>

        <div className="space-y-2">
          <div className="flex justify-between text-xs text-slate-300">
            <span>GPU Compute Utilization</span>
            <span className="font-mono text-emerald-400">{telemetry.gpu_utilization_percent}%</span>
          </div>
          <div className="w-full bg-surface-hover h-2 rounded-full overflow-hidden">
            <div className="bg-emerald-400 h-full transition-all" style={{ width: `${telemetry.gpu_utilization_percent}%` }} />
          </div>
        </div>

        <div className="space-y-2 pt-2">
          <div className="flex justify-between text-xs text-slate-300">
            <span>VRAM Allocation</span>
            <span className="font-mono text-purple-400">{vramUsedGb} GB / {vramTotalGb} GB</span>
          </div>
          <div className="w-full bg-surface-hover h-2 rounded-full overflow-hidden">
            <div className="bg-purple-400 h-full transition-all" style={{ width: `${Math.round((telemetry.vram_used_bytes / telemetry.vram_total_bytes) * 100)}%` }} />
          </div>
        </div>
      </div>
    </div>
  );
};
