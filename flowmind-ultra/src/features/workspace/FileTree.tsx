import { useState } from 'react';
import { useWorkerStore } from '../../stores/useWorkerStore';
import { listWorkspaceEntries, useWorkspaceStore, type WorkspaceEntry } from '../../stores/useWorkspaceStore';
import { ChevronRight, FileText, Folder, FolderOpen } from 'lucide-react';

export function FileTree() {
  const { workers } = useWorkerStore();
  const { currentWorkspace, entries, openWorkspace } = useWorkspaceStore();
  const [expandedPaths, setExpandedPaths] = useState<Record<string, boolean>>({});
  const [nestedEntries, setNestedEntries] = useState<Record<string, WorkspaceEntry[]>>({});

  const lockedFiles = new Set<string>();
  Object.values(workers).forEach(w => {
    if (w.task) {
      w.task.files.forEach(f => lockedFiles.add(f));
    }
  });

  const displayEntries = entries.length > 0
    ? entries
    : [
        { name: 'src', path: 'src', isDirectory: true },
        { name: 'src/main.rs', path: 'src/main.rs', isDirectory: false },
        { name: 'src/App.tsx', path: 'src/App.tsx', isDirectory: false },
        { name: 'Cargo.toml', path: 'Cargo.toml', isDirectory: false },
        { name: 'package.json', path: 'package.json', isDirectory: false },
      ];

  const toggleDirectory = async (entry: WorkspaceEntry) => {
    if (!entry.isDirectory) {
      return;
    }

    const isExpanded = expandedPaths[entry.path];
    if (!isExpanded && !nestedEntries[entry.path]) {
      const children = await listWorkspaceEntries(entry.path);
      setNestedEntries((current) => ({
        ...current,
        [entry.path]: children,
      }));
    }

    setExpandedPaths((current) => ({
      ...current,
      [entry.path]: !isExpanded,
    }));
  };

  const renderEntry = (entry: WorkspaceEntry, depth = 0) => {
    const isLocked = !entry.isDirectory && lockedFiles.has(entry.path);
    const isExpanded = !!expandedPaths[entry.path];
    const children = nestedEntries[entry.path] ?? [];

    return (
      <div key={entry.path} className="flex flex-col">
        <button
          type="button"
          onClick={() => void toggleDirectory(entry)}
          className={`flex items-center gap-2 px-2 py-1 rounded transition-colors text-left ${isLocked ? 'bg-red-500/10 border border-red-500/30' : 'hover:bg-[#1f2028]'}`}
          style={{ paddingLeft: `${8 + depth * 14}px` }}
        >
          {entry.isDirectory ? (
            <ChevronRight className={`w-3 h-3 shrink-0 text-slate-500 transition-transform ${isExpanded ? 'rotate-90' : ''}`} />
          ) : (
            <span className="w-3 shrink-0" />
          )}

          {entry.isDirectory ? (
            <Folder className="w-3.5 h-3.5 shrink-0 text-[#a855f7]" />
          ) : (
            <FileText className="w-3.5 h-3.5 shrink-0 text-slate-400" />
          )}

          <span className={`truncate ${isLocked ? 'text-red-400 font-bold' : entry.isDirectory ? 'text-[#c084fc] font-semibold' : 'text-slate-300'}`}>
            {entry.name}
          </span>

          {isLocked && <span className="text-[10px] ml-auto animate-pulse shrink-0">🔒</span>}
        </button>

        {entry.isDirectory && isExpanded && children.length > 0 && (
          <div className="flex flex-col gap-1">
            {children.map((child) => renderEntry(child, depth + 1))}
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="flex flex-col p-4 font-mono text-xs gap-2">
      <div className="text-slate-500 uppercase font-bold tracking-wider mb-2 border-b border-[#2e303a] pb-1">
        Workspace Source
      </div>

      <button
        onClick={openWorkspace}
        className="flex items-center gap-2 px-3 py-2 rounded bg-[#a855f7]/10 border border-[#a855f7]/30 text-[#a855f7] hover:bg-[#a855f7]/20 hover:border-[#a855f7]/60 transition-all text-[11px] font-bold uppercase tracking-wider w-full"
      >
        <FolderOpen className="w-3.5 h-3.5 shrink-0" />
        {currentWorkspace ? currentWorkspace.name : 'Open Workspace'}
      </button>

      <div className="flex flex-col gap-1 mt-1">
        {displayEntries.map((entry) => renderEntry(entry))}
      </div>
    </div>
  );
}

