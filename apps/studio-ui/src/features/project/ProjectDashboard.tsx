import React from 'react';
import { LayoutDashboard, Film, Video, Folder, CheckCircle } from 'lucide-react';

interface ProjectDashboardProps {
  sceneCount?: number;
  shotCount?: number;
  assetCount?: number;
  progressPercent?: number;
  lang?: 'ta-IN' | 'en-US';
}

export const ProjectDashboard: React.FC<ProjectDashboardProps> = ({
  sceneCount = 12,
  shotCount = 48,
  assetCount = 128,
  progressPercent = 65,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'திட்டத் தகவல் குழு (Dashboard)' : 'Film Production Dashboard';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <LayoutDashboard className="w-4 h-4 text-purple-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="grid grid-cols-2 gap-3 mb-4">
        <div className="bg-surface/80 border border-border/60 rounded-lg p-3">
          <span className="text-[10px] font-semibold text-slate-400 flex items-center gap-1">
            <Film className="w-3 h-3 text-blue-400" /> {lang === 'ta-IN' ? 'காட்சிகள்' : 'Scenes'}
          </span>
          <div className="text-xl font-bold font-mono text-slate-100 mt-1">{sceneCount}</div>
        </div>

        <div className="bg-surface/80 border border-border/60 rounded-lg p-3">
          <span className="text-[10px] font-semibold text-slate-400 flex items-center gap-1">
            <Video className="w-3 h-3 text-emerald-400" /> {lang === 'ta-IN' ? 'கோணங்கள்' : 'Shots'}
          </span>
          <div className="text-xl font-bold font-mono text-slate-100 mt-1">{shotCount}</div>
        </div>

        <div className="bg-surface/80 border border-border/60 rounded-lg p-3">
          <span className="text-[10px] font-semibold text-slate-400 flex items-center gap-1">
            <Folder className="w-3 h-3 text-amber-400" /> Assets
          </span>
          <div className="text-xl font-bold font-mono text-slate-100 mt-1">{assetCount}</div>
        </div>

        <div className="bg-surface/80 border border-border/60 rounded-lg p-3">
          <span className="text-[10px] font-semibold text-slate-400 flex items-center gap-1">
            <CheckCircle className="w-3 h-3 text-purple-400" /> Progress
          </span>
          <div className="text-xl font-bold font-mono text-purple-400 mt-1">{progressPercent}%</div>
        </div>
      </div>
    </div>
  );
};
