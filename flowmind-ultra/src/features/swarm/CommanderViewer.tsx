import { useSwarmStore } from '../../stores/useSwarmStore';

export function CommanderViewer() {
  const { commanderPlan } = useSwarmStore();

  if (!commanderPlan) {
    return (
      <div className="flex h-full w-full items-center justify-center text-slate-600 text-xs font-mono border border-[#2e303a] rounded bg-black/50 p-6 min-h-[150px]">
        [ Awaiting Commander Execution Routes ]
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4 text-xs font-mono p-4 bg-[#0a0a0f] border border-[#a855f7]/30 rounded h-full overflow-y-auto">
      <div className="text-[#a855f7] font-bold uppercase border-b border-[#a855f7]/20 pb-2 mb-2">
        Execution Clusters
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {commanderPlan.wizard_clusters?.map((cluster: any, idx: number) => (
          <div key={idx} className="bg-[#1f2028] border border-[#2e303a] p-3 rounded">
            <div className="text-[#22d3ee] font-bold mb-2">{cluster.title}</div>
            <ul className="list-disc pl-4 text-slate-300">
              {cluster.files.map((file: string, fidx: number) => (
                <li key={fidx} className="truncate">{file}</li>
              ))}
            </ul>
          </div>
        ))}
      </div>
      
      {commanderPlan.specialist_pairs?.length > 0 && (
        <>
          <div className="text-[#a855f7] font-bold uppercase border-b border-[#a855f7]/20 pb-2 mt-4 mb-2">
            Specialist Pairs
          </div>
          <div className="flex flex-col gap-2">
            {commanderPlan.specialist_pairs.map((pair: any, idx: number) => (
              <div key={idx} className="flex items-center gap-2 text-slate-300">
                <span className="bg-[#1f2028] px-2 py-1 rounded border border-[#2e303a] flex-1 truncate">{pair.producer_file}</span>
                <span className="text-[#a855f7]">➔</span>
                <span className="bg-[#1f2028] px-2 py-1 rounded border border-[#2e303a] flex-1 truncate">{pair.consumer_file}</span>
              </div>
            ))}
          </div>
        </>
      )}
    </div>
  );
}
