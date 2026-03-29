import { useTimeTravelStore } from '../../stores/useTimeTravelStore';
import { useWorkspaceStore } from '../../stores/useWorkspaceStore';
import { motion } from 'framer-motion';
import { History, Rewind, ChevronLeft, ChevronRight } from 'lucide-react';

export function ScrubSlider() {
  const { timeline, currentScrubIndex, setCurrentScrubIndex, revertSnapshot, isReverting } = useTimeTravelStore();
  const { currentWorkspace } = useWorkspaceStore();

  if (timeline.length === 0) return null;

  const handleScrub = (index: number) => {
    setCurrentScrubIndex(index);
  };

  const handleRevert = async () => {
    if (currentScrubIndex === -1 || !currentWorkspace) return;
    const snapshot = timeline[currentScrubIndex];
    await revertSnapshot(currentWorkspace.path.replace('file://', ''), snapshot.timestamp);
  };

  const activeSnapshot = currentScrubIndex !== -1 ? timeline[currentScrubIndex] : null;

  return (
    <div className="bg-[#0f172a]/80 border-t border-[#a855f7]/30 p-4 backdrop-blur-xl shrink-0">
      <div className="max-w-6xl mx-auto">
        <div className="flex items-center gap-4 mb-3">
          <div className="flex items-center gap-2">
            <History className="w-4 h-4 text-[#a855f7]" />
            <span className="text-xs font-bold uppercase tracking-widest text-slate-400">Temporal Nexus</span>
          </div>
          
          {activeSnapshot && (
            <motion.div 
              initial={{ opacity: 0, y: 5 }} 
              animate={{ opacity: 1, y: 0 }}
              className="flex-1 px-4 py-1.5 bg-purple-500/10 border border-purple-500/20 rounded-full flex items-center justify-between"
            >
              <div className="flex items-center gap-3">
                <span className="text-[10px] font-mono text-purple-400">
                  {new Date(activeSnapshot.timestamp * 1000).toLocaleTimeString()}
                </span>
                <span className="text-xs text-slate-200 font-medium truncate max-w-md">
                  {activeSnapshot.message}
                </span>
              </div>
              <button 
                onClick={handleRevert}
                disabled={isReverting}
                className="text-[10px] font-bold bg-purple-600 hover:bg-purple-500 text-white px-3 py-0.5 rounded-full shadow-[0_0_10px_rgba(168,85,247,0.4)] transition-all disabled:opacity-50"
              >
                {isReverting ? 'RESTORING...' : 'REVERT TO THIS STATE'}
              </button>
            </motion.div>
          )}
        </div>

        <div className="relative h-12 flex items-center px-2">
          {/* Timeline Track */}
          <div className="absolute left-0 right-0 h-0.5 bg-slate-800 rounded-full mx-4" />
          
          <div className="flex-1 flex justify-between items-center relative z-10">
            {timeline.slice().reverse().map((node, i) => {
              const actualIndex = timeline.length - 1 - i;
              const isActive = actualIndex === currentScrubIndex;
              return (
                <button
                  key={node.timestamp}
                  onClick={() => handleScrub(actualIndex)}
                  className={`group relative flex flex-col items-center transition-all ${isActive ? 'scale-125' : 'hover:scale-110'}`}
                >
                  <div className={`w-3 h-3 rounded-full border-2 transition-all ${isActive ? 'bg-[#a855f7] border-white shadow-[0_0_10px_#a855f7]' : 'bg-slate-900 border-slate-700 group-hover:border-slate-500'}`} />
                  <span className={`absolute -top-6 text-[9px] font-mono transition-opacity whitespace-nowrap ${isActive ? 'opacity-100 text-purple-400' : 'opacity-0 group-hover:opacity-100 text-slate-500'}`}>
                    T-{timeline.length - 1 - actualIndex}
                  </span>
                </button>
              );
            })}
          </div>
        </div>

        <div className="flex justify-center gap-8 mt-2">
           <button className="text-slate-600 hover:text-slate-400 transition-colors"><ChevronLeft className="w-4 h-4" /></button>
           <div className="flex items-center gap-2 text-[10px] text-slate-500 font-bold uppercase tracking-tighter">
             <Rewind className="w-3 h-3" />
             Historical Scrubbing Active
           </div>
           <button className="text-slate-600 hover:text-slate-400 transition-colors"><ChevronRight className="w-4 h-4" /></button>
        </div>
      </div>
    </div>
  );
}
