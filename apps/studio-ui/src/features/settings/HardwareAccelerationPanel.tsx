import React, { useState } from 'react';
import { Cpu } from 'lucide-react';
import { HardwareAccelerationConfigView } from './types';

interface HardwareAccelerationPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const HardwareAccelerationPanel: React.FC<HardwareAccelerationPanelProps> = ({ lang = 'ta-IN' }) => {
  const [config, setConfig] = useState<HardwareAccelerationConfigView>({
    gpu_name: 'NVIDIA GeForce RTX 4090',
    backend_api: 'CUDA',
    is_enabled: true,
    max_vram_allocation_mb: 20480,
  });

  const titleText = lang === 'ta-IN' ? 'வன்பொருள் முடுக்கம் (Hardware Acceleration)' : 'Hardware Acceleration Settings';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Cpu className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div className="p-3 bg-surface/80 border border-border/60 rounded-lg">
          <div className="text-xs font-semibold text-slate-200">{config.gpu_name}</div>
          <div className="text-[10px] font-mono text-emerald-400 mt-1">Backend: {config.backend_api}</div>
        </div>

        <div className="flex justify-between items-center p-3 bg-surface/80 border border-border/60 rounded-lg">
          <span className="text-xs text-slate-300">GPU Acceleration Status</span>
          <button
            onClick={() => setConfig({ ...config, is_enabled: !config.is_enabled })}
            className={`px-3 py-1 text-xs font-mono rounded font-bold transition-colors ${
              config.is_enabled ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/40' : 'bg-surface text-slate-400'
            }`}
          >
            {config.is_enabled ? 'ENABLED' : 'DISABLED'}
          </button>
        </div>
      </div>
    </div>
  );
};
