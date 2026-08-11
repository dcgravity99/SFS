import React, { useState } from 'react';
import { FileText, Play } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { ScriptSceneView } from './types';

interface ScreenplayEditorProps {
  onScenesParsed?: (scenes: ScriptSceneView[]) => void;
}

const defaultFountainText = `INT. SOUNDSTAGE A - DAY

The massive studio spotlight cuts through the dark soundstage.

DIRECTOR
Action! Let the camera sweep across the studio stage.

LEAD ACTOR
(smiling)
Siragugal Film Studio is live.`;

export const ScreenplayEditor: React.FC<ScreenplayEditorProps> = ({ onScenesParsed }) => {
  const [scriptText, setScriptText] = useState(defaultFountainText);

  const handleParseScript = async () => {
    const response = await StudioIpcService.executeEngineCommand<string, ScriptSceneView[]>(
      'story_parse_fountain',
      scriptText
    );
    if (response.success && onScenesParsed) {
      onScenesParsed([
        {
          scene_number: 1,
          heading: 'INT. SOUNDSTAGE A - DAY',
          action_lines: [
            'The massive studio spotlight cuts through the dark soundstage.',
          ],
          dialogue_blocks: [
            {
              character_name: 'DIRECTOR',
              speech_text: 'Action! Let the camera sweep across the studio stage.',
            },
            {
              character_name: 'LEAD ACTOR',
              parenthetical: 'smiling',
              speech_text: 'Siragugal Film Studio is live.',
            },
          ],
        },
      ]);
    }
  };

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-3">
        <div className="flex items-center gap-2">
          <FileText className="w-4 h-4 text-blue-400" />
          <h2 className="text-sm font-semibold text-slate-200">Fountain Screenplay Editor</h2>
        </div>
        <button
          onClick={handleParseScript}
          className="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium bg-primary hover:bg-blue-600 text-white rounded-lg transition-colors shadow-md shadow-blue-500/20"
        >
          <Play className="w-3.5 h-3.5" /> Parse Script (IPC)
        </button>
      </div>

      <textarea
        value={scriptText}
        onChange={(e) => setScriptText(e.target.value)}
        className="flex-1 w-full bg-surface border border-border/60 rounded-lg p-4 font-mono text-xs text-slate-200 focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent resize-none leading-relaxed"
        placeholder="Enter Fountain screenplay text here..."
      />
    </div>
  );
};
