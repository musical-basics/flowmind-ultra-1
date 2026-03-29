import { useEffect } from 'react';
import { useSwarmStore } from '../../stores/useSwarmStore';
import { NodeIndicator } from './NodeIndicator';

export function SwarmVisualizer() {
  const { stations, currentChunk, initListeners } = useSwarmStore();

  useEffect(() => {
    initListeners();
  }, [initListeners]);

  return (
    <div className="w-full bg-[#0a0a0f] border border-[#22d3ee]/20 p-4 rounded-lg flex flex-col gap-4 overflow-x-auto overflow-y-hidden shrink-0 shadow-[0_0_20px_rgba(34,211,238,0.1)]">
      <div className="flex items-center justify-between pb-2 border-b border-[#2e303a]">
        <span className="text-xs text-[#22d3ee] font-bold tracking-widest uppercase">Swarm Pipeline</span>
        {currentChunk && (
          <span className="text-[10px] text-[#a855f7] border border-[#a855f7]/30 px-3 py-1 rounded-full bg-[#160a22]">
            Sprint {currentChunk.id}/{currentChunk.total}: {currentChunk.title}
          </span>
        )}
      </div>
      
      <div className="flex items-start justify-center min-w-max px-2 py-2">
        {stations.map((s, idx) => (
          <NodeIndicator 
            key={s.id}
            label={s.label}
            status={s.status}
            detail={s.detail}
            isLast={idx === stations.length - 1}
          />
        ))}
      </div>
    </div>
  );
}
