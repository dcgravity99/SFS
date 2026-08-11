import React from 'react';
import { User, Layers, Mic } from 'lucide-react';
import { CharacterProfileView } from './types';
import { VisualConsistencyMeter } from './VisualConsistencyMeter';

interface CharacterGalleryProps {
  characters?: CharacterProfileView[];
}

const defaultCharacters: CharacterProfileView[] = [
  {
    character_id: 'char-vikram-101',
    name: 'Vikram',
    role: 'Lead Protagonist',
    voice_model_id: 'voice-elevenlabs-v1',
    lora_asset_id: 'ast-lora-actor1-v2',
    visual_anchor_count: 14,
    consistency_score: 0.94,
  },
  {
    character_id: 'char-ananya-102',
    name: 'Ananya',
    role: 'Lead Protagonist',
    voice_model_id: 'voice-xtts-v2',
    lora_asset_id: 'ast-lora-heroine-v1',
    visual_anchor_count: 18,
    consistency_score: 0.91,
  },
  {
    character_id: 'char-deva-103',
    name: 'Deva',
    role: 'Antagonist',
    voice_model_id: 'voice-coqui-v3',
    lora_asset_id: 'ast-lora-villain-v3',
    visual_anchor_count: 10,
    consistency_score: 0.88,
  },
];

export const CharacterGallery: React.FC<CharacterGalleryProps> = ({
  characters = defaultCharacters,
}) => {
  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <User className="w-4 h-4 text-blue-400" />
        <h2 className="text-sm font-semibold text-slate-200">Character Identity Gallery</h2>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 overflow-y-auto pr-1">
        {characters.map((char) => (
          <div
            key={char.character_id}
            className="bg-surface/80 border border-border/60 rounded-xl p-4 hover:border-primary/50 transition-colors flex flex-col justify-between"
          >
            <div>
              <div className="flex items-center justify-between">
                <h3 className="text-sm font-semibold text-slate-100">{char.name}</h3>
                <span className="text-[10px] font-medium text-blue-400 bg-blue-500/10 px-2 py-0.5 rounded">
                  {char.role}
                </span>
              </div>
              <p className="text-[10px] font-mono text-slate-400 mt-1">{char.character_id}</p>

              <div className="mt-3 text-xs text-slate-400 space-y-1.5 border-t border-border/40 pt-2">
                <div className="flex items-center justify-between">
                  <span className="flex items-center gap-1">
                    <Layers className="w-3 h-3 text-purple-400" /> LoRA AssetId:
                  </span>
                  <span className="font-mono text-slate-200 text-[11px]">{char.lora_asset_id}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="flex items-center gap-1">
                    <Mic className="w-3 h-3 text-emerald-400" /> Voice ID:
                  </span>
                  <span className="font-mono text-slate-300 text-[11px]">{char.voice_model_id}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span>Visual Anchors:</span>
                  <span className="text-slate-200 font-semibold">{char.visual_anchor_count} vectors</span>
                </div>
              </div>
            </div>

            <div className="mt-4 pt-3 border-t border-border/40">
              <VisualConsistencyMeter score={char.consistency_score} />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
