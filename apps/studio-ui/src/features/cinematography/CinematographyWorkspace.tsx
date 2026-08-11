import React from 'react';
import { Camera, Sun, Sliders } from 'lucide-react';

interface CinematographyWorkspaceProps {
  lang?: 'ta-IN' | 'en-US';
}

export const CinematographyWorkspace: React.FC<CinematographyWorkspaceProps> = ({ lang = 'ta-IN' }) => {
  const titleText = lang === 'ta-IN' ? 'ஒளிப்பதிவுத் துறை (Cinematography Workspace)' : 'Cinematography Optics Workspace';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Camera className="w-4 h-4 text-blue-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-blue-400 bg-blue-500/10 px-2 py-0.5 rounded border border-blue-500/20">
          Optics Active
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 flex-1">
        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Sliders className="w-3.5 h-3.5 text-purple-400" /> Camera Optics Status
            </span>
            <div className="mt-3 text-xl font-bold font-mono text-slate-100">Anamorphic 35mm Prime</div>
            <p className="text-xs text-slate-400 mt-2">
              f/1.8 Aperture • 5600K Color Temp • ISO 800
            </p>
          </div>
          <div className="text-[10px] font-mono text-blue-400 border-t border-border/40 pt-2">
            Sub-Engine: sira_engine_cinematography
          </div>
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Sun className="w-3.5 h-3.5 text-amber-400" /> Three-Point Lighting Rig
            </span>
            <div className="mt-3 text-lg font-semibold text-amber-400">Daylight Studio Profile</div>
            <p className="text-xs text-slate-400 mt-2">
              Key 80% • Fill 45% • Rim 60% • 5600K Balanced
            </p>
          </div>
          <div className="text-[10px] font-mono text-emerald-400 border-t border-border/40 pt-2">
            IPC Sync: Active
          </div>
        </div>
      </div>
    </div>
  );
};
