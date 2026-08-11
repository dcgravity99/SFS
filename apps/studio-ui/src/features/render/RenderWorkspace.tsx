import React from 'react';
import { Cpu, Layers } from 'lucide-react';

interface RenderWorkspaceProps {
  lang?: 'ta-IN' | 'en-US';
}

export const RenderWorkspace: React.FC<RenderWorkspaceProps> = ({ lang = 'ta-IN' }) => {
  const titleText = lang === 'ta-IN' ? 'வெளியீட்டுத் துறை (Render Studio Workspace)' : 'Production Render Workspace';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Cpu className="w-4 h-4 text-purple-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded border border-purple-500/20">
          Render Engine: sira_render_engine
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 flex-1">
        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Cpu className="w-3.5 h-3.5 text-blue-400" /> Batch Render Pipeline
            </span>
            <div className="mt-3 text-xl font-bold font-mono text-slate-100">4K / 8K Master Output</div>
            <p className="text-xs text-slate-400 mt-2">
              ProRes 4444 XQ and EXR 16-bit float frame sequence rendering.
            </p>
          </div>
          <div className="text-[10px] font-mono text-purple-400 border-t border-border/40 pt-2 flex items-center gap-1">
            <Layers className="w-3 h-3" /> Sub-Engine: sira_render_engine
          </div>
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Cpu className="w-3.5 h-3.5 text-emerald-400" /> Hardware Telemetry
            </span>
            <div className="mt-3 text-lg font-semibold text-emerald-400">NVIDIA RTX 4090 Active</div>
            <p className="text-xs text-slate-400 mt-2">
              VRAM utilization 18.4 GB / 24.0 GB (76%).
            </p>
          </div>
          <div className="text-[10px] font-mono text-emerald-400 border-t border-border/40 pt-2">
            IPC Telemetry: Active
          </div>
        </div>
      </div>
    </div>
  );
};
