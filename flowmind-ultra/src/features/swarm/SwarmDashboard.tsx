import { useState } from 'react';
import { SwarmVisualizer } from './SwarmVisualizer';
import { GraphViewer } from './GraphViewer';
import { CommanderViewer } from './CommanderViewer';
import { SpecViewer } from './SpecViewer';
import { SprintViewer } from './SprintViewer';
import { TerminalPanel } from '../terminal/TerminalPanel';
import { WorkerDashboard } from '../workers/WorkerDashboard';
import { ModelSelector } from '../chat/ModelSelector';
import { Bridge } from '../../core/ipc/bridge';
import { useLLMStore } from '../../stores/useLLMStore';
import { useWorkspaceStore } from '../../stores/useWorkspaceStore';
import { useSwarmStore } from '../../stores/useSwarmStore';
import { FileTree } from '../workspace/FileTree';

export function SwarmDashboard() {
  const [prompt, setPrompt] = useState('');
  const [working, setWorking] = useState(false);
  const [viewTab, setViewTab] = useState<'agents' | 'workers'>('agents');
  const { agents } = useLLMStore();
  const { currentWorkspace } = useWorkspaceStore();
  const { stations } = useSwarmStore();

  const isComplete = stations.find((s) => s.id === 'Complete')?.status === 'Complete';

  const handleStartSwarm = async () => {
    if (!prompt.trim() || !currentWorkspace) return;
    setWorking(true);
    // Strip trailing slash or protocol if necessary
    const path = currentWorkspace.path.replace('file://', '');
    
    await Bridge.startSwarm(
      path,
      prompt,
      agents.overseer.modelId,
      agents.planner.modelId,
      agents.executor.modelId
    );
  };

  return (
    <div className="flex h-screen w-full bg-[#050508] text-slate-300">
      {/* Left Sidebar: File Tree */}
      <div className="w-64 border-r border-panel-border bg-panel shrink-0 flex flex-col">
        <div className="h-12 flex items-center px-4 border-b border-panel-border shrink-0">
          <span className="font-bold text-sm tracking-wider uppercase text-neon-cyan drop-shadow-[0_0_8px_rgba(34,211,238,0.5)] flex items-center gap-2">
            Flowmind <span className="text-neon-purple">Ultra</span>
          </span>
        </div>
        <div className="flex-1 overflow-auto">
          <FileTree />
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col h-full overflow-y-auto p-4 gap-4">
        <SwarmVisualizer />
        
        <div className="grid grid-cols-1 xl:grid-cols-3 gap-4 shrink-0">
          <ModelSelector agentRole="overseer" />
          <ModelSelector agentRole="planner" />
          <ModelSelector agentRole="executor" />
        </div>

        <div className="flex gap-2 shrink-0">
          <input 
            className="flex-1 bg-[#0a0a0f] border border-[#22d3ee]/30 rounded p-3 text-sm text-[#22d3ee] font-mono outline-none shadow-[inset_0_0_10px_rgba(34,211,238,0.05)] focus:border-[#22d3ee] transition-colors placeholder:text-slate-600"
            placeholder="Enter Product Requirements or Feature Request..."
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
          />
          <button 
            onClick={handleStartSwarm}
            disabled={working && !isComplete}
            className="bg-[#a855f7] hover:bg-[#b975f8] text-white px-8 font-bold tracking-wider rounded border border-[#d8b4fe] shadow-[0_0_15px_rgba(168,85,247,0.5)] transition-all disabled:opacity-50 uppercase text-xs"
          >
            {working && !isComplete ? 'Executing...' : 'Engage Swarm'}
          </button>
        </div>

        <div className="flex flex-col flex-1 min-h-[400px]">
          <div className="flex gap-4 mb-2 shrink-0">
            <button onClick={() => setViewTab('agents')} className={`px-4 py-1 rounded text-xs font-bold uppercase transition-colors ${viewTab === 'agents' ? 'bg-[#a855f7] text-white' : 'text-slate-400 hover:text-white'}`}>Execution Graph</button>
            <button onClick={() => setViewTab('workers')} className={`px-4 py-1 rounded text-xs font-bold uppercase transition-colors ${viewTab === 'workers' ? 'bg-[#a855f7] text-white' : 'text-slate-400 hover:text-white'}`}>Worker Clusters</button>
          </div>

          <div className="flex-1 w-full bg-[#0a0a0f] border border-[#2e303a] rounded-lg p-4 h-full relative">
            {viewTab === 'agents' ? (
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 h-full shrink-0">
                <div className="flex flex-col gap-4 h-full">
                  <GraphViewer />
                  <CommanderViewer />
                </div>
                <div className="flex flex-col gap-4 h-full">
                  <SpecViewer />
                  <SprintViewer />
                </div>
              </div>
            ) : (
              <WorkerDashboard />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
