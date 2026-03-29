import { useLLMStore } from '../../stores/useLLMStore';
import { useState } from 'react';

export function FlattenerSettingsPanel() {
  const { ignoredDirectories, setIgnoredDirectories } = useLLMStore();
  const [newDir, setNewDir] = useState('');

  const removeDir = (dirToRemove: string) => {
    setIgnoredDirectories(ignoredDirectories.filter(d => d !== dirToRemove));
  };

  const addDir = () => {
    if (newDir.trim() && !ignoredDirectories.includes(newDir.trim())) {
      setIgnoredDirectories([...ignoredDirectories, newDir.trim()]);
      setNewDir('');
    }
  };

  return (
    <div className="bg-[#121318] border border-panel-border rounded-lg p-4 mb-4 flex flex-col gap-3">
      <div className="flex justify-between items-center">
        <h3 className="text-sm font-bold text-neon-cyan tracking-wider uppercase">Context Flattener Settings</h3>
        <span className="text-xs text-slate-500">Ignored Directories limit API Token usage by excluding heavy/unnecessary folders during origin ingestion.</span>
      </div>
      
      <div className="flex flex-wrap gap-2">
        {ignoredDirectories.map(dir => (
          <div key={dir} className="flex items-center gap-2 bg-[#1f2028] border border-[#2e303a] px-3 py-1 rounded text-xs">
            <span className="text-slate-300 font-mono">{dir}</span>
            <button 
              onClick={() => removeDir(dir)}
              className="text-slate-500 hover:text-red-400 transition-colors"
            >
              ✕
            </button>
          </div>
        ))}
        
        <div className="flex items-center gap-2 bg-[#0a0a0f] border border-[#2e303a] px-2 py-1 rounded">
          <input 
            value={newDir}
            onChange={e => setNewDir(e.target.value)}
            onKeyDown={e => e.key === 'Enter' && addDir()}
            placeholder="Add directory..."
            className="bg-transparent text-xs text-slate-300 outline-none w-24 font-mono placeholder:text-slate-600"
          />
          <button 
            onClick={addDir}
            className="text-neon-cyan hover:text-white transition-colors text-xs font-bold"
          >
            + ADD
          </button>
        </div>
      </div>
    </div>
  );
}
