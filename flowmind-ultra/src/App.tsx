import { useEffect } from 'react';
import { SwarmDashboard } from './features/swarm/SwarmDashboard';
import { useSwarmStore } from './stores/useSwarmStore';
import { useMemoryStore } from './stores/useMemoryStore';
import { useLLMStore } from './stores/useLLMStore';
import { useSupabaseStore } from './stores/useSupabaseStore';
import { useWorkspaceStore } from './stores/useWorkspaceStore';
import { invoke } from '@tauri-apps/api/core';

function App() {
  const initSwarm = useSwarmStore(s => s.initListeners);
  const initMemory = useMemoryStore(s => s.initListeners);
  const fetchModels = useLLMStore(s => s.fetchModels);
  const { initClient, syncRun } = useSupabaseStore();
  const { currentWorkspace } = useWorkspaceStore();

  useEffect(() => {
    initSwarm();
    initMemory();
    fetchModels();

    async function setupSupabase() {
      try {
        const config = await invoke<[string, string]>('get_supabase_config');
        if (config && config[0] && config[1]) {
          initClient(config[0], config[1]);
        }
      } catch (err) {
        console.warn('Supabase config failed:', err);
      }
    }
    setupSupabase();
  }, [initSwarm, initMemory, fetchModels, initClient]);

  useEffect(() => {
    if (currentWorkspace) {
      syncRun(currentWorkspace.path.replace('file://', ''));
    }
  }, [currentWorkspace, syncRun]);

  return (
    <div className="h-screen w-screen bg-[#050508]">
      <SwarmDashboard />
    </div>
  );
}

export default App;
