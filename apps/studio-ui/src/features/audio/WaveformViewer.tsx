import React from 'react';
import { Activity } from 'lucide-react';
import { WaveformSegmentView } from './types';

interface WaveformViewerProps {
  segment?: WaveformSegmentView;
  lang?: 'ta-IN' | 'en-US';
}

const defaultSegment: WaveformSegmentView = {
  segment_id: 'seg-wave-01',
  start_ms: 0,
  duration_ms: 5000,
  amplitude_peaks: [0.2, 0.4, 0.7, 0.9, 0.6, 0.8, 0.5, 0.3, 0.6, 0.9, 0.4, 0.2],
};

export const WaveformViewer: React.FC<WaveformViewerProps> = ({
  segment = defaultSegment,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'ஒலை அலை வடிவம் (Waveform Viewer)' : 'Waveform Visualizer';

  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Activity className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="p-4 bg-surface/80 border border-border/60 rounded-lg">
        <div className="flex items-end gap-1.5 h-16 w-full justify-between">
          {segment.amplitude_peaks.map((peak, idx) => (
            <div
              key={idx}
              className="bg-emerald-400/80 hover:bg-emerald-300 transition-colors flex-1 rounded-t"
              style={{ height: `${peak * 100}%` }}
            />
          ))}
        </div>
        <div className="mt-3 text-[10px] font-mono text-slate-400 flex justify-between border-t border-border/40 pt-2">
          <span>Start: {segment.start_ms}ms</span>
          <span>Duration: {segment.duration_ms}ms</span>
          <span>{segment.segment_id}</span>
        </div>
      </div>
    </div>
  );
};
