import { create } from 'zustand';
import { listen } from '@tauri-apps/api/event';

interface MemoryStore {
  retrievedContexts: string[];
  vectorDbStats: any;
  setRetrievedContexts: (contexts: string[]) => void;
  initListeners: () => void;
}

export const useMemoryStore = create<MemoryStore>((set) => ({
  retrievedContexts: [],
  vectorDbStats: null,
  setRetrievedContexts: (contexts) => set({ retrievedContexts: contexts }),
  initListeners: async () => {
    await listen<string>('memory_retrieved', (event) => {
        set({ retrievedContexts: event.payload.split('\n---\n') });
    });
  }
}));
