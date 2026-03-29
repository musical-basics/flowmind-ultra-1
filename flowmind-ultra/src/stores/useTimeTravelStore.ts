import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface CommitNode {
  timestamp: number;
  message: string;
}

interface TimeTravelStore {
  timeline: CommitNode[];
  currentScrubIndex: number;
  isReverting: boolean;
  
  fetchTimeline: (workspaceId: string) => Promise<void>;
  revertSnapshot: (workspaceId: string, timestamp: number) => Promise<void>;
  setCurrentScrubIndex: (index: number) => void;
}

export const useTimeTravelStore = create<TimeTravelStore>((set) => ({
  timeline: [],
  currentScrubIndex: -1,
  isReverting: false,

  fetchTimeline: async (workspaceId) => {
    try {
      const timeline = await invoke<CommitNode[]>('get_snapshot_timeline', { workspaceId });
      set({ timeline });
    } catch (err) {
      console.error('Failed to fetch timeline:', err);
    }
  },

  revertSnapshot: async (workspaceId, timestamp) => {
    set({ isReverting: true });
    try {
      await invoke('revert_to_snapshot', { workspaceId, timestamp });
    } catch (err) {
      console.error('Revert failed:', err);
    } finally {
      set({ isReverting: false });
    }
  },

  setCurrentScrubIndex: (index) => set({ currentScrubIndex: index }),
}));
