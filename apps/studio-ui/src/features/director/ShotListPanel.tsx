import React from 'react';
import { Camera, Plus } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { ShotItemView } from './types';

interface ShotListPanelProps {
  shots?: ShotItemView[];
  lang?: 'ta-IN' | 'en-US';
}

const defaultShots: ShotItemView[] = [
  {
    shot_id: 'shot-01-wide',
    scene_id: 1,
    shot_number: '1A',
    shot_type: 'Establishing',
    focal_length_mm: 24,
    duration_frames: 120,
    director_notes: { 'ta-IN': 'ஸ்டுடியோ பரந்த பார்வை', 'en-US': 'Studio Wide Establishing' },
  },
  {
    shot_id: 'shot-02-medium',
    scene_id: 1,
    shot_number: '1B',
    shot_type: 'Medium',
    focal_length_mm: 50,
    duration_frames: 72,
    director_notes: { 'ta-IN': 'இயக்குனர் இடைநிலை கோணம்', 'en-US': 'Director Medium Framing' },
  },
  {
    shot_id: 'shot-03-closeup',
    scene_id: 1,
    shot_number: '1C',
    shot_type: 'CloseUp',
    focal_length_mm: 85,
    duration_frames: 48,
    director_notes: { 'ta-IN': 'கதாநாயகன் நெருக்கமான பார்வை', 'en-US': 'Lead Close-Up Emotional' },
  },
];

export const ShotListPanel: React.FC<ShotListPanelProps> = ({
  shots = defaultShots,
  lang = 'ta-IN',
}) => {
  const handleAddShot = async () => {
    await StudioIpcService.executeEngineCommand('director_create_shot', {
      scene_id: 1,
      shot_type: 'Medium',
      focal_length_mm: 50,
    });
  };

  const titleText = lang === 'ta-IN' ? 'காட்சிப் பட்டியல் (Shot List)' : 'Shot List Inventory';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Camera className="w-4 h-4 text-purple-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <button
          onClick={handleAddShot}
          className="inline-flex items-center gap-1 px-2.5 py-1 text-xs font-medium bg-primary hover:bg-blue-600 text-white rounded-lg transition-colors"
        >
          <Plus className="w-3.5 h-3.5" /> {lang === 'ta-IN' ? 'சேர்' : 'Add'}
        </button>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {shots.map((s) => {
          const notes = s.director_notes[lang] || s.director_notes['en-US'];
          return (
            <div key={s.shot_id} className="bg-surface/80 border border-border/60 rounded-lg p-3">
              <div className="flex items-center justify-between">
                <span className="text-xs font-bold text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded">
                  Shot #{s.shot_number} ({s.shot_type})
                </span>
                <span className="text-[10px] font-mono text-slate-400">{s.focal_length_mm}mm Lens</span>
              </div>
              <p className="text-xs text-slate-300 mt-2">{notes}</p>
              <div className="mt-2 text-[10px] font-mono text-slate-400 flex justify-between border-t border-border/40 pt-2">
                <span>Duration: {s.duration_frames} frames</span>
                <span>{s.shot_id}</span>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
