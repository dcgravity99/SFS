import React from 'react';
import { Video, Music, Type, Lock, Eye, Link } from 'lucide-react';
import { TimelineClipView, TimelineTrackHeaderView } from './types';

interface MultiTrackCanvasProps {
  tracks?: TimelineTrackHeaderView[];
  clips?: TimelineClipView[];
  lang?: 'ta-IN' | 'en-US';
}

const defaultTracks: TimelineTrackHeaderView[] = [
  { track_id: 'track-v1', track_name: { 'ta-IN': 'வீடியோ V1', 'en-US': 'Video V1' }, track_type: 'Video', is_locked: false, is_visible: true },
  { track_id: 'track-v2', track_name: { 'ta-IN': 'வீடியோ V2', 'en-US': 'Video V2' }, track_type: 'Video', is_locked: false, is_visible: true },
  { track_id: 'track-a1', track_name: { 'ta-IN': 'ஆடியோ A1', 'en-US': 'Audio A1' }, track_type: 'Audio', is_locked: false, is_visible: true },
  { track_id: 'track-t1', track_name: { 'ta-IN': 'துணைத்தலைப்பு T1', 'en-US': 'Subtitle T1' }, track_type: 'Subtitle', is_locked: false, is_visible: true },
];

const defaultClips: TimelineClipView[] = [
  { clip_id: 'clip-v1-01', track_id: 'track-v1', display_name: { 'ta-IN': 'காட்சி 1 - முதன்மை கோணம்', 'en-US': 'Scene 1 Main Shot' }, start_frame: 0, duration_frames: 120, in_point_frame: 0, out_point_frame: 120, asset_id: 'ast-video-scene1' },
  { clip_id: 'clip-a1-01', track_id: 'track-a1', display_name: { 'ta-IN': 'கதாநாயகன் உரையாடல்', 'en-US': 'Dialogue Stem' }, start_frame: 0, duration_frames: 96, in_point_frame: 0, out_point_frame: 96, asset_id: 'ast-audio-dialogue' },
];

export const MultiTrackCanvas: React.FC<MultiTrackCanvasProps> = ({
  tracks = defaultTracks,
  clips = defaultClips,
  lang = 'ta-IN',
}) => {
  const getTrackIcon = (type: string) => {
    switch (type) {
      case 'Video': return <Video className="w-3.5 h-3.5 text-blue-400" />;
      case 'Audio': return <Music className="w-3.5 h-3.5 text-emerald-400" />;
      case 'Subtitle': return <Type className="w-3.5 h-3.5 text-purple-400" />;
      default: return null;
    }
  };

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <h2 className="text-sm font-semibold text-slate-200">
          {lang === 'ta-IN' ? 'பல-தடங்கள் (Multi-Track Canvas)' : 'Multi-Track Timeline Canvas'}
        </h2>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {tracks.map((t) => {
          const trackClips = clips.filter((c) => c.track_id === t.track_id);
          const tName = t.track_name[lang] || t.track_name['en-US'];
          return (
            <div key={t.track_id} className="bg-surface/80 border border-border/60 rounded-lg p-3">
              <div className="flex items-center justify-between border-b border-border/40 pb-2 mb-2">
                <div className="flex items-center gap-2">
                  {getTrackIcon(t.track_type)}
                  <span className="text-xs font-semibold text-slate-200">{tName}</span>
                </div>
                <div className="flex items-center gap-2">
                  <Eye className="w-3.5 h-3.5 text-slate-400 hover:text-slate-200 cursor-pointer" />
                  <Lock className="w-3.5 h-3.5 text-slate-400 hover:text-slate-200 cursor-pointer" />
                </div>
              </div>

              <div className="flex items-center gap-2 overflow-x-auto min-h-[40px]">
                {trackClips.map((c) => {
                  const cName = c.display_name[lang] || c.display_name['en-US'];
                  return (
                    <div
                      key={c.clip_id}
                      className="bg-primary/20 border border-primary/40 rounded px-3 py-1.5 text-xs text-blue-200 flex flex-col min-w-[140px]"
                    >
                      <span className="font-semibold text-[11px] truncate">{cName}</span>
                      <span className="text-[9px] font-mono text-blue-400 flex items-center gap-1 mt-1">
                        <Link className="w-2.5 h-2.5" /> {c.asset_id}
                      </span>
                    </div>
                  );
                })}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
