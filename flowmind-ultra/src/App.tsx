import { useEffect } from 'react';
import { SwarmDashboard } from './features/swarm/SwarmDashboard';
import { useSwarmStore } from './stores/useSwarmStore';
import { useMemoryStore } from './stores/useMemoryStore';
import { useLLMStore } from './stores/useLLMStore';

function App() {
  const initSwarm = useSwarmStore(s => s.initListeners);
  const initMemory = useMemoryStore(s => s.initListeners);
  const fetchModels = useLLMStore(s => s.fetchModels);

  useEffect(() => {
    initSwarm();
    initMemory();
    fetchModels();
  }, [initSwarm, initMemory, fetchModels]);

  return (
    <div className="h-screen w-screen bg-[#050508]">
      <SwarmDashboard />
    </div>
  );
}

export default App;
