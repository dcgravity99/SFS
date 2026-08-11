import React from 'react';
import { Settings, Shield, Globe } from 'lucide-react';

interface SettingsWorkspaceProps {
  lang?: 'ta-IN' | 'en-US';
}

export const SettingsWorkspace: React.FC<SettingsWorkspaceProps> = ({ lang = 'ta-IN' }) => {
  const titleText = lang === 'ta-IN' ? 'அமைப்புகள் (Studio Settings Workspace)' : 'Studio Settings & Configuration';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Settings className="w-4 h-4 text-purple-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded border border-purple-500/20">
          Shell Sync Active
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 flex-1">
        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Globe className="w-3.5 h-3.5 text-blue-400" /> Primary Product Language
            </span>
            <div className="mt-3 text-xl font-bold font-mono text-slate-100">Tamil (ta-IN) Active</div>
            <p className="text-xs text-slate-400 mt-2">
              SFS Tamil-first globalization architecture with English secondary fallback.
            </p>
          </div>
          <div className="text-[10px] font-mono text-blue-400 border-t border-border/40 pt-2">
            Globalization Standard: ta-IN
          </div>
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Shield className="w-3.5 h-3.5 text-emerald-400" /> Security Audit Compliance
            </span>
            <div className="mt-3 text-lg font-semibold text-emerald-400">OWASP ASVS Level 2</div>
            <p className="text-xs text-slate-400 mt-2">
              CSP strict protection enabled. AssetId handles enforced.
            </p>
          </div>
          <div className="text-[10px] font-mono text-emerald-400 border-t border-border/40 pt-2">
            Security Status: Compliant
          </div>
        </div>
      </div>
    </div>
  );
};
