import React, { useState } from 'react';
import { FileText, Save } from 'lucide-react';

interface DirectorNotesPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const DirectorNotesPanel: React.FC<DirectorNotesPanelProps> = ({ lang = 'ta-IN' }) => {
  const [noteText, setNoteText] = useState(
    lang === 'ta-IN'
      ? 'கதாநாயகனின் உணர்வுகளை வெளிப்படுத்தும் நெருக்கமான கோணம்.'
      : 'Maintain emotional tension on lead protagonist close-up.'
  );

  const titleText = lang === 'ta-IN' ? 'இயக்குனர் குறிப்புகள் (Director Notes)' : 'Director Annotations';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <FileText className="w-4 h-4 text-amber-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <button className="p-1 text-slate-400 hover:text-slate-100 transition-colors">
          <Save className="w-4 h-4" />
        </button>
      </div>

      <textarea
        value={noteText}
        onChange={(e) => setNoteText(e.target.value)}
        className="flex-1 w-full bg-surface border border-border/60 rounded-lg p-3 font-mono text-xs text-slate-200 focus:outline-none focus:ring-2 focus:ring-primary resize-none"
        placeholder="Enter director notes..."
      />
    </div>
  );
};
