import React from 'react';
import { Sliders } from 'lucide-react';
import { VisemeFrameView } from './types';

interface VisemeTimelineVisualizerProps {
  visemes?: VisemeFrameView[];
}

const defaultVisemes: VisemeFrameView[] = [
  { frame_index: 0, timecode_ms: 0, viseme_code: 'sil', weight: 1.0 },
  { frame_index: 2, timecode_ms: 83, viseme_code: 's', weight: 0.85 },
  { frame_index: 4, timecode_ms: 166, viseme_code: 'i', weight: 0.92 },
  { frame_index: 6, timecode_ms: 250, viseme_code: 'r', weight: 0.78 },
  { frame_index: 8, timecode_ms: 333, viseme_code: 'a', weight: 0.95 },
  { frame_index: 10, timecode_ms: 416, viseme_code: 'k', weight: 0.80 },
  { frame_index: 12, timecode_ms: 500, viseme_code: 'sil', weight: 1.0 },
];

export const VisemeTimelineVisualizer: React.FC<VisemeTimelineVisualizerProps> = ({
  visemes = defaultVisemes,
}) => {
  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Sliders className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">Viseme Lip-Sync Track Visualizer</h2>
      </div>

      <div className="flex items-center gap-3 overflow-x-auto pb-2">
        {visemes.map((v) => (
          <div
            key={`${v.frame_index}-${v.viseme_code}`}
            className="flex-shrink-0 bg-surface/80 border border-border/60 rounded-lg p-3 min-w-[90px] text-center hover:border-purple-500/50 transition-colors"
            tabIndex={0}
            role="region"
            aria-label={`Viseme ${v.viseme_code} at frame ${v.frame_index}, timecode ${v.timecode_ms} milliseconds`}
          >
            <span className="text-xs font-mono font-bold text-purple-400 bg-purple-500/10 px-2 py-0.5 rounded border border-purple-500/20">
              {v.viseme_code}
            </span>
            <div className="mt-2 text-[10px] text-slate-300 font-mono">F#{v.frame_index}</div>
            <div className="text-[10px] text-slate-400 font-mono">{v.timecode_ms}ms</div>
            <div className="mt-2 w-full bg-surface-hover rounded-full h-1 overflow-hidden">
              <div
                className="bg-purple-400 h-full rounded-full"
                style={{ width: `${v.weight * 100}%` }}
              />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
