import React from 'react';
import { Columns } from 'lucide-react';

interface VersionComparisonViewerProps {
  lang?: 'ta-IN' | 'en-US';
}

export const VersionComparisonViewer: React.FC<VersionComparisonViewerProps> = ({ lang = 'ta-IN' }) => {
  const titleText = lang === 'ta-IN' ? 'பதிப்பு ஒப்பீடு (Version Comparison)' : 'Side-by-Side Version Comparison';

  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Columns className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="grid grid-cols-2 gap-4 p-4 bg-surface/80 border border-border/60 rounded-lg">
        <div className="border border-border/40 rounded p-3 text-center bg-background/50">
          <span className="text-xs font-mono text-slate-400">Version v1.0 (Previous)</span>
          <div className="mt-2 text-xs font-semibold text-slate-300">Draft Render v1</div>
        </div>
        <div className="border border-purple-500/40 rounded p-3 text-center bg-purple-500/10">
          <span className="text-xs font-mono text-purple-300">Version v2.0 (Current)</span>
          <div className="mt-2 text-xs font-semibold text-slate-100">Master Approved Render</div>
        </div>
      </div>
    </div>
  );
};
