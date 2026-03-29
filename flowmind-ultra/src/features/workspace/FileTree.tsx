import { useWorkerStore } from '../../stores/useWorkerStore';

export function FileTree() {
  const { workers } = useWorkerStore();
  const lockedFiles = new Set<string>();
  
  Object.values(workers).forEach(w => {
    if (w.task) {
      w.task.files.forEach(f => lockedFiles.add(f));
    }
  });

  return (
    <div className="flex flex-col p-4 font-mono text-xs gap-2">
      <div className="text-slate-500 uppercase font-bold tracking-wider mb-2 border-b border-[#2e303a] pb-1">Workspace Source</div>
      <div className="flex flex-col gap-1">
        {/* Using mock target files temporarily to demonstrate visual locks updating when a Worker picks up a cluster job */}
        {['src/main.rs', 'src/App.tsx', 'Cargo.toml', 'package.json'].map(f => {
          const isLocked = lockedFiles.has(f);
          return (
            <div key={f} className={`flex items-center gap-2 px-2 py-1 rounded transition-colors ${isLocked ? 'bg-red-500/10 border border-red-500/30' : 'hover:bg-[#1f2028]'}`}>
              <span className="text-slate-400">📄</span>
              <span className={isLocked ? 'text-red-400 font-bold' : 'text-slate-300'}>{f}</span>
              {isLocked && <span className="text-[10px] ml-auto animate-pulse">🔒</span>}
            </div>
          )
        })}
      </div>
    </div>
  );
}
