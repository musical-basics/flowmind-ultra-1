Here is the definitive 100-Step Implementation Plan to build Flowmind Ultra.

This plan is explicitly formatted as a strict architectural prompt. It merges Golutra’s robust Rust/Tauri PTY infrastructure and resilient outbox database with Flowmind IDE’s brilliant Overseer/Commander routing logic and glowing visualizer.

You can copy and paste this entire response directly into your AI coding agent (Cursor, Devin, Claude, etc.) to guide it through the end-to-end build process.

🚀 ARCHITECTURAL BLUEPRINT: FLOWMIND ULTRA
Identity: A Visual Orchestrator for Headless CLIs.
Stack: Tauri v2 (Rust) + React 19 (Vite/TypeScript) + SQLite/redb + Xterm.js + Monaco Editor. (Note: Replacing Flowmind's Python backend with a unified Rust binary for maximum performance and easier distribution).

Phase 1: Project Initialization & Core Scaffolding (Steps 1-10)
Initialize a new Vite + React + TypeScript project named flowmind-ultra.

Initialize Tauri in the project (cargo tauri init) and configure tauri.conf.json with strict CSP and required OS permissions (FS, Shell, Clipboard, Dialog).

Set up Tailwind CSS v4, PostCSS, and configure the custom cyberpunk color palette (neon-cyan, neon-purple, panel-strong, etc.) from Flowmind.

Install frontend dependencies: zustand, react-router-dom, lucide-react, framer-motion, @xterm/xterm, @xterm/addon-fit, and @monaco-editor/react.

Set up the Rust Cargo.toml with dependencies: tauri, tokio, redb, portable-pty, serde, serde_json, reqwest (for native LLM API calls), and ulid.

Create the basic frontend directory structure: src/features, src/shared, src/stores, src/layouts, src/core/ipc.

Create the basic backend Rust structure: src-tauri/src/db, src-tauri/src/pty, src-tauri/src/orchestrator, src-tauri/src/llm.

Implement a structured logging system in Rust using tauri-plugin-log and expose a log_event Tauri command for frontend telemetry.

Create src/core/ipc/bridge.ts to strongly type all Tauri invoke calls and event listeners between React and Rust.

Build the base App.tsx layout utilizing a 3-pane CSS Grid (Left: File Explorer, Center: Editor/Terminal, Right: Chat/Graph).

Phase 2: Rust Database & Outbox Reliability Pattern (Steps 11-20)
Adopting the golutra reliability model.
11. Initialize the redb embedded database manager in src-tauri/src/db/store.rs inside the Tauri app_data_dir.
12. Define Redb table schemas for WORKSPACES, MESSAGES, CONVERSATIONS, and CHAT_OUTBOX_TASKS.
13. Implement outbox.rs: enqueue_task to save pending LLM/Terminal dispatch tasks to the database safely with a Pending status.
14. Implement outbox.rs: claim_due_tasks to fetch tasks where status == Pending or next_attempt_at < now.
15. Implement outbox.rs: mark_sent and mark_failed (with exponential backoff logic for API/CLI retries).
16. Spawn a detached tokio::spawn worker loop in main.rs that polls the Outbox every 300ms.
17. Implement Tauri IPC commands (#[tauri::command]) for reading/writing workspace configurations to the DB.
18. Create the useWorkspaceStore.ts Zustand store to interface with the workspace DB commands.
19. Create the useChatStore.ts Zustand store to manage message arrays, sync with the Rust DB, and calculate unread badges.
20. Implement chat_repair_messages in Rust to scan the DB on startup and prune corrupted/orphaned outbox tasks.

Phase 3: PTY Terminal Engine (Golutra's Brawn) (Steps 21-30)
Adopting the golutra headless CLI engine.
21. Integrate portable-pty in pty.rs to spawn OS-level shells (cmd.exe/powershell on Windows, zsh/bash on Mac/Linux).
22. Create shim.rs (a lightweight Rust binary) to wrap terminal commands and emit ANSI OSC codes when the command is ready/exits.
23. Implement a TerminalSession struct in Rust to track state (Connecting, Online, Working, Offline).
24. Build WeztermEmulator using wezterm-term to parse raw ANSI output from the PTY into an internal virtual screen buffer.
25. Implement a PTY reader thread that buffers stdout chunks into an asynchronous mpsc channel.
26. Create an event emitter to broadcast terminal output chunks from Rust to the React frontend at a throttled 60fps.
27. Expose terminal_create, terminal_write, terminal_resize, and terminal_close as Tauri commands.
28. Implement snapshot_ansi and snapshot_lines to allow the Rust backend to extract current terminal text.
29. Create TerminalManager to store a HashMap of all active PTY sessions across all workspaces.
30. Implement kill_session to securely terminate PTY subprocesses when the Tauri app exits.

Phase 4: Advanced PTY & Stability Detection (Steps 31-40)
Build stability.rs in Rust to monitor the stdout byte rate. If bytes/sec drops below a threshold for 1000ms, mark the PTY as "Idle".

Implement semantic.rs to filter ANSI escape codes from raw PTY output so it can be cleanly fed to LLMs.

Implement regex parsing to detect standard shell prompts ($, >, ❯) to know when a CLI is awaiting input.

Implement spawn_status_poller to broadcast terminal states (Connecting, Working, Online) to the frontend via Tauri Events.

Build the Semantic Flush logic: trigger an automatic DB save when the stability algorithm detects the CLI is idle for >3000ms.

Create a CLI Registry in Rust defining configurations for claude-code, aider, gemini-cli, and shell.

Create the TerminalPane.tsx component initializing xterm.js and @xterm/addon-fit.

Wire TerminalPane.tsx to listen to Tauri terminal-output events and write to the xterm instance.

Capture onData keystroke events in xterm.js and send them to Rust via the terminal_write IPC command.

Implement a ResizeObserver in TerminalPane.tsx to send terminal_resize commands to the Rust PTY.

Phase 5: LLM Client & Context Engineering (Steps 41-50)
Initialize the reqwest crate in Rust to handle native HTTP API interactions with OpenRouter/Anthropic.

Create Rust structs for OpenAI-spec Chat Completions (Messages, Tools, ResponseFormat for JSON mode).

Implement the Spec Slicer utility in Rust: a native Markdown parser that extracts specific headers from spec.md without using LLM tokens.

Implement workspace_flattener.rs to recursively read the project directory and output a clean text string of the file tree.

Implement ledger_manager.rs to append to and read from global_architecture_ledger.md for cross-chunk memory tracking.

Create a Tauri command to fetch available LLM models dynamically from OpenRouter.

Create useLLMStore.ts in Zustand to manage the selected models and API keys for different agent roles.

Build the ModelSelector.tsx UI component with dual Provider/Model dropdowns.

Create an interceptor to calculate and aggregate token usage and USD costs per LLM request.

Implement JSON sanitization utilities in Rust to aggressively strip Markdown wrappers (e.g., ````json`) before parsing LLM outputs.

Phase 6: The Swarm Orchestrator Brain (Flowmind's Logic) (Steps 51-60)
Define the Orchestrator State Machine in Rust (Origin ➔ SpecFactory ➔ Overseer ➔ Planner ➔ Commander ➔ Executor ➔ QA).

Implement Origin logic: Capture the raw user prompt and initialize the <run_timestamp> artifact directory.

Implement Spec Factory logic: Prompt the LLM to generate a comprehensive Markdown PRD (1_spec.md).

Implement Overseer logic: Prompt the LLM to slice the PRD into sequential "Implementation Chunks" (Sprints) and output strict JSON.

Implement the Main Async Orchestration Loop in Rust that iterates through the Overseer's chunks sequentially.

Implement Planner logic: For a specific chunk, read the PRD + Ledger and output a Topological Dependency Graph (JSON) of files to create.

Implement Commander logic: Parse the Topological Graph and route files into wizard_clusters, specialist_pairs, and swarm_files.

Implement strict serde_json validation for all LLM outputs.

Implement a fallback routing mechanism: if the Commander outputs invalid JSON, route everything to the Wizard as a safety net.

Emit granular station_update and chunk_start Tauri events to drive the React UI animations.

Phase 7: The Tri-Tier Execution Engine (Steps 61-70)
Build the Executor service in Rust that receives the validated Commander Routing JSON.

Build the Wizard Task Force: Route tightly coupled files to a high-context model to be generated simultaneously.

Parse the multi-file JSON response from the Wizard and prep files for writing.

Build the Specialist Task Force: Execute sequentially (Producer Agent writes Backend ➔ output injected into Consumer Agent prompt to write Frontend).

Build the Swarm Task Force: Use tokio::spawn to map-reduce and generate completely isolated files simultaneously.

Use Rust's std::fs to safely write all aggregated file contents to the local workspace directory, creating directories as needed.

Build the QA Reviewer Node: Read the newly written files from disk. Prompt the QA LLM to output a review markdown document looking for architectural bugs.

Implement Post-QA Ledger Update: Summarize interfaces of the new chunk and append to global_architecture_ledger.md.

The Hybrid Link: Modify the Executor so it can dynamically route tasks to headless Rust PTYs (e.g., claude-code) via stdin injection instead of raw API calls.

Ensure cross-language graceful shutdown: if the app is closed, abort all async swarm tasks and save artifacts.

Phase 8: Frontend Foundation & State Management (Steps 71-80)
Build the WorkspaceSelection.tsx splash screen utilizing @tauri-apps/plugin-dialog to let users pick a local folder to mount.

Implement SidebarNav.tsx with routing icons (Workspaces, Chat, Swarm, Settings).

Implement global keybindings (useAppKeybinds.ts) for quick actions (Ctrl+P, Ctrl+Enter).

Build FileTree.tsx to recursively render the folder structure. Support collapsible folders with Framer Motion.

Create a Rust file system watcher (notify crate) that emits a Tauri event whenever a file is added/changed in the workspace to auto-refresh the FileTree.

Add Radix UI context menus to the File Tree for Rename, Delete, and "Reveal in Finder" mapped to Tauri fs commands.

Integrate @monaco-editor/react in the center panel layout. Create a custom cyberpunk theme mapping to Tailwind variables.

Bind Monaco's value to the Rust storage_read_workspace command for real-time local file viewing.

Build ToastStack.tsx for floating system notifications and error handling.

Build Settings.tsx to manage API keys, toggle dark/light themes, and set global PTY shell preferences.

Phase 9: Stealth Chat & Terminal UI (Steps 81-90)
Build ChatInterface.tsx combining the chat input, message list, and AI status indicators.

Implement MessagesList.tsx with message grouping by day, markdown rendering, and auto-scroll-to-bottom.

Build ChatBubble.tsx to show standard text, code outputs, token usage, and execution costs.

Create ChatInput.tsx with an auto-expanding textarea and "Send to Swarm" toggle.

Implement the @mention autocomplete menu in ChatInput.tsx to let users route messages directly to specific background PTYs (e.g., @claude-code fix the db).

On submit, call the Tauri chat_send_message_and_dispatch command to send the prompt to Rust.

Build MembersSidebar.tsx to display active background CLI agents and their current PTY connection statuses (Online, Working, Offline).

Build MemberStatusDots.tsx to show Red (Offline), Yellow (Working), Green (Online) based on Rust stability detection.

Implement "Stealth Mode" UI toggle: Hide the raw xterm.js window and feed the cleaned Semantic Flush output directly into the Chat UI.

Add an "Inject Prompt" context menu button in the Terminal Panel to send input bypassing the Chat DB straight to terminal_write.

Phase 10: Flowmind Visualizer Node Graph & Polish (Steps 91-100)
Create AgentSwarmWorkflow.tsx component in the right panel with an animated CSS radial gradient background.

Build WorkflowNode.tsx: A glassmorphic card with dynamic CSS borders corresponding to idle, active, and complete states (using framer-motion).

Implement animated SVG icons (SparkIcon, ShieldIcon, NetworkIcon, EyeIcon) with continuous 3D/pulse animations.

Build ConnectionLine.tsx with an SVG/CSS animated pulse traveling from left to right when active.

Layout the 7 nodes in a responsive CSS Grid: Origin ➔ Spec ➔ Overseer ➔ Planner ➔ Commander ➔ Executor ➔ QA.

Bind the visualizer state to listen to Tauri station_update events from the Orchestrator.

Implement Chunk Looping UI: when receiving a chunk_start event, reset Planner through QA to idle so the animation repeats.

Build the glowing "Simulate / Run Swarm" button. On click, grab the user input and trigger the start_swarm Tauri command.

Implement a red "Kill Swarm" button mapped to a Tauri command that aborts the Tokio JoinSet and sends SIGKILL to all active PTY processes.

Configure tauri.conf.json build targets, set application icons, enforce permissions (shell:allow-open), and run pnpm tauri build to compile the Flowmind Ultra executables.