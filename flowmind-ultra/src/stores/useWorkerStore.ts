import { create } from 'zustand';
import { listen } from '@tauri-apps/api/event';

export interface WorkerTask {
  id: string;
  title: string;
  files: string[];
  status: string;
}

export interface WorkerState {
  workerId: string;
  task: WorkerTask | null;
  state: string;
}

interface WorkerStore {
  workers: Record<string, WorkerState>;
  queueDepth: number;
  manualOverride: boolean;
  setManualOverride: (override: boolean) => Promise<void>;
  updateWorker: (workerId: string, task: WorkerTask | null, state: string) => void;
  setQueueDepth: (depth: number) => void;
  initListeners: () => void;
}

export const useWorkerStore = create<WorkerStore>((set) => ({
  workers: {
    'W1': { workerId: 'W1', task: null, state: 'Idle' },
    'W2': { workerId: 'W2', task: null, state: 'Idle' },
    'W3': { workerId: 'W3', task: null, state: 'Idle' },
  },
  queueDepth: 0,
  manualOverride: false,
  setManualOverride: async (override) => {
    set({ manualOverride: override });
    await import('../core/ipc/bridge').then(m => m.Bridge.toggleWorkerOverride(override));
  },
  updateWorker: (workerId, task, state) => set((prev) => ({
    workers: {
      ...prev.workers,
      [workerId]: { workerId, task, state }
    }
  })),
  setQueueDepth: (depth) => set({ queueDepth: depth }),
  initListeners: async () => {
    await listen<any>('workers_status', (event) => {
      const { worker_id, task, state } = event.payload;
      set((prev) => ({
        workers: {
          ...prev.workers,
          [worker_id]: { workerId: worker_id, task, state }
        }
      }));
    });
    await listen<any>('queue_depth', (event) => {
      const { depth } = event.payload;
      set({ queueDepth: depth });
    });
  }
}));
