import React, { useState } from 'react';
import { Cpu, Link, Plus } from 'lucide-react';
import { StudioIpcService } from '../../services/ipc.service';
import { CharacterCreatePayload, LoraBindPayload } from './types';

interface LoraBindingPanelProps {
  onCharacterCreated?: () => void;
}

export const LoraBindingPanel: React.FC<LoraBindingPanelProps> = ({ onCharacterCreated }) => {
  const [characterName, setCharacterName] = useState('');
  const [characterRole, setCharacterRole] = useState('Lead Protagonist');
  const [selectedAssetId, setSelectedAssetId] = useState('ast-lora-actor1-v2');
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleCreateCharacter = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!characterName.trim()) return;

    const response = await StudioIpcService.executeEngineCommand<CharacterCreatePayload, { character_id: string }>(
      'character_create',
      { name: characterName, role: characterRole }
    );

    if (response.success) {
      const charId = response.data?.character_id || `char-${Date.now()}`;
      // Bind selected AssetId reference
      await StudioIpcService.executeEngineCommand<LoraBindPayload, void>('character_bind_lora', {
        character_id: charId,
        lora_asset_id: selectedAssetId,
      });

      setStatusMessage(`Created ${characterName} & bound AssetId: ${selectedAssetId}`);
      setCharacterName('');
      if (onCharacterCreated) onCharacterCreated();
    }
  };

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Cpu className="w-4 h-4 text-blue-400" />
        <h2 className="text-sm font-semibold text-slate-200">Character & LoRA Asset Binder</h2>
      </div>

      <form onSubmit={handleCreateCharacter} className="space-y-4 flex-1">
        <div>
          <label className="block text-xs font-medium text-slate-300 mb-1">Character Name</label>
          <input
            type="text"
            value={characterName}
            onChange={(e) => setCharacterName(e.target.value)}
            placeholder="e.g. Vikram"
            className="w-full bg-surface border border-border/60 rounded-lg px-3 py-2 text-xs text-slate-200 focus:outline-none focus:ring-2 focus:ring-primary"
          />
        </div>

        <div>
          <label className="block text-xs font-medium text-slate-300 mb-1">Character Role</label>
          <select
            value={characterRole}
            onChange={(e) => setCharacterRole(e.target.value)}
            className="w-full bg-surface border border-border/60 rounded-lg px-3 py-2 text-xs text-slate-200 focus:outline-none focus:ring-2 focus:ring-primary"
          >
            <option value="Lead Protagonist">Lead Protagonist</option>
            <option value="Antagonist">Antagonist</option>
            <option value="Supporting Character">Supporting Character</option>
          </select>
        </div>

        <div>
          <label className="block text-xs font-medium text-slate-300 mb-1 flex items-center gap-1">
            <Link className="w-3 h-3 text-purple-400" /> LoRA Model Reference (AssetId Only)
          </label>
          <select
            value={selectedAssetId}
            onChange={(e) => setSelectedAssetId(e.target.value)}
            className="w-full bg-surface border border-border/60 rounded-lg px-3 py-2 text-xs text-slate-200 font-mono focus:outline-none focus:ring-2 focus:ring-primary"
          >
            <option value="ast-lora-actor1-v2">ast-lora-actor1-v2 (.safetensors)</option>
            <option value="ast-lora-heroine-v1">ast-lora-heroine-v1 (.safetensors)</option>
            <option value="ast-lora-villain-v3">ast-lora-villain-v3 (.safetensors)</option>
          </select>
        </div>

        <button
          type="submit"
          className="w-full mt-4 flex items-center justify-center gap-1.5 py-2.5 px-4 bg-primary hover:bg-blue-600 text-white font-medium text-xs rounded-lg transition-colors shadow-lg shadow-blue-500/20"
        >
          <Plus className="w-4 h-4" /> Create & Bind AssetId
        </button>

        {statusMessage && (
          <div className="p-2.5 bg-emerald-500/10 border border-emerald-500/20 rounded-lg text-[11px] text-emerald-400">
            {statusMessage}
          </div>
        )}
      </form>
    </div>
  );
};
