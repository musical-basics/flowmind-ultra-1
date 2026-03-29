import { useState } from 'react';
import { SwarmVisualizer } from './SwarmVisualizer';
import { GraphViewer } from './GraphViewer';
import { CommanderViewer } from './CommanderViewer';
import { SpecViewer } from './SpecViewer';
import { SprintViewer } from './SprintViewer';
import { WorkerDashboard } from '../workers/WorkerDashboard';
import { ModelSelector } from '../chat/ModelSelector';
import { Bridge } from '../../core/ipc/bridge';
import { useLLMStore } from '../../stores/useLLMStore';
import { useWorkspaceStore } from '../../stores/useWorkspaceStore';
import { useSwarmStore } from '../../stores/useSwarmStore';
import { useWorkerStore } from '../../stores/useWorkerStore';
import { FileTree } from '../workspace/FileTree';
import { FlattenerSettingsPanel } from './FlattenerSettingsPanel';
import { MemoryViewer } from './MemoryViewer';
import { ScrubSlider } from './ScrubSlider';
import { useTimeTravelStore } from '../../stores/useTimeTravelStore';
import { useSupabaseStore } from '../../stores/useSupabaseStore';
import { useEffect } from 'react';
import { Link2, Link2Off } from 'lucide-react';

export function SwarmDashboard() {
  const [prompt, setPrompt] = useState('');
  const [working, setWorking] = useState(false);
  const [viewTab, setViewTab] = useState<'agents' | 'workers'>('agents');
  const [showSettings, setShowSettings] = useState(false);
  const { agents, ignoredDirectories } = useLLMStore();
  const { currentWorkspace } = useWorkspaceStore();
  const { stations } = useSwarmStore();
  const { manualOverride, setManualOverride } = useWorkerStore();
  const { fetchTimeline } = useTimeTravelStore();
  const { isCollaborative, remoteApprove } = useSupabaseStore();

  useEffect(() => {
    if (currentWorkspace) {
      fetchTimeline(currentWorkspace.path.replace('file://', ''));
    }
  }, [currentWorkspace, fetchTimeline]);

  const isComplete = stations.find((s) => s.id === 'Complete')?.status === 'Complete';
  const isAwaitingApproval = stations.find((s) => s.id === 'Commander')?.status === 'AwaitingApproval';

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
      agents.executor.modelId,
      ignoredDirectories
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

        {showSettings && <FlattenerSettingsPanel />}

        <div className="flex gap-2 shrink-0">
          <input 
            className="flex-1 bg-[#0a0a0f] border border-[#22d3ee]/30 rounded p-3 text-sm text-[#22d3ee] font-mono outline-none shadow-[inset_0_0_10px_rgba(34,211,238,0.05)] focus:border-[#22d3ee] transition-colors placeholder:text-slate-600"
            placeholder="Enter Product Requirements or Feature Request..."
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
          />

          {isAwaitingApproval && (
                <div className="flex gap-4 px-4 py-2 border-t border-slate-800 bg-slate-900/40">
                  <button 
                    onClick={() => Bridge.approveCommanderPlan()}
                    className="px-6 py-2 bg-purple-600 hover:bg-purple-500 text-white text-xs font-bold rounded-lg shadow-[0_0_20px_rgba(168,85,247,0.4)] transition-all flex items-center gap-2"
                  >
                    Approve Plan
                  </button>
                  {isCollaborative && (
                    <button 
                      onClick={() => currentWorkspace && remoteApprove(currentWorkspace.path.replace('file://', ''))}
                      className="px-6 py-2 bg-[#0f172a] border border-emerald-500/30 text-emerald-400 text-xs font-bold rounded-lg hover:bg-emerald-500/10 transition-all flex items-center gap-2"
                    >
                      <Link2 className="w-3.5 h-3.5" />
                      Remote Sync
                    </button>
                  )}
                </div>
          )}

          <button 
            onClick={() => setManualOverride(!manualOverride)}
            className={`px-8 font-bold tracking-wider rounded border transition-all uppercase text-xs ${manualOverride ? 'bg-red-600 hover:bg-red-500 text-white border-red-400 shadow-[0_0_15px_rgba(239,68,68,0.5)]' : 'bg-slate-800 hover:bg-slate-700 text-slate-300 border-slate-600'}`}
          >
            {manualOverride ? 'Override Active' : 'Manual Override'}
          </button>

          <button 
            onClick={handleStartSwarm}
            disabled={working && !isComplete}
            className="bg-[#a855f7] hover:bg-[#b975f8] text-white px-8 font-bold tracking-wider rounded border border-[#d8b4fe] shadow-[0_0_15px_rgba(168,85,247,0.5)] transition-all disabled:opacity-50 uppercase text-xs"
          >
            {working && !isComplete ? 'Executing...' : 'Engage Swarm'}
          </button>

          <button 
            onClick={() => setShowSettings(!showSettings)}
            className="bg-slate-800 hover:bg-slate-700 text-slate-300 px-4 font-bold rounded border border-slate-600 transition-colors"
            title="Toggle Flattener Settings"
          >
            ⚙️
          </button>
        </div>

        <div className="flex flex-1 min-h-[400px] gap-4">
          <div className="flex flex-col flex-1">
            <div className="flex gap-4 mb-2 shrink-0">
              <button onClick={() => setViewTab('agents')} className={`px-4 py-1 rounded text-xs font-bold uppercase transition-colors ${viewTab === 'agents' ? 'bg-[#a855f7] text-white' : 'text-slate-400 hover:text-white'}`}>Execution Graph</button>
              <button onClick={() => setViewTab('workers')} className={`px-4 py-1 rounded text-xs font-bold uppercase transition-colors ${viewTab === 'workers' ? 'bg-[#a855f7] text-white' : 'text-slate-400 hover:text-white'}`}>Worker Clusters</button>
            </div>

            <div className="flex-1 w-full bg-[#0a0a0f] border border-[#2e303a] rounded-lg p-4 relative h-full shrink-0">
              {viewTab === 'agents' ? (
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 h-full">
                  <div className="flex flex-col gap-4 h-full">
                    <GraphViewer />
                    <CommanderViewer />
                  </div>
                  <div className="flex items-center gap-4">
                    <h2 className="text-xl font-bold bg-gradient-to-r from-white to-slate-400 bg-clip-text text-transparent">
                      Swarm Station
                    </h2>
                    {isCollaborative ? (
                      <div className="flex items-center gap-1.5 px-2 py-0.5 bg-emerald-500/10 border border-emerald-500/20 rounded-full">
                        <Link2 className="w-3 h-3 text-emerald-400" />
                        <span className="text-[10px] font-bold text-emerald-400 uppercase tracking-tighter">Live Link Active</span>
                      </div>
                    ) : (
                      <div className="flex items-center gap-1.5 px-2 py-0.5 bg-slate-500/10 border border-slate-500/20 rounded-full opacity-50">
                        <Link2Off className="w-3 h-3 text-slate-400" />
                        <span className="text-[10px] font-bold text-slate-400 uppercase tracking-tighter">Local Only</span>
                      </div>
                    )}
                    <SpecViewer />
                    <SprintViewer />
                  </div>
                </div>
              ) : (
                <WorkerDashboard />
              )}
            </div>
          </div>
          
          {viewTab === 'agents' && <MemoryViewer />}
        </div>
      </div>
      <ScrubSlider />
    </div>
  );
}
