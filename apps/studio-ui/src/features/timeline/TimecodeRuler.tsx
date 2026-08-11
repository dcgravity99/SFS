import React from 'react';
import { Clock } from 'lucide-react';
import { useTimelineStore } from '../../stores/timeline.store';

interface TimecodeRulerProps {
  lang?: 'ta-IN' | 'en-US';
}

export const TimecodeRuler: React.FC<TimecodeRulerProps> = ({ lang = 'ta-IN' }) => {
  const { currentFrame, fps } = useTimelineStore();

  const formatSMPTE = (frame: number, rate: number) => {
    const totalSecs = Math.floor(frame / rate);
    const framesLeft = frame % rate;
    const mins = Math.floor(totalSecs / 60);
    const secs = totalSecs % 60;
    const hrs = Math.floor(mins / 60);

    const pad = (n: number) => n.toString().padStart(2, '0');
    return `${pad(hrs)}:${pad(mins % 60)}:${pad(secs)}:${pad(framesLeft)}`;
  };

  const titleText = lang === 'ta-IN' ? 'நேரக் குறியீடு (SMPTE Timecode)' : 'SMPTE Timecode Ruler';

  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-3">
        <div className="flex items-center gap-2">
          <Clock className="w-4 h-4 text-blue-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-xs font-mono font-bold text-emerald-400 bg-emerald-500/10 px-3 py-1 rounded border border-emerald-500/20">
          {formatSMPTE(currentFrame, fps)} @ {fps}fps
        </span>
      </div>

      <div className="flex items-center gap-2 overflow-x-auto py-2">
        {[0, 24, 48, 72, 96, 120, 144, 168, 192, 216, 240].map((f) => (
          <div key={f} className="flex-shrink-0 text-center min-w-[70px]">
            <div className="text-[10px] font-mono text-slate-400">{formatSMPTE(f, fps)}</div>
            <div className="w-0.5 h-3 bg-border/80 mx-auto mt-1" />
          </div>
        ))}
      </div>
    </div>
  );
};
