import { create } from 'zustand';

export interface CommitNode {
    timestamp: number;
    message: string;
}

interface TimeTravelStore {
  timeline: CommitNode[];
  currentScrubIndex: number;
}

export const useTimeTravelStore = create<TimeTravelStore>(() => ({
  timeline: [],
  currentScrubIndex: 0,
}));
