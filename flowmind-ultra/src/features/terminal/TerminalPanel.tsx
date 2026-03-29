import { useEffect, useRef, useState } from 'react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { Bridge } from '../../core/ipc/bridge';
import { useWorkspaceStore } from '../../stores/useWorkspaceStore';
import '@xterm/xterm/css/xterm.css';

interface PTYPayload {
  id: string;
  data: number[];
}

interface TerminalProps {
  overrideId?: string;
  hideHeader?: boolean;
}

export function TerminalPanel({ overrideId, hideHeader }: TerminalProps = {}) {
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<Terminal | null>(null);
  const fitAddonRef = useRef<FitAddon | null>(null);
  const sessionId = overrideId || 'primary-pty';
  const { currentWorkspace } = useWorkspaceStore();
  const [terminalState, setTerminalState] = useState<string>('Offline');
  const [activeTab, setActiveTab] = useState<'Primary' | 'Compiler'>('Primary');
  const compilerBufferRef = useRef<string>('');

  useEffect(() => {
    const isWorkerTerminal = sessionId.startsWith('worker-');
    if (!isWorkerTerminal && !currentWorkspace) return;
    if (!terminalRef.current) return;

    const term = new Terminal({
      theme: {
        background: '#0a0a0f',
        foreground: '#cbd5e1',
        cursor: '#22d3ee',
        selectionBackground: 'rgba(34, 211, 238, 0.3)',
      },
      fontFamily: '"Fira Code", monospace',
      fontSize: 12,
      cursorBlink: true,
    });

    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(terminalRef.current);
    fitAddon.fit();

    xtermRef.current = term;
    fitAddonRef.current = fitAddon;

    let unlisten: UnlistenFn | null = null;
    let unlistenCompiler: UnlistenFn | null = null;

    const initTerminal = async () => {
      unlisten = await listen<PTYPayload>('pty-output', (event) => {
        if (event.payload.id !== sessionId) return;
        const charData = new Uint8Array(event.payload.data);
        if (activeTab === 'Primary') {
            term.write(charData);
        }
        setTerminalState('Working');
      });

      unlistenCompiler = await listen<{ data: string }>('compiler_diagnostics_stream', (event) => {
        compilerBufferRef.current += event.payload.data + '\r\n';
        if (activeTab === 'Compiler') {
            term.write(event.payload.data.replace(/\n/g, '\r\n') + '\r\n');
        }
      });

      try {
        if (!isWorkerTerminal && currentWorkspace) {
          const path = currentWorkspace.path.replace('file://', '');
          await Bridge.terminalCreate(sessionId, path);
        }
        term.writeln('\x1b[36m[Flowmind Ultra Matrix Ready]\x1b[0m');
        setTerminalState('Online');
      } catch (err: any) {
        term.writeln(`\x1b[31m[Critical Error: ${err.message || String(err)}]\x1b[0m`);
      }
    };

    initTerminal();

    term.onData((data: string) => {
      const encoder = new TextEncoder();
      const bytes = Array.from(encoder.encode(data));
      Bridge.terminalWrite(sessionId, bytes).catch(console.error);
    });

    const handleResize = () => {
      if (fitAddonRef.current && xtermRef.current) {
        fitAddonRef.current.fit();
        const dims = fitAddonRef.current.proposeDimensions();
        if (dims) {
          Bridge.terminalResize(sessionId, dims.rows, dims.cols).catch(console.error);
        }
      }
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      if (unlisten) unlisten();
      if (unlistenCompiler) unlistenCompiler();
      Bridge.terminalClose(sessionId).catch(console.error);
      term.dispose();
    };
  }, [sessionId, currentWorkspace]);

  return (
    <div className="flex flex-col h-full bg-[#0a0a0f] relative w-full">
      {!hideHeader && (
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
          <span className="text-[#22d3ee] font-bold tracking-wider uppercase flex-1">
            Agent Terminal: {terminalState}
          </span>
          <div className="flex gap-2">
            <button 
              onClick={() => { setActiveTab('Primary'); xtermRef.current?.clear(); }}
              className={`px-3 py-1 rounded border-b-2 text-xs font-bold transition-all ${activeTab === 'Primary' ? 'text-[#22d3ee] border-[#22d3ee] bg-[#22d3ee]/10' : 'text-slate-500 border-transparent hover:text-slate-300'}`}
            >
              PTY Output
            </button>
            <button 
              onClick={() => { setActiveTab('Compiler'); xtermRef.current?.clear(); xtermRef.current?.write(compilerBufferRef.current); }}
              className={`px-3 py-1 rounded border-b-2 text-xs font-bold transition-all ${activeTab === 'Compiler' ? 'text-amber-500 border-amber-500 bg-amber-500/10' : 'text-slate-500 border-transparent hover:text-slate-300'}`}
            >
              Compiler Output
            </button>
          </div>
        </div>
      )}
      <div className="flex-1 w-full overflow-hidden relative">
        <div ref={terminalRef} className="absolute inset-0 p-2" />
      </div>
    </div>
  );
}
