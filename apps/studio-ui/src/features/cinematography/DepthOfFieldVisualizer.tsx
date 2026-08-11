import React from 'react';
import { Eye } from 'lucide-react';
import { DepthOfFieldView } from './types';

interface DepthOfFieldVisualizerProps {
  dof?: DepthOfFieldView;
  lang?: 'ta-IN' | 'en-US';
}

export const DepthOfFieldVisualizer: React.FC<DepthOfFieldVisualizerProps> = ({
  dof = {
    near_focus_limit_m: 2.1,
    far_focus_limit_m: 3.1,
    hyperfocal_distance_m: 12.4,
    bokeh_blur_factor: 0.85,
  },
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'புலத்தின் ஆழம் (Depth of Field)' : 'Depth of Field Range';

  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Eye className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="p-4 bg-surface/80 border border-border/60 rounded-lg flex items-center justify-between">
        <div>
          <div className="text-xs font-semibold text-slate-200">Focus Range Limits</div>
          <div className="text-[11px] font-mono text-slate-400 mt-1">
            Near: {dof.near_focus_limit_m}m • Far: {dof.far_focus_limit_m}m
          </div>
        </div>
        <div className="text-xs font-mono text-purple-400 bg-purple-500/10 px-3 py-1 rounded border border-purple-500/20">
          Bokeh Blur: {Math.round(dof.bokeh_blur_factor * 100)}%
        </div>
      </div>
    </div>
  );
};
