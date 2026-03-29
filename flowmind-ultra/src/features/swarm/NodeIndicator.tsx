import { motion } from 'framer-motion';
import type { NodeStatus } from '../../stores/useSwarmStore';

export function NodeIndicator({ label, status, detail, isLast }: { label: string, status: NodeStatus, detail?: string, isLast?: boolean }) {
  
  const getColors = () => {
    switch(status) {
      case 'Active': return { bg: 'bg-[#a855f7]', border: 'border-[#a855f7]', shadow: 'shadow-[0_0_15px_#a855f7]', text: 'text-[#a855f7]' };
      case 'Complete': return { bg: 'bg-[#22d3ee]', border: 'border-[#22d3ee]', shadow: 'shadow-[0_0_10px_#22d3ee]', text: 'text-[#22d3ee]' };
      case 'Failed': return { bg: 'bg-red-500', border: 'border-red-500', shadow: 'shadow-[0_0_10px_red]', text: 'text-red-500' };
      default: return { bg: 'bg-slate-700', border: 'border-slate-700', shadow: '', text: 'text-slate-400' };
    }
  };

  const colors = getColors();

  return (
    <div className="flex items-center">
      <div className="flex flex-col items-center">
        <motion.div 
          initial={false}
          animate={{
            scale: status === 'Active' ? [1, 1.2, 1] : 1,
            opacity: status === 'Idle' ? 0.3 : 1
          }}
          transition={{ repeat: status === 'Active' ? Infinity : 0, duration: 1.5 }}
          className={`w-4 h-4 rounded-full ${colors.bg} ${colors.shadow} mb-2 border-2 ${colors.border}`}
        />
        <span className={`text-[10px] font-mono tracking-wider uppercase ${colors.text} whitespace-nowrap`}>{label}</span>
        <div className="h-4 mt-1 flex items-center justify-center">
            {detail && <span className="text-[8px] text-slate-500 max-w-[60px] truncate text-center" title={detail}>{detail}</span>}
        </div>
      </div>
      
      {!isLast && (
        <div className="w-6 sm:w-10 md:w-16 h-0.5 mx-2 bg-slate-800 relative z-[-1] flex items-center shrink-0">
          {status === 'Complete' && (
            <motion.div 
              initial={{ width: 0 }}
              animate={{ width: '100%' }}
              className="absolute left-0 h-full bg-[#22d3ee] shadow-[0_0_5px_#22d3ee]"
            />
          )}
          {status === 'Active' && (
             <motion.div 
               animate={{ x: ['0%', '200%'] }}
               transition={{ repeat: Infinity, duration: 1, ease: 'linear' }}
               className="h-1 w-2 rounded-full bg-[#a855f7] shadow-[0_0_8px_#a855f7]"
             />
          )}
        </div>
      )}
    </div>
  );
}
