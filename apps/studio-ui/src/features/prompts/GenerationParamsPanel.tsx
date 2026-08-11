import React, { useState } from 'react';
import { Sliders, Send } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';

interface GenerationParamsPanelProps {
  lang?: 'ta-IN' | 'en-US';
}

export const GenerationParamsPanel: React.FC<GenerationParamsPanelProps> = ({ lang = 'ta-IN' }) => {
  const [cfgScale, setCfgScale] = useState(7.5);
  const [steps, setSteps] = useState(30);
  const [seed] = useState(-1);
  const [sampler, setSampler] = useState('DPM++ 2M Karras');
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleSubmitJob = async () => {
    const response = await StudioIpcService.executeEngineCommand('ai_submit_generation_job', {
      prompt_id: 'prompt-gen-01',
      positive_prompt: 'Cinematic lead protagonist in rainy alleyway',
      negative_prompt: 'blurry, low quality',
      model_id: 'model-sira-sdxl-v1',
      generation_parameters: {
        cfg_scale: cfgScale,
        steps,
        seed: seed === -1 ? Math.floor(Math.random() * 1000000) : seed,
        sampler_name: sampler,
      },
    });

    if (response.success) {
      setStatusMessage(lang === 'ta-IN' ? 'உருவாக்கப் பணி அனுப்பப்பட்டது (Job Submitted)' : 'AI Generation Job Submitted via IPC');
    }
  };

  const titleText = lang === 'ta-IN' ? 'அமைப்புகள் (Generation Params)' : 'Generation Parameters';
  const cfgLabel = lang === 'ta-IN' ? 'வழிகாட்டுதல் அளவு (CFG Scale)' : 'CFG Scale';
  const stepsLabel = lang === 'ta-IN' ? 'படிநிலைகள் (Steps)' : 'Sampling Steps';
  const submitText = lang === 'ta-IN' ? 'உருவாக்கத் தொடங்கு' : 'Submit Generation Job';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Sliders className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-4 flex-1">
        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>{cfgLabel}</span>
            <span className="font-mono text-purple-400">{cfgScale}</span>
          </div>
          <input
            type="range"
            min={1.0}
            max={20.0}
            step={0.5}
            value={cfgScale}
            aria-label={cfgLabel}
            onChange={(e) => setCfgScale(parseFloat(e.target.value))}
            className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-purple-400"
          />
        </div>

        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>{stepsLabel}</span>
            <span className="font-mono text-slate-200">{steps}</span>
          </div>
          <input
            type="range"
            min={10}
            max={150}
            step={5}
            value={steps}
            aria-label={stepsLabel}
            onChange={(e) => setSteps(parseInt(e.target.value, 10))}
            className="w-full h-1.5 bg-surface-hover rounded-lg appearance-none cursor-pointer accent-purple-400"
          />
        </div>

        <div>
          <div className="flex justify-between text-xs text-slate-300 mb-1">
            <span>Sampler Algorithm</span>
          </div>
          <select
            value={sampler}
            onChange={(e) => setSampler(e.target.value)}
            className="w-full bg-surface border border-border/60 rounded p-2 font-mono text-xs text-slate-200"
          >
            <option value="DPM++ 2M Karras">DPM++ 2M Karras</option>
            <option value="Euler A">Euler A</option>
            <option value="DDIM">DDIM</option>
          </select>
        </div>

        <button
          onClick={handleSubmitJob}
          aria-label={submitText}
          className="w-full flex items-center justify-center gap-2 py-2.5 px-4 bg-purple-600 hover:bg-purple-500 text-white font-medium text-xs rounded-lg transition-colors shadow-lg shadow-purple-500/20 mt-4"
        >
          <Send className="w-4 h-4" />
          {submitText}
        </button>

        {statusMessage && (
          <div className="p-2.5 bg-purple-500/10 border border-purple-500/20 rounded-lg text-[11px] text-purple-300 font-mono">
            {statusMessage}
          </div>
        )}
      </div>
    </div>
  );
};
