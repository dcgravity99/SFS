import React from 'react';
import { Camera, Link } from 'lucide-react';
import { LensProfileView } from './types';

interface LensProfilePanelProps {
  lenses?: LensProfileView[];
  selectedLensId?: string;
  onSelectLens?: (lensId: string) => void;
  lang?: 'ta-IN' | 'en-US';
}

const defaultLenses: LensProfileView[] = [
  {
    lens_profile_id: 'lens-anamorphic-35mm',
    display_name: { 'ta-IN': 'அனாமார்பிக் 35mm லென்ஸ்', 'en-US': 'Anamorphic 35mm Prime' },
    focal_length_mm: 35,
    max_aperture: 1.8,
    squeeze_factor: 2.0,
  },
  {
    lens_profile_id: 'lens-spherical-50mm',
    display_name: { 'ta-IN': 'கோள 50mm லென்ஸ்', 'en-US': 'Spherical 50mm Prime' },
    focal_length_mm: 50,
    max_aperture: 1.4,
    squeeze_factor: 1.0,
  },
];

export const LensProfilePanel: React.FC<LensProfilePanelProps> = ({
  lenses = defaultLenses,
  selectedLensId = 'lens-anamorphic-35mm',
  onSelectLens,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'லென்ஸ் சுயவிவரம் (Lens Profile)' : 'Lens Profile Management';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Camera className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {lenses.map((l) => {
          const name = l.display_name[lang] || l.display_name['en-US'];
          const isSelected = selectedLensId === l.lens_profile_id;
          return (
            <div
              key={l.lens_profile_id}
              onClick={() => onSelectLens && onSelectLens(l.lens_profile_id)}
              className={`p-3 rounded-lg border transition-all cursor-pointer ${
                isSelected
                  ? 'bg-purple-500/10 border-purple-500 text-white shadow-sm'
                  : 'bg-surface/80 border-border/60 text-slate-300 hover:border-border'
              }`}
            >
              <div className="flex items-center justify-between">
                <span className="text-xs font-semibold">{name}</span>
                <span className="text-[10px] font-mono text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded">
                  {l.squeeze_factor > 1.0 ? '2.0x Anamorphic' : '1.0x Spherical'}
                </span>
              </div>
              <div className="mt-2 text-[10px] font-mono text-slate-400 flex items-center justify-between">
                <span className="flex items-center gap-1">
                  <Link className="w-3 h-3 text-purple-400" /> {l.lens_profile_id}
                </span>
                <span>f/{l.max_aperture} Max</span>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
