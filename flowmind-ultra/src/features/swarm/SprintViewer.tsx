import { useSwarmStore } from '../../stores/useSwarmStore';

export function SprintViewer() {
  const { sprints, currentChunk } = useSwarmStore();

  if (!sprints || sprints.length === 0) {
    return null;
  }

  return (
    <div className="flex flex-col gap-2 text-xs font-mono p-4 bg-[#0a0a0f] border border-[#a855f7]/30 rounded flex-1 min-h-[300px] overflow-y-auto shadow-inner shadow-black">
      <div className="text-[#a855f7] font-bold uppercase border-b border-[#a855f7]/20 pb-2 mb-2 shrink-0 top-0 sticky bg-[#0a0a0f] z-10">
        Execution Sprints
      </div>
      <div className="flex flex-col gap-3">
        {sprints.map((sprint, idx) => {
          const isActive = currentChunk?.id === sprint.id;
          return (
            <div key={idx} className={`p-3 rounded border transition-all duration-300 ${isActive ? 'bg-[#a855f7]/10 border-[#a855f7] shadow-[0_0_15px_rgba(168,85,247,0.3)] scale-[1.02]' : 'bg-[#1f2028] border-[#2e303a]'}`}>
              <div className={`font-bold mb-1 ${isActive ? 'text-[#22d3ee]' : 'text-slate-300'}`}>
                Sprint {sprint.id}: {sprint.title}
              </div>
              <div className="text-slate-400">
                {sprint.description}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
