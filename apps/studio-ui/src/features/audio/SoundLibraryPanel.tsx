import React from 'react';
import { Folder, Link } from 'lucide-react';
import { SoundAssetReference } from './types';

interface SoundLibraryPanelProps {
  sounds?: SoundAssetReference[];
  onSelectSound?: (assetId: string) => void;
  lang?: 'ta-IN' | 'en-US';
}

const defaultSounds: SoundAssetReference[] = [
  {
    asset_id: 'ast-audio-foley-footsteps',
    display_name: { 'ta-IN': 'நடக்கும் ஒலி (Foley)', 'en-US': 'Studio Footsteps Foley' },
    category: 'Foley',
  },
  {
    asset_id: 'ast-audio-ambience-rain',
    display_name: { 'ta-IN': 'மழை பின்னணி ஒலி', 'en-US': 'Rain Ambience Loop' },
    category: 'Ambience',
  },
];

export const SoundLibraryPanel: React.FC<SoundLibraryPanelProps> = ({
  sounds = defaultSounds,
  onSelectSound,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'ஒலி நூலகம் (Sound Library)' : 'Foley & SFX Asset Library';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Folder className="w-4 h-4 text-amber-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {sounds.map((s) => {
          const name = s.display_name[lang] || s.display_name['en-US'];
          return (
            <div
              key={s.asset_id}
              onClick={() => onSelectSound && onSelectSound(s.asset_id)}
              className="bg-surface/80 border border-border/60 rounded-lg p-3 hover:border-amber-500/50 transition-colors cursor-pointer"
            >
              <div className="flex items-center justify-between">
                <span className="text-xs font-semibold text-slate-200">{name}</span>
                <span className="text-[10px] text-amber-400 bg-amber-500/10 px-2 py-0.5 rounded">
                  {s.category}
                </span>
              </div>
              <div className="mt-2 text-[10px] font-mono text-slate-400 flex items-center gap-1">
                <Link className="w-3 h-3 text-purple-400" /> AssetId: {s.asset_id}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
