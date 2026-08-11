import React from 'react';
import { Activity, ShieldCheck, Cpu } from 'lucide-react';

export const StatusBar: React.FC = () => {
  return (
    <footer className="h-8 glass-panel border-t border-border px-4 flex items-center justify-between text-xs text-slate-400 select-none">
      <div className="flex items-center gap-4">
        <span className="flex items-center gap-1.5 text-emerald-400">
          <Activity className="w-3.5 h-3.5" /> Ready
        </span>
        <span className="flex items-center gap-1.5">
          <ShieldCheck className="w-3.5 h-3.5 text-blue-400" /> OWASP ASVS L2 Secured
        </span>
      </div>

      <div className="flex items-center gap-4">
        <span className="flex items-center gap-1.5">
          <Cpu className="w-3.5 h-3.5 text-indigo-400" /> Hardware Accelerated
        </span>
        <span>Siragugal Studio v1.0.0</span>
      </div>
    </footer>
  );
};
