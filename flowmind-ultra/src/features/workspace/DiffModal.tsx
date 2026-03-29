import { DiffEditor } from '@monaco-editor/react';
import { motion, AnimatePresence } from 'framer-motion';
import { X, Save, RotateCcw } from 'lucide-react';

interface DiffModalProps {
  isOpen: boolean;
  onClose: () => void;
  original: string;
  modified: string;
  fileName: string;
  onConfirm: () => void;
}

export function DiffModal({ isOpen, onClose, original, modified, fileName, onConfirm }: DiffModalProps) {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center p-8 bg-black/60 backdrop-blur-sm">
      <motion.div 
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.95 }}
        className="bg-[#1e1e2e] border border-slate-700 rounded-xl w-full h-full max-w-6xl flex flex-col overflow-hidden shadow-2xl"
      >
        <div className="h-14 flex items-center justify-between px-6 border-b border-slate-800 bg-[#1e1e2e]">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-purple-500/20 flex items-center justify-center">
              <RotateCcw className="w-4 h-4 text-purple-400" />
            </div>
            <div>
              <h3 className="text-sm font-bold text-slate-200">Temporal Synchronization</h3>
              <p className="text-[10px] text-slate-500 font-mono italic">{fileName}</p>
            </div>
          </div>
          <button onClick={onClose} className="p-2 hover:bg-slate-800 rounded-lg transition-colors">
            <X className="w-5 h-5 text-slate-400" />
          </button>
        </div>

        <div className="flex-1 bg-[#1e1e2e]">
          <DiffEditor
            height="100%"
            language="typescript" // Or dynamic based on file ext
            original={original}
            modified={modified}
            theme="vs-dark"
            options={{
              readOnly: true,
              renderSideBySide: true,
              minimap: { enabled: false },
              fontSize: 12,
              fontFamily: 'JetBrains Mono, Menlo, Monaco, Courier New, monospace',
            }}
          />
        </div>

        <div className="h-16 flex items-center justify-end px-6 border-t border-slate-800 gap-4 bg-[#161625]">
          <button 
            onClick={onClose}
            className="px-6 py-2 text-xs font-bold text-slate-400 hover:text-slate-200 uppercase tracking-widest transition-colors"
          >
            Cancel
          </button>
          <button 
            onClick={onConfirm}
            className="flex items-center gap-2 px-8 py-2 bg-purple-600 hover:bg-purple-500 text-white text-xs font-bold rounded-lg shadow-[0_0_20px_rgba(168,85,247,0.3)] transition-all uppercase tracking-widest"
          >
            <Save className="w-3.5 h-3.5" />
            Apply Temporal Reversion
          </button>
        </div>
      </motion.div>
    </div>
  );
}
