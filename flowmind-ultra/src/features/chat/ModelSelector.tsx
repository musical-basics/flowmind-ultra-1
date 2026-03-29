import { useEffect } from 'react';
import { useLLMStore } from '../../stores/useLLMStore';

export function ModelSelector({ agentRole }: { agentRole: 'overseer' | 'planner' | 'executor' }) {
  const { agents, models, setAgentConfig, fetchModels } = useLLMStore();
  const config = agents[agentRole];

  useEffect(() => {
    if (models.length === 0) fetchModels();
  }, [models.length, fetchModels]);

  return (
    <div className="flex flex-col gap-2 bg-[#0a0a0f] p-3 rounded-lg border border-[#22d3ee]/20 text-xs w-full shadow-[0_0_15px_rgba(0,0,0,0.5)]">
      <span className="text-[#a855f7] uppercase font-bold tracking-wider">{agentRole} Node</span>
      <div className="grid grid-cols-2 gap-2">
        <select 
          className="bg-[#1f2028] text-slate-300 rounded p-1 border border-[#2e303a] outline-none hover:border-[#22d3ee]/50 transition-colors"
          value={config.provider}
          onChange={(e) => setAgentConfig(agentRole, { ...config, provider: e.target.value as any })}
        >
          <option value="openrouter">OpenRouter</option>
          <option value="anthropic">Anthropic</option>
        </select>
        <select 
          className="bg-[#1f2028] text-slate-300 rounded p-1 border border-[#2e303a] outline-none hover:border-[#a855f7]/50 transition-colors truncate"
          value={config.modelId}
          onChange={(e) => setAgentConfig(agentRole, { ...config, modelId: e.target.value })}
        >
          {models.length > 0 ? (
            models.map((m: any) => (
              <option key={m.id} value={m.id}>{m.name}</option>
            ))
          ) : (
            <option value={config.modelId}>{config.modelId} (Loading...)</option>
          )}
        </select>
      </div>
    </div>
  );
}
