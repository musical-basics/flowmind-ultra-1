import { create } from 'zustand';
import { Bridge } from '../core/ipc/bridge';

export interface LLMConfig {
  provider: 'openrouter' | 'anthropic';
  modelId: string;
}

interface LLMState {
  apiKey: string;
  agents: {
    overseer: LLMConfig;
    planner: LLMConfig;
    executor: LLMConfig;
  };
  models: any[];
  setApiKey: (key: string) => void;
  setAgentConfig: (agent: 'overseer' | 'planner' | 'executor', config: LLMConfig) => void;
  fetchModels: () => Promise<void>;
}

export const useLLMStore = create<LLMState>((set, get) => ({
  apiKey: '',
  agents: {
    overseer: { provider: 'openrouter', modelId: 'anthropic/claude-3-haiku' },
    planner: { provider: 'openrouter', modelId: 'google/gemini-2.5-flash' },
    executor: { provider: 'openrouter', modelId: 'anthropic/claude-3.5-sonnet' },
  },
  models: [],
  setApiKey: (key) => set({ apiKey: key }),
  setAgentConfig: (agent, config) => set((state) => ({
    agents: { ...state.agents, [agent]: config }
  })),
  fetchModels: async () => {
    const { apiKey } = get();
    try {
      const data = await Bridge.llmFetchModels(apiKey);
      if (data && data.data) {
        set({ models: data.data });
      }
    } catch (e) {
      console.warn("Failed to fetch models", e);
      set({
        models: [
          { id: 'anthropic/claude-3.5-sonnet', name: 'Claude 3.5 Sonnet (Fallback)' },
          { id: 'anthropic/claude-3-haiku', name: 'Claude 3 Haiku (Fallback)' },
          { id: 'google/gemini-2.5-flash', name: 'Gemini 2.5 Flash (Fallback)' }
        ]
      });
    }
  }
}));
