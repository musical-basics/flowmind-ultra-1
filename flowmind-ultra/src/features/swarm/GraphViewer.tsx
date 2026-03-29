import { ReactFlow, Background, Controls } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { useSwarmStore } from '../../stores/useSwarmStore';

export function GraphViewer() {
  const { graph } = useSwarmStore();

  if (!graph || !graph.files) {
    return (
      <div className="flex h-full w-full items-center justify-center text-slate-600 text-xs font-mono border border-[#2e303a] rounded bg-black/50 p-6 min-h-[300px]">
        [ Awaiting Topological Dependency Graph ]
      </div>
    );
  }

  const nodes = graph.files.map((f: any, idx: number) => ({
    id: f.filepath,
    position: { x: (idx % 3) * 220, y: Math.floor(idx / 3) * 120 },
    data: { label: f.filepath.split('/').pop() || f.filepath },
    style: { 
      background: '#1f2028', 
      color: '#22d3ee', 
      border: '1px solid rgba(34, 211, 238, 0.3)', 
      borderRadius: '8px',
      fontSize: '12px',
      width: 180,
      fontFamily: 'ui-monospace, Consolas, monospace'
    },
  }));

  const edges = graph.files.flatMap((f: any) => 
    (f.dependencies || []).map((dep: string) => ({
      id: `${dep}->${f.filepath}`,
      source: dep,
      target: f.filepath,
      animated: true,
      style: { stroke: '#a855f7', strokeWidth: 2 }
    }))
  );

  return (
    <div className="w-full h-full min-h-[300px] border border-[#2e303a] rounded bg-black/50">
      <ReactFlow nodes={nodes} edges={edges} fitView>
        <Background gap={16} size={1} color="#2e303a" />
        <Controls />
      </ReactFlow>
    </div>
  );
}
