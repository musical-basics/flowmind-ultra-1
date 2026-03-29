import { create } from 'zustand';

interface MemoryStore {
  retrievedContexts: any[];
  vectorDbStats: any;
}

export const useMemoryStore = create<MemoryStore>(() => ({
  retrievedContexts: [],
  vectorDbStats: null,
}));
