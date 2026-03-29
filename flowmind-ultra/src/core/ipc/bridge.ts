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
};
