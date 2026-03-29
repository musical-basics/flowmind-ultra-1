import { create } from 'zustand';
import { createClient } from '@supabase/supabase-js';

interface SupabaseStore {
  client: any | null;
  isCollaborative: boolean;
  activeRunId: string | null;
  
  initClient: (url: string, key: string) => void;
  syncRun: (workspaceId: string) => Promise<void>;
  remoteApprove: (workspaceId: string) => Promise<void>;
}

export const useSupabaseStore = create<SupabaseStore>((set, get) => ({
  client: null,
  isCollaborative: false,
  activeRunId: null,

  initClient: (url, key) => {
    if (!url || !key) return;
    const client = createClient(url, key, {
      db: { schema: 'flowmind' } // Set default schema to flowmind
    });
    set({ client, isCollaborative: true });
  },

  syncRun: async (workspaceId) => {
    const { client } = get();
    if (!client) return;

    // Initial fetch
    const { data } = await client
      .from('swarm_runs')
      .select('*')
      .eq('workspace_id', workspaceId)
      .single();

    if (data) {
      set({ activeRunId: data.id });
    }

    // Subscribe to Realtime
    client
      .channel('swarm_sync')
      .on(
        'postgres_changes',
        { event: '*', schema: 'flowmind', table: 'swarm_runs', filter: `workspace_id=eq.${workspaceId}` },
        (payload: any) => {
          console.log('Remote State Change:', payload);
          // Here we could trigger local store updates if needed
        }
      )
      .subscribe();
  },

  remoteApprove: async (workspaceId) => {
    const { client } = get();
    if (!client) return;

    await client
      .from('swarm_runs')
      .update({ is_commander_approved: true })
      .eq('workspace_id', workspaceId);
  }
}));
