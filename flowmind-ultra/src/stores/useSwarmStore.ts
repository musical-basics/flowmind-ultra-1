import { create } from 'zustand';
import { listen } from '@tauri-apps/api/event';

export type NodeStatus = 'Idle' | 'Active' | 'AwaitingApproval' | 'Complete' | 'Failed';

export interface SwarmStation {
  id: string;
  label: string;
  status: NodeStatus;
  detail?: string;
}

interface ChunkInfo {
  id: number;
  title: string;
  total: number;
}

interface SwarmState {
  stations: SwarmStation[];
  currentChunk: ChunkInfo | null;
  prdMarkdown: string | null;
  sprints: any[];
  graph: any | null;
  commanderPlan: any | null;
  updateStation: (station: string, status: NodeStatus, detail?: string) => void;
  setChunk: (chunk: ChunkInfo) => void;
  setPrd: (prd: string) => void;
  setSprints: (sprints: any[]) => void;
  setGraph: (graph: any) => void;
  setCommanderPlan: (plan: any) => void;
  initListeners: () => void;
}

const INITIAL_STATIONS: SwarmStation[] = [
  { id: 'Origin', label: 'Origin', status: 'Idle' },
  { id: 'SpecFactory', label: 'SpecFactory', status: 'Idle' },
  { id: 'Overseer', label: 'Overseer', status: 'Idle' },
  { id: 'Planner', label: 'Planner', status: 'Idle' },
  { id: 'Commander', label: 'Commander', status: 'Idle' },
  { id: 'Executor', label: 'Executor', status: 'Idle' },
  { id: 'QA', label: 'QA Review', status: 'Idle' },
];

export const useSwarmStore = create<SwarmState>((set, get) => ({
  stations: [...INITIAL_STATIONS],
  currentChunk: null,
  prdMarkdown: null,
  sprints: [],
  graph: null,
  commanderPlan: null,

  updateStation: (stationId, status, detail) => set((state) => ({
    stations: state.stations.map((s) => 
      s.id === stationId ? { ...s, status, detail } : s
    )
  })),

  setChunk: (chunk) => set({ currentChunk: chunk }),
  setPrd: (prd) => set({ prdMarkdown: prd }),
  setSprints: (sprints) => set({ sprints }),
  setGraph: (graph) => set({ graph }),
  setCommanderPlan: (plan) => set({ commanderPlan: plan }),

  initListeners: async () => {
    await listen<any>('station_update', (event) => {
      const { station, status, detail } = event.payload;
      get().updateStation(station, status as NodeStatus, detail);
    });

    await listen<any>('chunk_start', (event) => {
      const { chunk_id, chunk_title, total_chunks } = event.payload;
      get().setChunk({ id: chunk_id, title: chunk_title, total: total_chunks });
      
      set(state => ({
        stations: state.stations.map(s => 
          ['Planner', 'Commander', 'Executor', 'QA'].includes(s.id) 
            ? { ...s, status: 'Idle', detail: undefined } 
            : s
        )
      }));
    });
  }
}));
