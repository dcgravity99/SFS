import React, { useState } from 'react';
import { Ban } from 'lucide-react';

interface NegativePromptEditorProps {
  lang?: 'ta-IN' | 'en-US';
}

export const NegativePromptEditor: React.FC<NegativePromptEditorProps> = ({ lang = 'ta-IN' }) => {
  const [negativePrompt, setNegativePrompt] = useState(
    'blurry, low quality, distorted anatomy, extra limbs, bad lighting, cartoon, illustration'
  );

  const titleText = lang === 'ta-IN' ? 'எதிர்மறை குறிப்பு (Negative Prompt)' : 'Negative Prompt Exclusions';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Ban className="w-4 h-4 text-red-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <textarea
        value={negativePrompt}
        onChange={(e) => setNegativePrompt(e.target.value)}
        className="flex-1 w-full bg-surface border border-border/60 rounded-lg p-3 font-mono text-xs text-slate-200 focus:outline-none focus:ring-2 focus:ring-red-500 resize-none mb-3"
        placeholder="Enter elements to exclude..."
      />

      <div className="flex flex-wrap gap-1.5 pt-2 border-t border-border/40">
        {['Low Quality', 'Blurry', 'Bad Lighting', 'Distorted'].map((tag) => (
          <button
            key={tag}
            onClick={() => setNegativePrompt((prev) => `${prev}, ${tag}`)}
            className="inline-flex items-center gap-1 px-2 py-1 text-[10px] bg-red-500/10 hover:bg-red-500/20 text-red-300 border border-red-500/20 rounded transition-colors"
          >
            {tag}
          </button>
        ))}
      </div>
    </div>
  );
};
