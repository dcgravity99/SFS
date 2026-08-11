import React from 'react';
import { Sparkles, Cpu, Layers } from 'lucide-react';

interface PromptBuilderWorkspaceProps {
  lang?: 'ta-IN' | 'en-US';
}

export const PromptBuilderWorkspace: React.FC<PromptBuilderWorkspaceProps> = ({ lang = 'ta-IN' }) => {
  const titleText = lang === 'ta-IN' ? 'செயற்கை நுண்ணறிவு குறிப்புத் தயாரிப்பு (AI Prompt Builder)' : 'AI Prompt Builder Workspace';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Sparkles className="w-4 h-4 text-purple-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded border border-purple-500/20">
          Provider Connected: sira_ai_provider
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 flex-1">
        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Cpu className="w-3.5 h-3.5 text-blue-400" /> AI Generation Job Preparation
            </span>
            <div className="mt-3 text-xl font-bold font-mono text-slate-100">Structured Prompt Pipeline</div>
            <p className="text-xs text-slate-400 mt-2">
              Assembly of positive, negative, and LoRA character weight anchors.
            </p>
          </div>
          <div className="text-[10px] font-mono text-purple-400 border-t border-border/40 pt-2">
            Sub-Engine: sira_ai_provider
          </div>
        </div>

        <div className="bg-surface/60 border border-border/40 rounded-lg p-4 flex flex-col justify-between">
          <div>
            <span className="text-xs font-semibold text-slate-300 flex items-center gap-1.5">
              <Layers className="w-3.5 h-3.5 text-emerald-400" /> Character & Style Consistency
            </span>
            <div className="mt-3 text-lg font-semibold text-emerald-400">LoRA Binding Active</div>
            <p className="text-xs text-slate-400 mt-2">
              CharacterId visual anchor attached to generation payload.
            </p>
          </div>
          <div className="text-[10px] font-mono text-emerald-400 border-t border-border/40 pt-2">
            IPC Job Submit: Ready
          </div>
        </div>
      </div>
    </div>
  );
};
