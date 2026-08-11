import React, { useState } from 'react';
import { Mic, Play, Volume2 } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { ActorSynthesizePayload, VisemeFrameView } from './types';

interface ActorVoiceSelectorProps {
  onSynthesize?: (visemes: VisemeFrameView[]) => void;
}

export const ActorVoiceSelector: React.FC<ActorVoiceSelectorProps> = ({ onSynthesize }) => {
  const [selectedVoiceId, setSelectedVoiceId] = useState('voice-elevenlabs-v1');
  const [dialogueText, setDialogueText] = useState('Siragugal Film Studio performance engine is ready.');
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleSynthesizeSpeech = async () => {
    const response = await StudioIpcService.executeEngineCommand<ActorSynthesizePayload, VisemeFrameView[]>(
      'actor_synthesize_speech',
      {
        character_id: 'char-vikram-101',
        voice_model_id: selectedVoiceId,
        dialogue_text: dialogueText,
        emotional_tone: 'Dramatic',
      }
    );

    if (response.success) {
      setStatusMessage(`Speech synthesized via ${selectedVoiceId}`);
      if (onSynthesize) {
        onSynthesize([
          { frame_index: 0, timecode_ms: 0, viseme_code: 'sil', weight: 1.0 },
          { frame_index: 2, timecode_ms: 83, viseme_code: 's', weight: 0.85 },
          { frame_index: 4, timecode_ms: 166, viseme_code: 'i', weight: 0.92 },
          { frame_index: 6, timecode_ms: 250, viseme_code: 'r', weight: 0.78 },
          { frame_index: 8, timecode_ms: 333, viseme_code: 'a', weight: 0.95 },
          { frame_index: 10, timecode_ms: 416, viseme_code: 'k', weight: 0.80 },
          { frame_index: 12, timecode_ms: 500, viseme_code: 'sil', weight: 1.0 },
        ]);
      }
    }
  };

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Mic className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">Actor Voice Model Selector</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div>
          <label className="block text-xs font-medium text-slate-300 mb-1 flex items-center gap-1">
            <Volume2 className="w-3 h-3 text-blue-400" /> Voice Model Reference (VoiceModelId)
          </label>
          <select
            value={selectedVoiceId}
            onChange={(e) => setSelectedVoiceId(e.target.value)}
            className="w-full bg-surface border border-border/60 rounded-lg px-3 py-2 text-xs text-slate-200 font-mono focus:outline-none focus:ring-2 focus:ring-primary"
          >
            <option value="voice-elevenlabs-v1">voice-elevenlabs-v1 (Neural Studio Voice)</option>
            <option value="voice-xtts-v2">voice-xtts-v2 (Zero-Shot Clone)</option>
            <option value="voice-coqui-v3">voice-coqui-v3 (Multilingual Voice)</option>
          </select>
        </div>

        <div>
          <label className="block text-xs font-medium text-slate-300 mb-1">Dialogue Text Prompt</label>
          <textarea
            value={dialogueText}
            onChange={(e) => setDialogueText(e.target.value)}
            className="w-full h-32 bg-surface border border-border/60 rounded-lg p-3 font-mono text-xs text-slate-200 focus:outline-none focus:ring-2 focus:ring-primary resize-none"
            placeholder="Enter dialogue line to synthesize..."
          />
        </div>

        <button
          onClick={handleSynthesizeSpeech}
          className="w-full flex items-center justify-center gap-1.5 py-2.5 px-4 bg-emerald-600 hover:bg-emerald-700 text-white font-medium text-xs rounded-lg transition-colors shadow-lg shadow-emerald-500/20"
        >
          <Play className="w-4 h-4" /> Synthesize & Extract Visemes (IPC)
        </button>

        {statusMessage && (
          <div className="p-2.5 bg-emerald-500/10 border border-emerald-500/20 rounded-lg text-[11px] text-emerald-400">
            {statusMessage}
          </div>
        )}
      </div>
    </div>
  );
};
