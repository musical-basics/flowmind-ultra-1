import { create } from 'zustand';

interface VoiceStore {
  isRecording: boolean;
  isTranscribing: boolean;
  transcript: string | null;
  volumeLevel: number;
}

export const useVoiceStore = create<VoiceStore>(() => ({
  isRecording: false,
  isTranscribing: false,
  transcript: null,
  volumeLevel: 0,
}));
