import { invoke } from '@tauri-apps/api/core';

export const Bridge = {
  logEvent: async (message: string): Promise<void> => {
    return invoke('log_event', { message });
  },
};
