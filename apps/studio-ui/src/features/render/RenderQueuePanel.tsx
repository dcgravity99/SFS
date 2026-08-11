import React from 'react';
import { Layers, Plus } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { RenderJobCard } from './RenderJobCard';
import { RenderJobView } from './types';

interface RenderQueuePanelProps {
  jobs?: RenderJobView[];
  lang?: 'ta-IN' | 'en-US';
}

const defaultJobs: RenderJobView[] = [
  {
    job_id: 'job-render-scene1-master',
    display_name: { 'ta-IN': 'காட்சி 1 முதன்மை ரெண்டர்', 'en-US': 'Scene 1 Master Render 4K' },
    priority: 'High',
    status: 'Rendering',
    current_frame: 142,
    total_frames: 360,
    output_format: 'EXR Sequence 16-bit',
    eta_seconds: 48,
  },
  {
    job_id: 'job-render-scene2-shot1',
    display_name: { 'ta-IN': 'காட்சி 2 கோணம் 1 ரெண்டர்', 'en-US': 'Scene 2 Shot 1 Render' },
    priority: 'Normal',
    status: 'Queued',
    current_frame: 0,
    total_frames: 240,
    output_format: 'ProRes 4444 XQ',
    eta_seconds: 120,
  },
];

export const RenderQueuePanel: React.FC<RenderQueuePanelProps> = ({
  jobs = defaultJobs,
  lang = 'ta-IN',
}) => {
  const handleSubmitJob = async () => {
    await StudioIpcService.executeEngineCommand('render_submit_job', {
      project_id: 'prj-siragugal-01',
      output_format: 'ProRes 4444 XQ',
      priority: 'High',
    });
  };

  const titleText = lang === 'ta-IN' ? 'ரெண்டர் வரிசை (Render Queue)' : 'Batch Render Queue';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Layers className="w-4 h-4 text-purple-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <button
          onClick={handleSubmitJob}
          className="inline-flex items-center gap-1 px-2.5 py-1 text-xs font-medium bg-purple-600 hover:bg-purple-500 text-white rounded-lg transition-colors"
        >
          <Plus className="w-3.5 h-3.5" /> {lang === 'ta-IN' ? 'சேர்' : 'Add'}
        </button>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {jobs.map((j) => (
          <RenderJobCard key={j.job_id} job={j} lang={lang} />
        ))}
      </div>
    </div>
  );
};
