import React, { useState } from 'react';
import { Sun } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { LightingProfileView } from './types';

interface LightingControlPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const LightingControlPanel: React.FC<LightingControlPanelProps> = ({ lang = 'ta-IN' }) => {
  const [lighting, setLighting] = useState<LightingProfileView>({
    key_light_intensity: 0.8,
    fill_light_intensity: 0.45,
    back_light_intensity: 0.6,
    color_temperature_kelvin: 5600,
  });

  const handleUpdateKelvin = async (val: number) => {
    const newLighting = { ...lighting, color_temperature_kelvin: val };
    setLighting(newLighting);

    await StudioIpcService.executeEngineCommand('cinematography_update_lighting', newLighting);
  };

  const titleText = lang === 'ta-IN' ? 'ஒளி அமைப்பு (Lighting Control)' : 'Three-Point Lighting Rig';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Sun className="w-4 h-4 text-amber-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>Color Temperature (Kelvin)</span>
            <span className="font-mono text-amber-400">{lighting.color_temperature_kelvin} K</span>
          </div>
          <div className="flex gap-2">
            <button
              onClick={() => handleUpdateKelvin(3200)}
              className={`flex-1 py-1.5 px-2 text-xs font-mono rounded border transition-colors ${
                lighting.color_temperature_kelvin === 3200
                  ? 'bg-amber-500/20 text-amber-300 border-amber-500/40'
                  : 'bg-surface text-slate-400 border-border/60'
              }`}
            >
              3200K Warm
            </button>
            <button
              onClick={() => handleUpdateKelvin(5600)}
              className={`flex-1 py-1.5 px-2 text-xs font-mono rounded border transition-colors ${
                lighting.color_temperature_kelvin === 5600
                  ? 'bg-blue-500/20 text-blue-300 border-blue-500/40'
                  : 'bg-surface text-slate-400 border-border/60'
              }`}
            >
              5600K Daylight
            </button>
          </div>
        </div>

        <div className="space-y-3 pt-2">
          <div>
            <div className="flex justify-between text-xs text-slate-300 mb-1">
              <span>Key Light</span>
              <span className="font-mono text-slate-200">{Math.round(lighting.key_light_intensity * 100)}%</span>
            </div>
            <input
              type="range"
              min={0}
              max={1}
              step={0.05}
              value={lighting.key_light_intensity}
              onChange={(e) => setLighting({ ...lighting, key_light_intensity: parseFloat(e.target.value) })}
              className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-amber-400"
            />
          </div>

          <div>
            <div className="flex justify-between text-xs text-slate-300 mb-1">
              <span>Fill Light</span>
              <span className="font-mono text-slate-200">{Math.round(lighting.fill_light_intensity * 100)}%</span>
            </div>
            <input
              type="range"
              min={0}
              max={1}
              step={0.05}
              value={lighting.fill_light_intensity}
              onChange={(e) => setLighting({ ...lighting, fill_light_intensity: parseFloat(e.target.value) })}
              className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-amber-400"
            />
          </div>
        </div>
      </div>
    </div>
  );
};
