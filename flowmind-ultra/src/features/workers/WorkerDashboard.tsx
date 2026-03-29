import { useEffect } from 'react';
import { useWorkerStore } from '../../stores/useWorkerStore';
import { TerminalPanel } from '../terminal/TerminalPanel';

export function WorkerDashboard() {
  const { workers, queueDepth, initListeners } = useWorkerStore();

  useEffect(() => {
    initListeners();
  }, [initListeners]);

  const workerList = Object.values(workers);

  return (
    <div className="flex flex-col gap-4 font-mono text-xs h-full w-full">
      <div className="flex items-center justify-between border-b border-[#a855f7]/30 pb-2">
        <span className="text-[#a855f7] font-bold uppercase tracking-wider">Parallel Execution Pools</span>
        <span className="text-[#22d3ee] px-2 py-1 bg-[#1f2028] rounded border border-[#2e303a] shadow-inner shadow-black">
          Queue Depth: {queueDepth} Pending
        </span>
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-3 gap-4 w-full flex-1 min-h-[300px]">
        {workerList.map((worker) => (
          <div key={worker.workerId} className="flex flex-col bg-[#0a0a0f] border border-[#2e303a] rounded-lg shadow-[0_0_20px_rgba(0,0,0,0.5)] overflow-hidden">
            <div className="flex items-center justify-between px-3 py-2 bg-[#160a22] border-b border-[#a855f7]/30 shrink-0">
               <span className="text-[#a855f7] font-bold tracking-wider">Worker {worker.workerId}</span>
               <span className={`px-2 py-0.5 rounded-full text-[10px] ${
                  worker.state === 'Running' ? 'bg-[#a855f7] text-white animate-pulse shadow-[0_0_10px_rgba(168,85,247,0.5)]' : 'bg-slate-700 text-slate-300'
               }`}>
                 {worker.state.toUpperCase()}
               </span>
            </div>
            
            <div className="p-3 bg-[#111216] border-b border-[#2e303a] h-24 shrink-0 overflow-y-auto">
              {worker.task ? (
                <div className="flex flex-col gap-1">
                  <div className="text-[#22d3ee] font-bold pr-2 break-words leading-tight" title={worker.task.title}>
                    {worker.task.title}
                  </div>
                  <div className="text-slate-400 text-[10px] truncate max-w-full">
                    <span className="text-slate-500">Lock:</span> {worker.task.files.join(', ')}
                  </div>
                  <div className="mt-1 text-[#a855f7] text-[10px]">
                    <span className="text-slate-500">Status:</span> {worker.task.status}
                  </div>
                </div>
              ) : (
                <div className="text-slate-600 flex items-center justify-center h-full italic">
                  Awaiting Assignment...
                </div>
              )}
            </div>

            <div className="flex-1 bg-black relative">
               <TerminalPanel overrideId={`worker-${worker.workerId}`} hideHeader={true} />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
