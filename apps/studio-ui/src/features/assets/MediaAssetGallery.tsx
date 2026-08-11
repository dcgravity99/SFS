import React from 'react';
import { Folder, Link } from 'lucide-react';
import { AssetMetadataView } from './types';

interface MediaAssetGalleryProps {
  assets?: AssetMetadataView[];
  selectedAssetId?: string;
  onSelectAsset?: (assetId: string) => void;
  lang?: 'ta-IN' | 'en-US';
}

const defaultAssets: AssetMetadataView[] = [
  {
    asset_id: 'ast-video-scene1-master',
    display_name: { 'ta-IN': 'காட்சி 1 முதன்மை வீடியோ', 'en-US': 'Scene 1 Master Render' },
    asset_category: 'Video',
    mime_type: 'video/mp4',
    file_size_bytes: 45200100,
    sha256_checksum: 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855',
    created_at: '2026-08-04T08:00:00Z',
  },
  {
    asset_id: 'ast-lora-vikram-v2',
    display_name: { 'ta-IN': 'விக்ரம் லோரா மாடல்', 'en-US': 'Vikram LoRA Model Asset' },
    asset_category: 'Model',
    mime_type: 'application/x-safetensors',
    file_size_bytes: 144000000,
    sha256_checksum: '8f4e2c1a3b5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f',
    created_at: '2026-08-04T07:30:00Z',
  },
];

export const MediaAssetGallery: React.FC<MediaAssetGalleryProps> = ({
  assets = defaultAssets,
  selectedAssetId = 'ast-video-scene1-master',
  onSelectAsset,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'வளக் கூடம் (Media Asset Gallery)' : 'Digital Asset Gallery';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Folder className="w-4 h-4 text-emerald-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {assets.map((a) => {
          const name = a.display_name[lang] || a.display_name['en-US'];
          const isSelected = selectedAssetId === a.asset_id;
          return (
            <div
              key={a.asset_id}
              onClick={() => onSelectAsset && onSelectAsset(a.asset_id)}
              className={`p-3 rounded-lg border transition-all cursor-pointer ${
                isSelected
                  ? 'bg-emerald-500/10 border-emerald-500 text-white shadow-sm'
                  : 'bg-surface/80 border-border/60 text-slate-300 hover:border-border'
              }`}
            >
              <div className="flex items-center justify-between">
                <span className="text-xs font-semibold">{name}</span>
                <span className="text-[10px] font-mono text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded">
                  {a.asset_category}
                </span>
              </div>
              <div className="mt-2 text-[10px] font-mono text-slate-400 flex items-center justify-between">
                <span className="flex items-center gap-1">
                  <Link className="w-3 h-3 text-purple-400" /> {a.asset_id}
                </span>
                <span>{(a.file_size_bytes / 1024 / 1024).toFixed(1)} MB</span>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
