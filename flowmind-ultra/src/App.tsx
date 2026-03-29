import { TerminalPanel } from './features/terminal/TerminalPanel';
import { FileTree } from './features/workspace/FileTree';
import { ChatGraph } from './features/chat/ChatGraph';

function App() {
  return (
    <div className="grid grid-cols-[250px_1fr_350px] h-screen w-screen overflow-hidden text-slate-300">
      <div className="border-r border-panel-border overflow-y-auto w-full bg-[#16171d]">
        <FileTree />
      </div>
      <div className="flex flex-col h-full bg-[#0a0a0f] overflow-hidden">
        <TerminalPanel />
      </div>
      <div className="border-l border-panel-border overflow-y-auto w-full bg-[#16171d]">
        <ChatGraph />
      </div>
    </div>
  );
}

export default App;
