ARCHITECTURAL BLUEPRINT: FLOWMIND ULTRA (PHASE 11-20)

Target Audience: Autonomous AI Coding Agent (DeepThink / Claude / Devin / Cursor)
System Context: Tauri v2 (Rust) + React 19 (Zustand/Vite) + Redb + Tokio + Portable-PTY.
Mandate: Execute the following 100 strict architectural steps (101-200) to implement the four advanced Epics. Do not write the code; generate the structural scaffolding, specify the exact crates, detail the Tauri IPC bridges, and define the React/Zustand state topologies.

Epic 1: The Self-Healing Compiler Loop (QA 2.0) (Steps 101-125)
Objective: Upgrade the QA station from a passive markdown reviewer to an active, headless LSP/Compiler loop that automatically rewrites failing code.

Cargo.toml Update: Add the regex crate for high-performance stderr parsing and tokio-util for asynchronous PTY stream handling.

Rust Module Creation: Create src-tauri/src/workers/compiler.rs to manage headless compiler sessions, entirely isolated from the user-facing TerminalManager.

Headless PTY Wrapper: In compiler.rs, implement a CompilerSession struct using portable-pty. Configure it to run without PTY echo, strictly piping stderr and stdout into a Tokio BytesMut buffer.

Compiler Detection: Implement detect_compiler_env() in compiler.rs. Scan the workspace_dir for Cargo.toml (cargo check), tsconfig.json (tsc --noEmit), or requirements.txt (flake8) to dynamically resolve the build command.

Execution Routine: Create run_compiler_check() that spawns the detected compiler, awaits the exit code, and returns a Result<(), String> containing the raw stderr on failure.

Error Parsing: Implement a CompilerDiagnostics struct and use regex to parse the stderr buffer, extracting exact file_path, line_number, and error_message.

State Machine Update: In src-tauri/src/orchestrator/state.rs, update the SwarmState enum to include SelfHealing { attempt: u8, max_attempts: u8 }.

Loop Integration: In loop_runner.rs, intercept the transition after the Executor phase. Await run_compiler_check(). If exit code is 0, proceed to QA/Ledger. If >0, transition state to SelfHealing.

Sniper Prompt Assembly: Create src-tauri/src/llm/sniper.rs. Construct a strict prompt containing: 1) The failing file's current AST/code, 2) The parsed compiler error, 3) Instructions to output ONLY valid JSON containing the exact code replacement.

LLM Invocation: Call LlmClient::complete() using the executorSpecialist model profile to generate the patch.

Patch Sanitization: Pass the Sniper's output through sanitize_json() in src-tauri/src/llm/sanitizer.rs to strip markdown wrappers.

Lock Bypass: In src-tauri/src/workers/conflict.rs, implement a force_acquire_lock() method allowing the Sniper agent to bypass the standard queue and lock the broken file synchronously.

File Overwrite: Apply the JSON patch to the local file system via std::fs::write and immediately release the lock.

Recursive Healing: Recursively loop run_compiler_check(). Cap the recursion at max_attempts (default 3) to prevent infinite API spend.

Fallback Escalation: If max_attempts is breached, emit a station_update with status Failed, halting the Swarm and prompting human intervention.

Tauri IPC: Expose #[tauri::command] pub async fn manual_compiler_override(workspace_id: String) in orchestrator/commands.rs to let the user unblock the Swarm.

Frontend State: In src/stores/useSwarmStore.ts, update NodeStatus to accept 'Healing' and 'AwaitingHumanFix'.

UI Animation: In src/features/swarm/NodeIndicator.tsx, implement a CSS keyframe animation (pulsing amber with a wrench icon) for the Healing status.

Live Telemetry: Emit a Tauri event compiler_diagnostics_stream containing the raw stderr buffer.

Terminal UI: In src/features/terminal/TerminalPanel.tsx, add a hidden "Compiler Output" tab that tails the compiler_diagnostics_stream event when active.

Chat UI Notification: In src/features/chat/ChatGraph.tsx, inject a ChatBubble stating: "🚨 Compiler Error Detected in {file}. Sniper Agent deployed (Attempt {n}/3)..."

Success Broadcast: On successful compilation, emit a UI event: "✅ Compilation successful. Codebase stabilized."

Ledger Annotation: In src-tauri/src/llm/ledger.rs, append the resolved error and patch summary to global_architecture_ledger.md so the Planner avoids repeating the syntax error in future chunks.

Settings Toggle: Add a toggle in src/features/Settings.tsx to enable/disable the "Self-Healing Compiler Loop".

Command Registration: Register manual_compiler_override in the tauri::generate_handler! macro in main.rs.

Epic 2: Acoustic Command Interface (Vibecoding by Voice) (Steps 126-150)
Objective: Integrate offline, native voice-to-text directly into the Rust backend to bypass keyboard bottlenecks.

Cargo.toml Update: Add cpal (low-latency audio I/O), hound (WAV encoding), rubato (audio resampling), and whisper-rs (Whisper C++ bindings).

OS Permissions: Update tauri.conf.json to include macOS NSMicrophoneUsageDescription and Windows microphone manifests.

Audio Module: Create a new directory src-tauri/src/audio/ containing mod.rs, capture.rs, and transcribe.rs.

Model Initialization: In main.rs, write a startup hook that checks app_cache_dir() for ggml-base.en.bin. If missing, asynchronously download it via reqwest.

Context Setup: In transcribe.rs, initialize WhisperContext and WhisperState into a globally managed Arc<Mutex<>> on boot.

Audio Capture: In capture.rs, use cpal to build start_audio_stream(), targeting the default input device at 1 channel (mono), f32 format.

Ring Buffer: Pipe the cpal stream into a thread-safe crossbeam-channel or std::sync::mpsc buffer.

Resampling: Implement a Tokio background task that consumes the buffer, uses rubato to resample the audio to exactly 16kHz, and temporarily encodes it to WAV via hound.

Voice Activity Detection (VAD): Implement a simple RMS (Root Mean Square) amplitude calculator in the stream. If RMS drops below a threshold for 1.5s, trigger auto-stop.

Tauri IPC (Start): Expose #[tauri::command] pub async fn start_voice_dictation() -> Result<(), String>.

Tauri IPC (Stop): Expose #[tauri::command] pub async fn stop_and_transcribe_audio() -> Result<String, String>.

Inference Execution: In stop_and_transcribe_audio, pass the 16kHz PCM data to WhisperState::full(), extract the tokens, and concatenate the transcript.

Volume Telemetry: Emit a Tauri event dictation_volume_level at 30fps containing the f32 RMS value to drive frontend UI equalizers.

Frontend Store: Create src/stores/useVoiceStore.ts with state variables: isRecording, isTranscribing, transcript, and volumeLevel.

UI Component: Create src/features/chat/VoiceInput.tsx featuring a microphone button (using lucide-react).

Push-to-Talk Logic: Bind onMouseDown to start_voice_dictation and onMouseUp to stop_and_transcribe_audio.

Waveform Animation: In VoiceInput.tsx, map the dictation_volume_level event to CSS transform: scale() on a radial gradient behind the mic icon.

Origin Node Binding: Automatically append the resolved transcript string from Rust to the prompt state variable in SwarmDashboard.tsx.

Hardware Acceleration: Ensure whisper-rs is built with --features coreml on macOS and --features cuda on Windows in Cargo.toml.

IPC (Cancel): Expose #[tauri::command] pub async fn cancel_voice_dictation() to flush the buffer and abort inference.

Grammar Correction Pipeline: Create an optional sub-routine in transcribe.rs that pipes the raw Whisper output through a fast, local LLM prompt: "Format this raw dictation into a clean developer prompt."

Settings UI: Add a "Voice Model" selector in Settings.tsx to let users choose between tiny.en, base.en, or small.en models.

Auto-Deploy Toggle: Add a setting "Auto-Deploy Swarm on Voice Stop" to immediately trigger the orchestrator once transcription completes.

Resource Cleanup: Implement an impl Drop or hook into WindowEvent::Destroyed to ensure the cpal stream releases the OS microphone lock on app exit.

Command Registration: Register start_voice_dictation, stop_and_transcribe_audio, and cancel_voice_dictation in main.rs.

Epic 3: Vectorized Swarm Memory (Long-Term Context) (Steps 151-175)
Objective: Eliminate Swarm amnesia by persisting architectural decisions in a local vector database for semantic retrieval (RAG).

Cargo.toml Update: Add lancedb (embedded vector DB), arrow-array, candle-core, candle-nn, and tokenizers.

Database Initialization: Create src-tauri/src/db/vector.rs. Initialize a LanceDB connection pool pointing to .golutra/vectors inside the active workspace.

Schema Definition: Define an Arrow Schema for the SwarmMemory table: chunk_id (String), file_path (String), text (String), vector (FixedSizeList<Float32, 384>).

Embeddings Engine: Create src-tauri/src/llm/embeddings.rs. Initialize the sentence-transformers/all-MiniLM-L6-v2 model using Candle on application startup.

Vector Generation: Implement generate_embedding(text: &str) -> Vec<f32> in embeddings.rs that tokenizes the string and passes it through the Candle ONNX model.

Chunking Utility: Create src-tauri/src/llm/memory_indexer.rs. Implement a recursive text splitter that chunks global_architecture_ledger.md and source code into 512-token segments with 50-token overlaps.

Orchestrator Hook: In loop_runner.rs, add a MemoryIngestion phase immediately after the QA station finishes its chunk.

Background Indexing: Spawn a tokio::task that passes the newly written files and ledger updates to memory_indexer.rs, generates embeddings, and INSERTs them into LanceDB.

Tauri IPC (Query): Expose #[tauri::command] pub async fn query_swarm_memory(workspace_id: String, query: String, limit: u32) -> Result<Vec<String>, String>.

Search Execution: Inside query_swarm_memory, embed the query string, then execute a vector similarity search (lancedb::query().nearest_to(vector).limit(limit)).

Planner RAG Integration: In src-tauri/src/orchestrator/nodes.rs, modify run_planner(). Before hitting the OpenRouter API, call query_swarm_memory using the sprint_desc as the query.

Prompt Injection: Inject the retrieved LanceDB text chunks into the Planner's system_prompt wrapped inside <Historical_Context> XML tags.

Commander RAG Integration: Allow the Commander node to search past routing JSONs in LanceDB to maintain consistency in how files are mapped to wizard_clusters.

Frontend Store: Create src/stores/useMemoryStore.ts to manage retrievedContexts and vectorDbStats.

UI Component: Build src/features/swarm/MemoryViewer.tsx, a collapsible side-panel that displays the historical chunks the Planner currently recalled.

Telemetry Event: Emit a memory_retrieved Tauri event containing the text and cosine-similarity score of the pulled vectors.

Visual Feedback: In SwarmVisualizer.tsx, render a glowing "Synapse" animation connecting the MemoryViewer to the Planner node during RAG retrieval.

Tauri IPC (Re-index): Expose #[tauri::command] pub async fn force_reindex_workspace(workspace_id: String).

Tauri IPC (Clear): Expose #[tauri::command] pub async fn clear_vector_memory(workspace_id: String) to drop the LanceDB table.

Chat UI Integration: Update execute_agent_chat in main.py (or Rust equivalent) so if a user asks "How did we implement auth?", it triggers a RAG search before replying.

Garbage Collection: Implement a Rust cron-like task in vector.rs that deletes embeddings linked to files that have been removed from the file system.

LRU Cache: Implement an LRU cache in embeddings.rs to avoid re-embedding identical text chunks (like the unaltered top half of the ledger).

Cost Ignorance: Update cost tracking telemetry to register 0 API cost for local candle-core embedding generations.

Settings UI: Add a "Vector Memory" tab in Settings.tsx to view DB size, manually trigger re-indexes, or wipe memory.

Command Registration: Register all memory IPC commands in the tauri::generate_handler! macro.

Epic 4: Visual State Time-Travel (Git-Backed Redb) (Steps 176-200)
Objective: Build an instant undo/rollback slider leveraging high-performance text diffing in Redb.

Cargo.toml Update: Add the similar crate for Myers diffing algorithms, and zstd for extreme compression of diff payloads.

Redb Schema Extension: In src-tauri/src/db/store.rs, define FILE_SNAPSHOTS: TableDefinition<(u64, &str), &[u8]> mapping (Timestamp, Filepath) -> Zstd Compressed Diff.

Timeline Schema: Define SNAPSHOT_TIMELINE: TableDefinition<u64, &str> mapping Timestamp -> Commit Message (e.g., "Sprint 2: Backend Auth").

History Module: Create src-tauri/src/workers/history.rs to encapsulate diff generation and patching logic.

Pre-Write Hook: In src-tauri/src/workers/conflict.rs, intercept attempt_lock. Before a worker writes, read the current file state from disk.

Diff Generation: In history.rs, use similar::TextDiff to compute a unified diff between the existing disk content and the incoming LLM code.

Storage Execution: Compress the diff via zstd, serialize it, and insert it into FILE_SNAPSHOTS keyed by the active Swarm timestamp.

Checkpointing: In loop_runner.rs, generate a master run_timestamp at the start of each SprintChunk. Insert a summary into SNAPSHOT_TIMELINE.

Tauri IPC (Timeline): Expose #[tauri::command] pub async fn get_snapshot_timeline(workspace_id: String) -> Result<Vec<CommitNode>, String>.

Tauri IPC (Revert): Expose #[tauri::command] pub async fn revert_to_snapshot(workspace_id: String, timestamp: u64) -> Result<(), String>.

Revert Logic: Inside revert_to_snapshot, query FILE_SNAPSHOTS for all diffs between "now" and the target_timestamp. Apply reverse-patches to the physical files via std::fs::write.

FS Watcher Suspension: Temporarily disable the notify file watcher during a revert to prevent a flood of file_changed events crashing the UI.

Frontend Store: Create src/stores/useTimeTravelStore.ts in Zustand to manage timeline: CommitNode[] and currentScrubIndex: number.

UI Component: Build src/features/swarm/ScrubSlider.tsx featuring a horizontal <input type="range"> mapped from 0 to timeline.length - 1.

Scrubbing Logic: Add a debounced onChangeEnd handler to the slider that invokes Bridge.revertToSnapshot(timeline[index].timestamp).

Monaco Editor Sync: When a revert completes, emit a workspace_files_restored Tauri event. The frontend must catch this, invalidate fileContentsCache, and force Monaco to reload the active file.

Diff Preview IPC: Create #[tauri::command] pub async fn preview_snapshot_diff(timestamp: u64) to fetch the exact code changes without physically altering the file system.

Visual Diff UI: Build src/features/workspace/DiffModal.tsx utilizing @monaco-editor/react's <DiffEditor /> to show the user what will change before they drop the slider.

Orchestrator Safety: Lock the ScrubSlider (disabled={true}) whenever useSwarmStore.active_state is not Idle, Complete, or Failed.

Panic Button: Add an "Undo Swarm Run" button to SwarmDashboard.tsx that instantly fetches the timestamp from the start of the current run and executes revert_to_snapshot.

Task Abortion: Update loop_runner.rs so that if a revert is triggered while the Swarm is running, all Tokio JoinHandles are immediately abort()ed.

Ledger Rollback: Ensure global_architecture_ledger.md is strictly included in the snapshot diffs, preventing the AI from retaining memories of the reverted future.

Redb Garbage Collection: Implement a startup task in store.rs that prunes snapshots older than 14 days to prevent .redb file bloat.

Error Boundary: Wrap the revert_to_snapshot patching logic in a transaction. If a patch fails to apply cleanly, abort the file write and emit an error Toast to the UI.

Command Registration: Register get_snapshot_timeline, revert_to_snapshot, and preview_snapshot_diff in main.rs to finalize the Phase 11-20 Architectural Blueprint.