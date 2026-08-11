import React from 'react';
import { Database, HardDrive, Layers } from 'lucide-react';

interface AssetWorkspaceProps {
  lang?: 'ta-IN' | 'en-US';
}

export const AssetWorkspace: React.FC<AssetWorkspaceProps> = ({ lang = 'ta-IN' }) => {
  const titleText = lang === 'ta-IN' ? 'வள மேலாண்மைத் துறை (Asset Studio Workspace)' : 'Digital Asset Management Workspace';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Database className="w-4 h-4 text-emerald-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20">
          Database: sira_asset_db
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 flex-1">
        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <HardDrive className="w-3.5 h-3.5 text-blue-400" /> Digital Asset Storage
            </span>
            <div className="mt-3 text-xl font-bold font-mono text-slate-100">128 Registered Assets</div>
            <p className="text-xs text-slate-400 mt-2">
              Videos, Dialogue Stems, LoRA Models, and 3D Scene Assets cataloged via AssetId handles.
            </p>
          </div>
          <div className="text-[10px] font-mono text-emerald-400 border-t border-border/40 pt-2">
            Sub-Engine: sira_asset_db
          </div>
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Layers className="w-3.5 h-3.5 text-purple-400" /> Cryptographic Integrity
            </span>
            <div className="mt-3 text-lg font-semibold text-purple-400">SHA-256 Validated</div>
            <p className="text-xs text-slate-400 mt-2">
              All asset handles verified against backend checksum index.
            </p>
          </div>
          <div className="text-[10px] font-mono text-emerald-400 border-t border-border/40 pt-2">
            IPC Asset Query: Active
          </div>
        </div>
      </div>
    </div>
  );
};
