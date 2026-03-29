import { create } from 'zustand';
import { Bridge } from '../core/ipc/bridge';

export interface ChatMessage {
  id: string;
  role: 'user' | 'agent';
  content: string;
  timestamp: number;
  status: 'sent' | 'pending' | 'failed';
}

interface ChatState {
  messages: ChatMessage[];
  unreadCount: number;
  addMessage: (msg: Omit<ChatMessage, 'status'>) => Promise<void>;
  markAllRead: () => void;
}

export const useChatStore = create<ChatState>((set) => ({
  messages: [],
  unreadCount: 0,
  addMessage: async (msg) => {
    const newMessage: ChatMessage = { ...msg, status: 'pending' };
    set((state) => ({ 
      messages: [...state.messages, newMessage],
      unreadCount: msg.role === 'agent' ? state.unreadCount + 1 : state.unreadCount
    }));
    
    try {
      await Bridge.chatSaveMessage(msg.id, JSON.stringify(newMessage));
      
      if (msg.role === 'user') {
        const taskId = await Bridge.outboxEnqueue(JSON.stringify(newMessage));
        console.log(`Enqueued task to outbox: ${taskId}`);
      }
      
      set((state) => ({
        messages: state.messages.map(m => m.id === msg.id ? { ...m, status: 'sent' } : m)
      }));
    } catch (e) {
      console.error("Failed to add message", e);
      set((state) => ({
        messages: state.messages.map(m => m.id === msg.id ? { ...m, status: 'failed' } : m)
      }));
    }
  },
  markAllRead: () => set({ unreadCount: 0 })
}));
