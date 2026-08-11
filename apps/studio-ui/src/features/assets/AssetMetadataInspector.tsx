import React from 'react';
import { Shield, FileText } from 'lucide-react';
import { AssetMetadataView } from './types';

interface AssetMetadataInspectorProps {
  asset?: AssetMetadataView;
  lang?: 'ta-IN' | 'en-US';
}

const defaultAsset: AssetMetadataView = {
  asset_id: 'ast-video-scene1-master',
  display_name: { 'ta-IN': 'காட்சி 1 முதன்மை வீடியோ', 'en-US': 'Scene 1 Master Render' },
  asset_category: 'Video',
  mime_type: 'video/mp4',
  file_size_bytes: 45200100,
  sha256_checksum: 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855',
  created_at: '2026-08-04T08:00:00Z',
};

export const AssetMetadataInspector: React.FC<AssetMetadataInspectorProps> = ({
  asset = defaultAsset,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'வளத் தரவுகள் (Asset Metadata)' : 'Asset Metadata Inspector';
  const name = asset.display_name[lang] || asset.display_name['en-US'];

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <FileText className="w-4 h-4 text-blue-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3 flex-1 overflow-y-auto">
        <div className="bg-surface/80 border border-border/60 rounded-lg p-3">
          <div className="text-xs font-semibold text-slate-200">{name}</div>
          <div className="text-[11px] font-mono text-purple-400 mt-1">{asset.asset_id}</div>
        </div>

        <div className="bg-surface/80 border border-border/60 rounded-lg p-3 space-y-2 text-xs">
          <div className="flex justify-between">
            <span className="text-slate-400">MIME Type:</span>
            <span className="font-mono text-slate-200">{asset.mime_type}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-slate-400">File Size:</span>
            <span className="font-mono text-slate-200">{(asset.file_size_bytes / 1024 / 1024).toFixed(2)} MB</span>
          </div>
          <div className="flex justify-between">
            <span className="text-slate-400">Category:</span>
            <span className="font-mono text-slate-200">{asset.asset_category}</span>
          </div>
        </div>

        <div className="bg-surface/80 border border-border/60 rounded-lg p-3">
          <div className="flex items-center gap-1.5 text-xs text-emerald-400 font-semibold mb-1">
            <Shield className="w-3.5 h-3.5" /> SHA-256 Checksum
          </div>
          <div className="text-[10px] font-mono text-slate-300 break-all bg-background p-2 rounded border border-border/40">
            {asset.sha256_checksum}
          </div>
        </div>
      </div>
    </div>
  );
};
