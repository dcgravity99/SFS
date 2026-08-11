import React from 'react';
import { Box, Link } from 'lucide-react';
import { PropAssetReference } from './types';

interface PropRegistryPickerProps {
  propsList?: PropAssetReference[];
  onSelectProp?: (assetId: string) => void;
  lang?: 'ta-IN' | 'en-US';
}

const defaultPropsList: PropAssetReference[] = [
  {
    asset_id: 'ast-prop-table-55',
    display_name: { 'ta-IN': 'இயக்குனர் மேஜை', 'en-US': 'Director Table' },
    category: 'Furniture',
  },
  {
    asset_id: 'ast-prop-chair-56',
    display_name: { 'ta-IN': 'திரைப்பட நாற்காலி', 'en-US': 'Director Chair' },
    category: 'Furniture',
  },
  {
    asset_id: 'ast-prop-spotlight-88',
    display_name: { 'ta-IN': 'ஸ்டுடியோ விளக்கு', 'en-US': 'Studio Spotlight' },
    category: 'Lighting',
  },
];

export const PropRegistryPicker: React.FC<PropRegistryPickerProps> = ({
  propsList = defaultPropsList,
  onSelectProp,
  lang = 'ta-IN',
}) => {
  const titleText = lang === 'ta-IN' ? 'சொத்து தேர்வு (Prop Registry)' : 'Prop Registry Picker';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center gap-2 border-b border-border pb-3 mb-4">
        <Box className="w-4 h-4 text-amber-400" />
        <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
      </div>

      <div className="space-y-3 overflow-y-auto flex-1 pr-1">
        {propsList.map((item) => {
          const name = item.display_name[lang] || item.display_name['en-US'];
          return (
            <div
              key={item.asset_id}
              onClick={() => onSelectProp && onSelectProp(item.asset_id)}
              className="bg-surface/80 border border-border/60 rounded-lg p-3 hover:border-amber-500/50 transition-colors cursor-pointer"
            >
              <div className="flex items-center justify-between">
                <span className="text-xs font-semibold text-slate-200">{name}</span>
                <span className="text-[10px] text-amber-400 bg-amber-500/10 px-2 py-0.5 rounded">
                  {item.category}
                </span>
              </div>
              <div className="mt-2 text-[10px] font-mono text-slate-400 flex items-center gap-1">
                <Link className="w-3 h-3 text-purple-400" /> AssetId: {item.asset_id}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
