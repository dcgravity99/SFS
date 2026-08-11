import React from 'react';
import { Mic, Link } from 'lucide-react';
import { AudioTrackChannelView } from './types';

interface DialogueTrackPanelProps {
  tracks?: AudioTrackChannelView[];
  lang?: 'ta-IN' | 'en-US';
}

const defaultTracks: AudioTrackChannelView[] = [
  {
    track_id: 'track-dialogue-01',
    display_name: { 'ta-IN': 'கதாநாயகன் உரையாடல்', 'en-US': 'Lead Dialogue Stem' },
    track_type: 'Dialogue',
    volume_db: 0.0,
    pan: 0.0,
    is_muted: false,
    is_solo: false,
    asset_id: 'ast-audio-dialogue-v1',
  },
  {
    track_id: 'track-foley-02',
    display_name: { 'ta-IN': 'அடிச்சுவட்டு ஒலி (Foley)', 'en-US': 'Footsteps Foley' },
    track_type: 'Foley',
    volume_db: -4.5,
    pan: -0.2,
    is_muted: false,
    is_solo: false,
    asset_id: 'ast-audio-foley-steps',
  },
];

export const DialogueTrackPanel: React.FC<DialogueTrackPanelProps> = ({
  tracks = defaultTracks,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'உரையாடல் தடம் (Dialogue Track)' : 'Dialogue Track Manager';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Mic className="w-4 h-4 text-blue-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {tracks.map((t) => {
          const name = t.display_name[lang] || t.display_name['en-US'];
          return (
            <div key={t.track_id} className="bg-surface/80 border border-border/60 rounded-lg p-3">
              <div className="flex items-center justify-between">
                <span className="text-xs font-semibold text-slate-200">{name}</span>
                <span className="text-[10px] font-mono text-blue-400 bg-blue-500/10 px-2 py-0.5 rounded">
                  {t.track_type}
                </span>
              </div>
              <div className="mt-2 text-[10px] font-mono text-slate-400 flex items-center justify-between">
                <span className="flex items-center gap-1">
                  <Link className="w-3 h-3 text-purple-400" /> {t.asset_id}
                </span>
                <span>{t.volume_db} dB</span>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
