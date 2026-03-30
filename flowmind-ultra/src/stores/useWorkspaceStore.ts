import { create } from 'zustand';
import { Bridge } from '../core/ipc/bridge';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { readDir } from '@tauri-apps/plugin-fs';

export interface WorkspaceConfig {
  id: string;
  name: string;
  path: string;
}

export interface WorkspaceEntry {
  name: string;
  path: string;
  isDirectory: boolean;
}

interface WorkspaceState {
  currentWorkspace: WorkspaceConfig | null;
  entries: WorkspaceEntry[];
  openWorkspace: () => Promise<void>;
  loadWorkspace: (id: string) => Promise<void>;
  saveWorkspace: (config: WorkspaceConfig) => Promise<void>;
}

export async function listWorkspaceEntries(dirPath: string): Promise<WorkspaceEntry[]> {
  try {
    const entries = await readDir(dirPath);
    return entries
      .filter((entry) => entry.name)
      .map((entry) => ({
        name: entry.name as string,
        path: `${dirPath.replace(/\/$/, '')}/${entry.name}`,
        isDirectory: entry.isDirectory,
      }))
      .sort((left, right) => {
        if (left.isDirectory !== right.isDirectory) {
          return left.isDirectory ? -1 : 1;
        }

        return left.name.localeCompare(right.name);
      })
      .slice(0, 50);
  } catch {
    return [];
  }
}

export const useWorkspaceStore = create<WorkspaceState>((set) => ({
  currentWorkspace: null,
  entries: [],
  openWorkspace: async () => {
    const selected = await openDialog({ directory: true, multiple: false, title: 'Open Workspace Folder' });
    if (!selected || typeof selected !== 'string') return;

    const name = selected.split('/').pop() ?? selected;
    const config: WorkspaceConfig = { id: selected, name, path: selected };
    await Bridge.workspaceSave(config.id, JSON.stringify(config));

    const entries = await listWorkspaceEntries(selected);
    set({ currentWorkspace: config, entries });
  },
  loadWorkspace: async (id) => {
    const data = await Bridge.workspaceRead(id);
    if (data) {
      const config: WorkspaceConfig = JSON.parse(data);
      const entries = await listWorkspaceEntries(config.path);
      set({ currentWorkspace: config, entries });
    }
  },
  saveWorkspace: async (config) => {
    await Bridge.workspaceSave(config.id, JSON.stringify(config));
    const entries = await listWorkspaceEntries(config.path);
    set({ currentWorkspace: config, entries });
  },
}));
