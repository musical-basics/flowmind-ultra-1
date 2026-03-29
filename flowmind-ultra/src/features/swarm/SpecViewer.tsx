import { useSwarmStore } from '../../stores/useSwarmStore';

export function SpecViewer() {
  const { prdMarkdown } = useSwarmStore();

  if (!prdMarkdown) {
    return null;
  }

  return (
    <div className="flex flex-col text-xs font-mono p-4 bg-[#0a0a0f] border border-[#a855f7]/30 rounded flex-1 min-h-[300px] overflow-y-auto shadow-inner shadow-black">
      <div className="text-[#a855f7] font-bold uppercase border-b border-[#a855f7]/20 pb-2 mb-4 shrink-0 top-0 sticky bg-[#0a0a0f] z-10">
        Generated PRD Specification
      </div>
      <div className="text-slate-300 whitespace-pre-wrap font-sans text-sm leading-relaxed">
        {prdMarkdown}
      </div>
    </div>
  );
}
