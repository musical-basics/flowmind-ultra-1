import { useSwarmStore } from '../../stores/useSwarmStore';
import { motion, AnimatePresence } from 'framer-motion';

export function ChatGraph() {
  const qaStation = useSwarmStore(s => s.stations.find(st => st.id === 'QA'));

  return (
    <div className="p-4 h-full flex flex-col gap-4">
      <h2 className="text-[#a855f7] font-bold tracking-wider uppercase border-b border-[#a855f7]/30 pb-2">Chat & Diagnostics Stream</h2>
      
      <div className="flex-1 overflow-y-auto">
        <AnimatePresence>
          {qaStation?.status === 'Healing' && (
            <motion.div 
              initial={{ opacity: 0, scale: 0.9, y: 10 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.9 }}
              className="px-4 py-3 bg-amber-500/10 border border-amber-500/50 rounded-lg shadow-[0_0_15px_rgba(245,158,11,0.2)] mb-4"
            >
              <div className="flex items-center gap-3">
                <span className="text-xl animate-bounce">🚨</span>
                <div className="flex flex-col">
                  <span className="text-amber-500 font-bold uppercase tracking-wider text-xs">Self-Healing Initiated</span>
                  <span className="text-slate-300 text-sm mt-1">{qaStation.detail}</span>
                </div>
              </div>
            </motion.div>
          )}

          {qaStation?.status === 'AwaitingHumanFix' && (
            <motion.div 
              initial={{ opacity: 0, scale: 0.9, y: 10 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              className="px-4 py-3 bg-red-500/10 border border-red-500/50 rounded-lg shadow-[0_0_15px_rgba(239,68,68,0.2)] mb-4"
            >
              <div className="flex items-center gap-3">
                <span className="text-xl animate-pulse">🛑</span>
                <div className="flex flex-col">
                  <span className="text-red-500 font-bold uppercase tracking-wider text-xs">Manual Override Required</span>
                  <span className="text-slate-300 text-sm mt-1">{qaStation.detail}</span>
                  <div className="mt-3">
                    <button 
                      onClick={() => import('../../core/ipc/bridge').then(m => m.Bridge.manualCompilerOverride('workspace_id'))}
                      className="px-4 py-1.5 bg-red-600/20 hover:bg-red-500 hover:text-white border border-red-500/50 rounded-md text-red-500 transition-colors text-xs font-bold uppercase"
                    >
                      Bypass & Unblock
                    </button>
                  </div>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}
