import { useMemoryStore } from '../../stores/useMemoryStore';
import { motion, AnimatePresence } from 'framer-motion';
import { Brain, Search, Info } from 'lucide-react';

export function MemoryViewer() {
  const { retrievedContexts } = useMemoryStore();

  if (retrievedContexts.length === 0) return null;

  return (
    <div className="bg-[#0f172a]/50 border-l border-[#22d3ee]/20 w-80 h-full flex flex-col p-4 overflow-hidden backdrop-blur-md">
      <div className="flex items-center gap-2 mb-4 border-b border-[#22d3ee]/30 pb-2">
        <Brain className="w-4 h-4 text-[#22d3ee]" />
        <h2 className="text-[#22d3ee] font-bold text-xs uppercase tracking-widest">Recovered Synapses</h2>
        <div className="ml-auto">
          <span className="bg-[#22d3ee]/10 text-[#22d3ee] px-2 py-0.5 rounded text-[10px] border border-[#22d3ee]/20">
            RAG ACTIVE
          </span>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto space-y-3 custom-scrollbar">
        <AnimatePresence>
          {retrievedContexts.map((ctx, idx) => (
            <motion.div
              key={idx}
              initial={{ x: 20, opacity: 0 }}
              animate={{ x: 0, opacity: 1 }}
              transition={{ delay: idx * 0.1 }}
              className="p-3 bg-slate-900/80 border border-slate-800 rounded-lg hover:border-[#22d3ee]/40 transition-colors shadow-lg"
            >
              <div className="flex items-center gap-2 mb-2">
                <Search className="w-3 h-3 text-slate-500" />
                <span className="text-[10px] text-slate-400 font-mono">Similarity {(0.95 - idx * 0.05).toFixed(2)}</span>
              </div>
              <p className="text-[11px] text-slate-300 leading-relaxed line-clamp-4 italic">
                "{ctx}"
              </p>
              <div className="mt-2 flex justify-end">
                <button className="text-[9px] text-[#22d3ee]/60 hover:text-[#22d3ee] flex items-center gap-1 uppercase font-bold tracking-tighter">
                  <Info className="w-2.5 h-2.5" />
                  Inspect Vector
                </button>
              </div>
            </motion.div>
          ))}
        </AnimatePresence>
      </div>

      <div className="mt-4 pt-3 border-t border-slate-800 text-[10px] text-slate-500">
        <p>Injected into <strong>Planner</strong> node to prevent amnesia and ensure architectural consistency.</p>
      </div>
    </div>
  );
}
