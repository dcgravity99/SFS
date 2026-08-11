import React, { useState } from 'react';
import { Smile } from 'lucide-react';
import { ActorPerformanceConfig } from './types';

export const EmotionalTonePanel: React.FC = () => {
  const [config, setConfig] = useState<ActorPerformanceConfig>({
    character_id: 'char-vikram-101',
    voice_model_id: 'voice-elevenlabs-v1',
    emotional_tone: 'Dramatic',
    pitch_shift: 0,
    speech_rate: 1.0,
  });

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Smile className="w-4 h-4 text-amber-400" />
        <h2 className="text-sm font-semibold text-slate-200">Emotional Tone & Dialect Configurator</h2>
      </div>

      <div className="space-y-5 flex-1">
        <div>
          <label className="block text-xs font-medium text-slate-300 mb-2">Emotional Preset</label>
          <div className="grid grid-cols-2 gap-2">
            {(['Neutral', 'Dramatic', 'Angry', 'Melancholic'] as const).map((tone) => (
              <button
                key={tone}
                onClick={() => setConfig({ ...config, emotional_tone: tone })}
                className={`py-2 px-3 text-xs font-medium rounded-lg border transition-all ${
                  config.emotional_tone === tone
                    ? 'bg-amber-500/10 text-amber-400 border-amber-500/40 shadow-sm'
                    : 'bg-surface text-slate-400 border-border/60 hover:text-slate-200'
                }`}
              >
                {tone}
              </button>
            ))}
          </div>
        </div>

        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>Pitch Shift</span>
            <span className="font-mono text-slate-200">{config.pitch_shift} semitones</span>
          </div>
          <input
            type="range"
            min={-12}
            max={12}
            value={config.pitch_shift}
            onChange={(e) => setConfig({ ...config, pitch_shift: parseInt(e.target.value) })}
            className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-amber-400"
          />
        </div>

        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>Speech Rate / Tempo</span>
            <span className="font-mono text-slate-200">{config.speech_rate}x</span>
          </div>
          <input
            type="range"
            min={0.5}
            max={2.0}
            step={0.1}
            value={config.speech_rate}
            onChange={(e) => setConfig({ ...config, speech_rate: parseFloat(e.target.value) })}
            className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-amber-400"
          />
        </div>

        <div className="p-3 bg-surface/60 border border-border/40 rounded-lg text-xs text-slate-400 space-y-1">
          <div className="font-semibold text-slate-200 mb-1">Performance Parameters</div>
          <div className="flex justify-between">
            <span>Tone Preset:</span>
            <span className="text-amber-400 font-medium">{config.emotional_tone}</span>
          </div>
          <div className="flex justify-between">
            <span>Synthesis Status:</span>
            <span className="text-emerald-400 font-mono">Ready</span>
          </div>
        </div>
      </div>
    </div>
  );
};
