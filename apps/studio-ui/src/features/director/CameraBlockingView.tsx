import React from 'react';
import { Compass } from 'lucide-react';
import { CameraBlockingViewModel } from './types';

interface CameraBlockingViewProps {
  model?: CameraBlockingViewModel;
  lang?: 'ta-IN' | 'en-US';
}

export const CameraBlockingView: React.FC<CameraBlockingViewProps> = ({
  model = {
    camera_id: 'cam-rig-01',
    track_position: [0, 1.5, 4],
    target_character_id: 'char-vikram-101',
  },
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'கேமரா தளம் (Camera Blocking)' : 'Camera Blocking Diagram';

  return (
    <div className="glass-card rounded-xl p-4">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Compass className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="p-4 bg-surface/80 border border-border/60 rounded-lg flex items-center justify-between">
        <div>
          <div className="text-xs font-semibold text-slate-200">Camera Rig: {model.camera_id}</div>
          <div className="text-[11px] font-mono text-slate-400 mt-1">
            Tracking Target: {model.target_character_id}
          </div>
        </div>
        <div className="text-xs font-mono text-emerald-400 bg-emerald-500/10 px-3 py-1 rounded border border-emerald-500/20">
          Pos: [{model.track_position.join(', ')}]
        </div>
      </div>
    </div>
  );
};
