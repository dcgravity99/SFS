import React from 'react';
import { Box, Camera, User, Layers, MapPin, Eye } from 'lucide-react';
import { SceneNodeView } from './types';

interface SceneGraphInspectorProps {
  nodes?: SceneNodeView[];
  selectedNodeId?: string;
  onSelectNode?: (nodeId: string) => void;
  lang?: 'ta-IN' | 'en-US';
}

const defaultNodes: SceneNodeView[] = [
  {
    node_id: 'node-cam-main-01',
    display_name: { 'ta-IN': 'முதன்மை கேமரா', 'en-US': 'Main Camera' },
    node_type: 'Camera',
    transform: { position: [0, 1.6, 5], rotation: [0, 0, 0], scale: [1, 1, 1] },
  },
  {
    node_id: 'node-char-vikram-02',
    display_name: { 'ta-IN': 'விக்ரம் (கதாநாயகன்)', 'en-US': 'Vikram (Lead)' },
    node_type: 'Character',
    transform: { position: [-1, 0, 0], rotation: [0, 45, 0], scale: [1, 1, 1] },
    asset_id: 'ast-char-vikram-101',
  },
  {
    node_id: 'node-prop-table-03',
    display_name: { 'ta-IN': 'இயக்குனர் மேஜை', 'en-US': 'Director Table' },
    node_type: 'Prop',
    transform: { position: [0, 0, 1], rotation: [0, 0, 0], scale: [1, 1, 1] },
    asset_id: 'ast-prop-table-55',
  },
  {
    node_id: 'node-marker-focal-04',
    display_name: { 'ta-IN': 'குவியப் புள்ளி', 'en-US': 'Focal Marker' },
    node_type: 'Marker',
    transform: { position: [0, 1.2, 0], rotation: [0, 0, 0], scale: [1, 1, 1] },
  },
];

export const SceneGraphInspector: React.FC<SceneGraphInspectorProps> = ({
  nodes = defaultNodes,
  selectedNodeId = 'node-cam-main-01',
  onSelectNode,
  lang = 'ta-IN',
}) => {
  const getNodeIcon = (type: string) => {
    switch (type) {
      case 'Camera': return <Camera className="w-3.5 h-3.5 text-blue-400" />;
      case 'Character': return <User className="w-3.5 h-3.5 text-emerald-400" />;
      case 'Prop': return <Box className="w-3.5 h-3.5 text-amber-400" />;
      case 'Marker': return <MapPin className="w-3.5 h-3.5 text-purple-400" />;
      default: return <Layers className="w-3.5 h-3.5 text-slate-400" />;
    }
  };

  const titleText = lang === 'ta-IN' ? 'காட்சி அமைப்பு (Scene Graph)' : 'Scene Graph Inspector';

  return (
    <div className="glass-card rounded-xl p-4 flex flex-col h-[500px]">
      <div className="flex items-center justify-between border-b border-border pb-3 mb-4">
        <div className="flex items-center gap-2">
          <Eye className="w-4 h-4 text-blue-400" />
          <h2 className="text-sm font-semibold text-slate-200">{titleText}</h2>
        </div>
        <span className="text-[10px] font-mono text-slate-400 bg-surface px-2 py-0.5 rounded border border-border">
          {nodes.length} Nodes
        </span>
      </div>

      <div className="space-y-2 overflow-y-auto flex-1 pr-1">
        {nodes.map((node) => {
          const isSelected = selectedNodeId === node.node_id;
          const name = node.display_name[lang] || node.display_name['en-US'];
          return (
            <div
              key={node.node_id}
              onClick={() => onSelectNode && onSelectNode(node.node_id)}
              className={`p-3 rounded-lg border transition-all cursor-pointer flex items-center justify-between ${
                isSelected
                  ? 'bg-primary/10 border-primary text-white shadow-sm'
                  : 'bg-surface/60 border-border/40 text-slate-300 hover:border-border hover:bg-surface-hover'
              }`}
            >
              <div className="flex items-center gap-2.5">
                {getNodeIcon(node.node_type)}
                <div>
                  <div className="text-xs font-medium">{name}</div>
                  <div className="text-[10px] font-mono text-slate-400">{node.node_id}</div>
                </div>
              </div>
              <span className="text-[10px] font-mono text-slate-400 bg-surface px-2 py-0.5 rounded">
                {node.node_type}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
};
