import React from 'react';
import {
  BookOpen,
  User,
  Mic,
  Box,
  Video,
  Camera,
  Music,
  Sliders,
  Cpu,
  Folder,
  Settings,
} from 'lucide-react';
import { useWorkspaceStore, WorkspaceMode } from '../../stores/workspace.store';

interface NavItem {
  id: WorkspaceMode;
  label: string;
  icon: React.ElementType;
}

const navItems: NavItem[] = [
  { id: 'story', label: 'Story Engine', icon: BookOpen },
  { id: 'character', label: 'Character Studio', icon: User },
  { id: 'actor', label: 'Actor Studio', icon: Mic },
  { id: 'scene', label: 'Scene Builder', icon: Box },
  { id: 'director', label: 'Director Plan', icon: Video },
  { id: 'cinematography', label: 'Cinematography', icon: Camera },
  { id: 'audio', label: 'Audio Mixer', icon: Music },
  { id: 'timeline', label: 'NLE Timeline', icon: Sliders },
  { id: 'render', label: 'Render Queue', icon: Cpu },
  { id: 'assets', label: 'Asset Browser', icon: Folder },
  { id: 'settings', label: 'Studio Settings', icon: Settings },
];

export const Sidebar: React.FC = () => {
  const { activeMode, setMode } = useWorkspaceStore();

  return (
    <aside className="w-56 glass-panel border-r border-border p-3 flex flex-col gap-1 select-none">
      <div className="text-xs font-semibold text-slate-400 uppercase tracking-wider px-3 py-2">
        Sub-Engines
      </div>
      {navItems.map((item) => {
        const Icon = item.icon;
        const isActive = activeMode === item.id;
        return (
          <button
            key={item.id}
            onClick={() => setMode(item.id)}
            className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs font-medium transition-all ${
              isActive
                ? 'bg-primary text-white shadow-md shadow-blue-500/20'
                : 'text-slate-400 hover:text-slate-200 hover:bg-surface-hover'
            }`}
          >
            <Icon className="w-4 h-4" />
            {item.label}
          </button>
        );
      })}
    </aside>
  );
};
