import { create } from 'zustand';
import { Bridge } from '../core/ipc/bridge';

export interface WorkspaceConfig {
  id: string;
  name: string;
  path: string;
}

interface WorkspaceState {
  currentWorkspace: WorkspaceConfig | null;
  loadWorkspace: (id: string) => Promise<void>;
  saveWorkspace: (config: WorkspaceConfig) => Promise<void>;
}

export const useWorkspaceStore = create<WorkspaceState>((set) => ({
  currentWorkspace: null,
  loadWorkspace: async (id) => {
    const data = await Bridge.workspaceRead(id);
    if (data) {
      set({ currentWorkspace: JSON.parse(data) });
    }
  },
  saveWorkspace: async (config) => {
    await Bridge.workspaceSave(config.id, JSON.stringify(config));
    set({ currentWorkspace: config });
  },
}));
