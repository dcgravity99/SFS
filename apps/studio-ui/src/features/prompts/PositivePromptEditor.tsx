import React, { useState } from 'react';
import { Sparkles, Tag } from 'lucide-react';

interface PositivePromptEditorProps {
  lang?: 'ta-IN' | 'en-US';
}

export const PositivePromptEditor: React.FC<PositivePromptEditorProps> = ({ lang = 'ta-IN' }) => {
  const [positivePrompt, setPositivePrompt] = useState(
    'Cinematic wide shot of lead protagonist Vikram standing in a rainy Chennai alleyway, 35mm film grain, volumetric lighting, photorealistic, 8k resolution'
  );

  const titleText = lang === 'ta-IN' ? 'நேர்மறை குறிப்பு (Positive Prompt)' : 'Positive Prompt Editor';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Sparkles className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <textarea
        value={positivePrompt}
        onChange={(e) => setPositivePrompt(e.target.value)}
        className="flex-1 w-full bg-surface border border-border/60 rounded-lg p-3 font-mono text-xs text-slate-200 focus:outline-none focus:ring-2 focus:ring-purple-500 resize-none mb-3"
        placeholder="Enter visual description prompt..."
      />

      <div className="flex flex-wrap gap-1.5 pt-2 border-t border-border/40">
        {['Cinematic 35mm', 'Volumetric Lighting', 'Master Shot', 'Photorealistic'].map((tag) => (
          <button
            key={tag}
            onClick={() => setPositivePrompt((prev) => `${prev}, ${tag}`)}
            className="inline-flex items-center gap-1 px-2 py-1 text-[10px] bg-purple-500/10 hover:bg-purple-500/20 text-purple-300 border border-purple-500/20 rounded transition-colors"
          >
            <Tag className="w-2.5 h-2.5" /> {tag}
          </button>
        ))}
      </div>
    </div>
  );
};
