import React from 'react';
import { Layers } from 'lucide-react';
import { StoryBeatView } from './types';

interface BeatSheetBoardProps {
  beats?: StoryBeatView[];
}

const defaultBeats: StoryBeatView[] = [
  {
    beat_id: 'beat-1',
    beat_type: 'Opening Image',
    scene_ids: [1],
    description: 'Establishes the visual tone and studio setting.',
  },
  {
    beat_id: 'beat-2',
    beat_type: 'Catalyst',
    scene_ids: [2],
    description: 'Inciting incident disrupting status quo.',
  },
  {
    beat_id: 'beat-3',
    beat_type: 'Midpoint',
    scene_ids: [5],
    description: 'Stakes are raised; central conflict pivots.',
  },
  {
    beat_id: 'beat-4',
    beat_type: 'Climax',
    scene_ids: [10],
    description: 'Final narrative resolution and story climax.',
  },
];

export const BeatSheetBoard: React.FC<BeatSheetBoardProps> = ({ beats = defaultBeats }) => {
  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Layers className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">3-Act Story Beat Sheet Board</h2>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
        {beats.map((beat) => (
          <div
            key={beat.beat_id}
            className="bg-surface/80 border border-border/60 rounded-lg p-3 hover:border-primary/50 transition-colors"
          >
            <span className="text-[10px] font-semibold uppercase tracking-wider text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded">
              {beat.beat_type}
            </span>
            <p className="text-xs text-slate-300 mt-2 line-clamp-3">{beat.description}</p>
            <div className="mt-3 text-[10px] text-slate-400 flex justify-between border-t border-border/40 pt-2">
              <span>Scenes: #{beat.scene_ids.join(', #')}</span>
              <span className="font-mono">{beat.beat_id}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
