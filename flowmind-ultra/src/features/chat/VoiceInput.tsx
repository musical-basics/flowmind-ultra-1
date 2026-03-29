import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { Mic, Loader2 } from 'lucide-react';
import { useVoiceStore } from '../../stores/useVoiceStore';

interface VoiceInputProps {
  onTranscription: (text: string) => void;
}

export function VoiceInput({ onTranscription }: VoiceInputProps) {
  const { isRecording, isTranscribing, transcript, volumeLevel } = useVoiceStore();
  
  useEffect(() => {
    let unlisten: () => void;
    let unmounted = false;

    const setupTelemetry = async () => {
      unlisten = await listen<number>('dictation_volume_level', (event) => {
        if (!unmounted) {
            useVoiceStore.setState({ volumeLevel: event.payload });
        }
      });
    };

    setupTelemetry();

    return () => {
      unmounted = true;
      if (unlisten) unlisten();
    };
  }, []);

  const handleMouseDown = async () => {
    try {
      if (isTranscribing) return;
      useVoiceStore.setState({ isRecording: true });
      await invoke('start_voice_dictation');
    } catch (e) {
      console.error("Failed to start dictation", e);
      useVoiceStore.setState({ isRecording: false });
    }
  };

  const handleMouseUp = async () => {
    try {
      if (!useVoiceStore.getState().isRecording) return;
      useVoiceStore.setState({ isRecording: false, isTranscribing: true });
      const text = await invoke<string>('stop_and_transcribe_audio');
      useVoiceStore.setState({ transcript: text, isTranscribing: false });
      if (text.trim()) {
        onTranscription(text);
      }
    } catch (e) {
      console.error("Failed to stop dictation", e);
      useVoiceStore.setState({ isRecording: false, isTranscribing: false });
    }
  };

  const scaleFactor = 1 + (volumeLevel * 5); // RMS mapped to scale

  return (
    <div className="relative flex items-center justify-center">
      {isRecording && (
        <div 
          className="absolute inset-0 bg-[#a855f7] rounded-full opacity-30 transition-transform duration-75 pointer-events-none"
          style={{ transform: `scale(${scaleFactor})` }}
        />
      )}
      <button
        onMouseDown={handleMouseDown}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
        disabled={isTranscribing}
        className={`relative z-10 p-3 rounded-full transition-all duration-200 shadow-lg border-2 ${
            isTranscribing 
                ? 'bg-slate-700 border-slate-600 text-slate-400 cursor-wait'
                : isRecording 
                    ? 'bg-[#a855f7] border-[#d8b4fe] text-white shadow-[0_0_15px_#a855f7]' 
                    : 'bg-slate-800 border-slate-700 text-[#22d3ee] hover:bg-slate-700 hover:border-[#22d3ee]/50'
        }`}
      >
        {isTranscribing ? <Loader2 className="w-5 h-5 animate-spin" /> : <Mic className="w-5 h-5" />}
      </button>
    </div>
  );
}
