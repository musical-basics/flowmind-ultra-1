import { invoke } from '@tauri-apps/api/core';

export const Bridge = {
  logEvent: async (message: string): Promise<void> => {
    return invoke('log_event', { message });
  },
  workspaceSave: async (id: string, config: string): Promise<void> => {
    return invoke('workspace_save', { id, config });
  },
  workspaceRead: async (id: string): Promise<string | null> => {
    return invoke('workspace_read', { id });
  },
  chatSaveMessage: async (id: string, payload: string): Promise<void> => {
    return invoke('chat_save_message', { id, payload });
  },
  outboxEnqueue: async (payload: string): Promise<string> => {
    return invoke('outbox_enqueue', { payload });
  },
  terminalCreate: async (id: string, cwd: string): Promise<void> => {
    return invoke('terminal_create', { id, cwd });
  },
  terminalWrite: async (id: string, data: number[]): Promise<void> => {
    return invoke('terminal_write', { id, data });
  },
  terminalResize: async (id: string, rows: number, cols: number): Promise<void> => {
    return invoke('terminal_resize', { id, rows, cols });
  },
  terminalClose: async (id: string): Promise<void> => {
    return invoke('terminal_close', { id });
  },
  llmFetchModels: async (apiKey: string): Promise<any> => {
    return invoke('fetch_models', { apiKey });
  },
  startSwarm: async (
    workspaceDir: string, 
    prompt: string, 
    overseerModel: string, 
    plannerModel: string, 
    executorModel: string
  ): Promise<void> => {
    return invoke('start_swarm', { 
      workspaceDir, 
      prompt, 
      overseerModel, 
      plannerModel, 
      executorModel 
    });
  },
  approveCommanderPlan: async (): Promise<void> => {
    return invoke('approve_commander_plan');
  },
  toggleWorkerOverride: async (paused: boolean): Promise<void> => {
    return invoke('set_worker_override', { paused });
  },
};
