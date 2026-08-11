import React, { useState } from 'react';
import { Sliders, Volume2, VolumeX } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';

interface AudioMixerPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const AudioMixerPanel: React.FC<AudioMixerPanelProps> = ({ lang = 'ta-IN' }) => {
  const [volumeDb, setVolumeDb] = useState(0.0);
  const [pan, setPan] = useState(0.0);
  const [isMuted, setIsMuted] = useState(false);
  const [isSolo, setIsSolo] = useState(false);

  const handleUpdateMixer = async (newVol: number, newPan: number, muted: boolean, solo: boolean) => {
    setVolumeDb(newVol);
    setPan(newPan);
    setIsMuted(muted);
    setIsSolo(solo);

    await StudioIpcService.executeEngineCommand('audio_update_track', {
      track_id: 'track-dialogue-01',
      volume_db: newVol,
      pan: newPan,
      is_muted: muted,
      is_solo: solo,
    });
  };

  const titleText = lang === 'ta-IN' ? 'ஒலி கலவை (Audio Mixer)' : 'Audio Channel Mixer';
  const volLabel = lang === 'ta-IN' ? 'ஒலி அளவு (Volume)' : 'Volume (dB)';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Sliders className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>{volLabel}</span>
            <span className="font-mono text-emerald-400">{volumeDb > 0 ? `+${volumeDb}` : volumeDb} dB</span>
          </div>
          <input
            type="range"
            min={-60}
            max={6}
            step={0.5}
            value={volumeDb}
            aria-label={lang === 'ta-IN' ? 'ஒலி அளவு' : 'Volume'}
            onChange={(e) => handleUpdateMixer(parseFloat(e.target.value), pan, isMuted, isSolo)}
            className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-emerald-400"
          />
        </div>

        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>Pan (L/R)</span>
            <span className="font-mono text-slate-200">{pan === 0 ? 'Center' : pan < 0 ? `L ${Math.abs(pan)}` : `R ${pan}`}</span>
          </div>
          <input
            type="range"
            min={-1.0}
            max={1.0}
            step={0.1}
            value={pan}
            aria-label="Pan"
            onChange={(e) => handleUpdateMixer(volumeDb, parseFloat(e.target.value), isMuted, isSolo)}
            className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-emerald-400"
          />
        </div>

        <div className="flex gap-2 pt-2">
          <button
            onClick={() => handleUpdateMixer(volumeDb, pan, !isMuted, isSolo)}
            aria-label={lang === 'ta-IN' ? 'மௌனம்' : 'Mute'}
            className={`flex-1 py-2 px-3 text-xs font-semibold rounded-lg border transition-colors flex items-center justify-center gap-1.5 ${
              isMuted
                ? 'bg-red-500/20 text-red-400 border-red-500/40'
                : 'bg-surface text-slate-300 border-border/60 hover:bg-surface-hover'
            }`}
          >
            {isMuted ? <VolumeX className="w-3.5 h-3.5" /> : <Volume2 className="w-3.5 h-3.5" />}
            {lang === 'ta-IN' ? 'மௌனம்' : 'Mute'}
          </button>

          <button
            onClick={() => handleUpdateMixer(volumeDb, pan, isMuted, !isSolo)}
            aria-label={lang === 'ta-IN' ? 'தனி ஒலி' : 'Solo'}
            className={`flex-1 py-2 px-3 text-xs font-semibold rounded-lg border transition-colors flex items-center justify-center gap-1.5 ${
              isSolo
                ? 'bg-amber-500/20 text-amber-400 border-amber-500/40'
                : 'bg-surface text-slate-300 border-border/60 hover:bg-surface-hover'
            }`}
          >
            {lang === 'ta-IN' ? 'தனி ஒலி' : 'Solo'}
          </button>
        </div>
      </div>
    </div>
  );
};
