import React, { useState } from 'react';
import { Sliders } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { SceneTransformView } from './types';

interface TransformInspectorProps {
  nodeId?: string;
  initialTransform?: SceneTransformView;
  lang?: 'ta-IN' | 'en-US';
}

export const TransformInspector: React.FC<TransformInspectorProps> = ({
  nodeId = 'node-cam-main-01',
  initialTransform = { position: [0, 1.6, 5], rotation: [0, 0, 0], scale: [1, 1, 1] },
  lang = 'ta-IN',
}) => {
  const [transform, setTransform] = useState<SceneTransformView>(initialTransform);
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleUpdatePosition = async (axisIndex: number, val: number) => {
    const newPos = [...transform.position] as [number, number, number];
    newPos[axisIndex] = val;
    const newTransform = { ...transform, position: newPos };
    setTransform(newTransform);

    // Emits machine-readable IPC command cleanly
    const response = await StudioIpcService.executeEngineCommand('scene_update_transform', {
      node_id: nodeId,
      transform: newTransform,
    });

    if (response.success) {
      setStatusMessage(lang === 'ta-IN' ? 'மாற்றம் புதுப்பிக்கப்பட்டது' : 'Transform Updated via IPC');
    }
  };

  const titleText = lang === 'ta-IN' ? 'மாற்றக் கட்டுப்பாடுகள் (Transform Controls)' : 'Transform Controls';
  const posLabel = lang === 'ta-IN' ? 'நிலை (Position)' : 'Position';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Sliders className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div className="text-xs font-mono text-slate-400 bg-surface px-2.5 py-1 rounded border border-border">
          Active Node: {nodeId}
        </div>

        <div>
          <label className="block text-xs font-medium text-slate-300 mb-2">{posLabel}</label>
          {['X', 'Y', 'Z'].map((axis, idx) => (
            <div key={axis} className="flex items-center gap-2 mb-2">
              <span className="w-4 text-xs font-mono text-slate-400">{axis}:</span>
              <input
                type="number"
                step="0.1"
                value={transform.position[idx]}
                onChange={(e) => handleUpdatePosition(idx, parseFloat(e.target.value) || 0)}
                className="w-full bg-surface border border-border/60 rounded px-2 py-1 text-xs font-mono text-slate-200 focus:outline-none focus:ring-1 focus:ring-primary"
              />
            </div>
          ))}
        </div>

        {statusMessage && (
          <div className="p-2.5 bg-purple-500/10 border border-purple-500/20 rounded-lg text-[11px] text-purple-400 font-mono">
            {statusMessage}
          </div>
        )}
      </div>
    </div>
  );
};
