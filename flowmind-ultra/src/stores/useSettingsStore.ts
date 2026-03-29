import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface SettingsState {
  isVoiceGrammarFixEnabled: boolean;
  isAutoDeployVoiceEnabled: boolean;
  isVectorMemoryEnabled: boolean;
  whisperModelSize: 'tiny.en' | 'base.en' | 'small.en';
  
  setVoiceGrammarFix: (enabled: boolean) => void;
  setAutoDeployVoice: (enabled: boolean) => void;
  setVectorMemory: (enabled: boolean) => void;
  setWhisperModelSize: (size: 'tiny.en' | 'base.en' | 'small.en') => void;
}

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      isVoiceGrammarFixEnabled: true,
      isAutoDeployVoiceEnabled: false,
      isVectorMemoryEnabled: true,
      whisperModelSize: 'base.en',

      setVoiceGrammarFix: (enabled) => set({ isVoiceGrammarFixEnabled: enabled }),
      setAutoDeployVoice: (enabled) => set({ isAutoDeployVoiceEnabled: enabled }),
      setVectorMemory: (enabled) => set({ isVectorMemoryEnabled: enabled }),
      setWhisperModelSize: (size) => set({ whisperModelSize: size }),
    }),
    {
      name: 'flowmind-settings',
    }
  )
);
