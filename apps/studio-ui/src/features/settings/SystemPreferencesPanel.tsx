import React, { useState } from 'react';
import { Sliders, Save } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { StudioPreferencesView } from './types';

interface SystemPreferencesPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const SystemPreferencesPanel: React.FC<SystemPreferencesPanelProps> = ({ lang = 'ta-IN' }) => {
  const [preferences, setPreferences] = useState<StudioPreferencesView>({
    primary_locale: 'ta-IN',
    theme_mode: 'Dark',
    auto_save_interval_mins: 5,
    undo_history_depth: 100,
    gpu_acceleration_enabled: true,
    vram_limit_mb: 16384,
  });

  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleUpdatePreferences = async () => {
    const response = await StudioIpcService.executeEngineCommand('settings_update_config', preferences);
    if (response.success) {
      setStatusMessage(lang === 'ta-IN' ? 'விருப்பங்கள் புதுப்பிக்கப்பட்டன' : 'Studio Preferences Saved via IPC');
    }
  };

  const titleText = lang === 'ta-IN' ? 'பொது விருப்பங்கள் (System Preferences)' : 'Global System Preferences';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Sliders className="w-4 h-4 text-purple-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <button
          onClick={handleUpdatePreferences}
          className="p-1 text-slate-400 hover:text-slate-100 transition-colors"
        >
          <Save className="w-4 h-4" />
        </button>
      </div>

      <div className="space-y-4 flex-1">
        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>Auto-Save Interval (Minutes)</span>
            <span className="font-mono text-purple-400">{preferences.auto_save_interval_mins} min</span>
          </div>
          <input
            type="range"
            min={1}
            max={30}
            step={1}
            value={preferences.auto_save_interval_mins}
            onChange={(e) => setPreferences({ ...preferences, auto_save_interval_mins: parseInt(e.target.value, 10) })}
            className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-purple-400"
          />
        </div>

        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>Undo History Depth</span>
            <span className="font-mono text-slate-200">{preferences.undo_history_depth} steps</span>
          </div>
          <input
            type="range"
            min={50}
            max={500}
            step={25}
            value={preferences.undo_history_depth}
            onChange={(e) => setPreferences({ ...preferences, undo_history_depth: parseInt(e.target.value, 10) })}
            className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-purple-400"
          />
        </div>

        {statusMessage && (
          <div className="p-2.5 bg-purple-500/10 border border-purple-500/20 rounded-lg text-[11px] text-purple-300 font-mono">
            {statusMessage}
          </div>
        )}
      </div>
    </div>
  );
};
