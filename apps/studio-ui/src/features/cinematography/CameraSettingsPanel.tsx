import React, { useState } from 'react';
import { Sliders } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { CameraSettingsView } from './types';

interface CameraSettingsPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const CameraSettingsPanel: React.FC<CameraSettingsPanelProps> = ({ lang = 'ta-IN' }) => {
  const [settings, setSettings] = useState<CameraSettingsView>({
    camera_id: 'cam-main-01',
    lens_profile_id: 'lens-anamorphic-35mm',
    focal_length_mm: 35,
    aperture_fstop: 1.8,
    focus_distance_m: 2.5,
    shutter_angle_deg: 180,
    iso_rating: 800,
  });

  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleUpdateFStop = async (val: number) => {
    const newSettings = { ...settings, aperture_fstop: val };
    setSettings(newSettings);

    const response = await StudioIpcService.executeEngineCommand('cinematography_update_camera', newSettings);
    if (response.success) {
      setStatusMessage(lang === 'ta-IN' ? 'கேமரா புதுப்பிக்கப்பட்டது' : 'Camera Settings Updated via IPC');
    }
  };

  const titleText = lang === 'ta-IN' ? 'கேமரா அமைப்புகள் (Camera Settings)' : 'Camera Parameters';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Sliders className="w-4 h-4 text-blue-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>Aperture (f-stop)</span>
            <span className="font-mono text-blue-400">f/{settings.aperture_fstop}</span>
          </div>
          <div className="grid grid-cols-3 gap-2">
            {[1.4, 1.8, 2.8, 4.0, 5.6, 8.0].map((f) => (
              <button
                key={f}
                onClick={() => handleUpdateFStop(f)}
                className={`py-1.5 px-2 text-xs font-mono rounded border transition-colors ${
                  settings.aperture_fstop === f
                    ? 'bg-primary text-white border-primary'
                    : 'bg-surface text-slate-300 border-border/60 hover:bg-surface-hover'
                }`}
              >
                f/{f}
              </button>
            ))}
          </div>
        </div>

        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>Focus Distance</span>
            <span className="font-mono text-slate-200">{settings.focus_distance_m} m</span>
          </div>
          <input
            type="range"
            min={0.5}
            max={20.0}
            step={0.5}
            value={settings.focus_distance_m}
            onChange={(e) => setSettings({ ...settings, focus_distance_m: parseFloat(e.target.value) })}
            className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-blue-400"
          />
        </div>

        {statusMessage && (
          <div className="p-2.5 bg-blue-500/10 border border-blue-500/20 rounded-lg text-[11px] text-blue-400 font-mono">
            {statusMessage}
          </div>
        )}
      </div>
    </div>
  );
};
