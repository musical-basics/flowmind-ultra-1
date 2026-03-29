import { useEffect, useRef, useState } from 'react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { Bridge } from '../../core/ipc/bridge';
import '@xterm/xterm/css/xterm.css';

interface PTYPayload {
  id: string;
  data: number[];
}

interface TerminalStatePayload {
  id: string;
  state: string;
}

export function TerminalPanel() {
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<Terminal | null>(null);
  const fitAddonRef = useRef<FitAddon | null>(null);
  const sessionId = 'primary-pty';
  const [terminalState, setTerminalState] = useState<string>('Offline');

  useEffect(() => {
    if (!terminalRef.current) return;

    const term = new Terminal({
      theme: {
        background: '#0a0a0f',
        foreground: '#22d3ee',
        cursor: '#a855f7',
        selectionBackground: 'rgba(170, 59, 255, 0.3)',
      },
      fontFamily: 'ui-monospace, Consolas, monospace',
      fontSize: 14,
      cursorBlink: true,
    });

    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(terminalRef.current);
    
    // Fit addon requires a tiny timeout occasionally on initial mount in React strict mode
    setTimeout(() => fitAddon.fit(), 50);

    xtermRef.current = term;
    fitAddonRef.current = fitAddon;

    let unlistenOutput: UnlistenFn | null = null;
    let unlistenState: UnlistenFn | null = null;

    const setup = async () => {
      unlistenOutput = await listen<PTYPayload>('pty-output', (event) => {
        if (event.payload.id === sessionId) {
          term.write(new Uint8Array(event.payload.data));
        }
      });

      unlistenState = await listen<TerminalStatePayload>('pty-state', (event) => {
        if (event.payload.id === sessionId) {
          setTerminalState(event.payload.state);
        }
      });

      term.onData((data) => {
        const encoder = new TextEncoder();
        const ui8 = encoder.encode(data);
        Bridge.terminalWrite(sessionId, Array.from(ui8));
      });

      term.onResize((size) => {
        Bridge.terminalResize(sessionId, size.rows, size.cols);
      });

      // Request PTY creation in Rust
      await Bridge.terminalCreate(sessionId);
    };

    setup();

    const resizeObserver = new ResizeObserver(() => {
      try {
        fitAddon.fit();
      } catch (e) {}
    });
    resizeObserver.observe(terminalRef.current);

    return () => {
      resizeObserver.disconnect();
      if (unlistenOutput) unlistenOutput();
      if (unlistenState) unlistenState();
      
      // Cleanup daemon PTY in backend
      Bridge.terminalClose(sessionId);
      term.dispose();
    };
  }, []);

  return (
    <div className="flex flex-col h-full bg-[#0a0a0f] relative w-full">
      <div className="flex items-center gap-2 px-4 py-2 border-b border-[#22d3ee]/20 bg-[#0a0a0f] text-xs font-mono shrink-0">
        <span 
          className={`w-2 h-2 rounded-full ${
            terminalState === 'Working' 
              ? 'bg-[#a855f7] animate-pulse shadow-[0_0_8px_#a855f7]' 
              : terminalState === 'Online' 
                ? 'bg-[#22d3ee] shadow-[0_0_8px_#22d3ee]' 
                : 'bg-slate-600'
          }`}
        />
        <span className="text-[#22d3ee] font-bold tracking-wider uppercase">
          Agent Terminal: {terminalState}
        </span>
      </div>
      <div className="flex-1 w-full overflow-hidden relative">
        <div ref={terminalRef} className="absolute inset-0 p-2" />
      </div>
    </div>
  );
}
