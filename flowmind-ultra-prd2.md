# 🚀 ARCHITECTURAL BLUEPRINT: FLOWMIND ULTRA (PHASE 11-20)

**Target Audience:** Autonomous AI Coding Agent (DeepThink / Claude / Devin / Cursor)  
**System Context:** Tauri v2 (Rust) + React 19 (Zustand/Vite) + Redb + Tokio + Portable-PTY.  
**Mandate:** Execute the following 100 strict architectural steps (101-200) to implement the four advanced Epics. Do not write the code yet; use this as the structural scaffolding, specify the exact crates, detail the Tauri IPC bridges, and define the React/Zustand state topologies.

---

### Epic 1: The Self-Healing Compiler Loop (QA 2.0) (Steps 101-125)
*Objective: Upgrade the QA station from a passive markdown reviewer to an active, headless LSP/Compiler loop that automatically rewrites failing code.*

101. **Cargo.toml Update:** Add the `regex` crate for high-performance `stderr` parsing and `tokio-util` for asynchronous PTY stream handling.
102. **Rust Module Creation:** Create `src-tauri/src/workers/compiler.rs` to manage headless compiler sessions, entirely isolated from the user-facing `TerminalManager`.
103. **Headless PTY Wrapper:** In `compiler.rs`, implement a `CompilerSession` struct using `portable-pty`. Configure it to run without PTY echo, strictly piping `stderr` and `stdout` into a Tokio `BytesMut` buffer.
104. **Compiler Detection:** Implement `detect_compiler_env()` in `compiler.rs`. Scan the `workspace_dir` for `Cargo.toml` (`cargo check`), `tsconfig.json` (`tsc --noEmit`), or `requirements.txt` (`flake8`) to dynamically resolve the build command.
105. **Execution Routine:** Create `run_compiler_check()` that spawns the detected compiler, awaits the exit code, and returns a `Result<(), String>` containing the raw `stderr` on failure.
106. **Error Parsing:** Implement a `CompilerDiagnostics` struct and use `regex` to parse the `stderr` buffer, extracting exact `file_path`, `line_number`, and `error_message`.
107. **State Machine Update:** In `src-tauri/src/orchestrator/state.rs`, update the `SwarmState` enum to include `SelfHealing { attempt: u8, max_attempts: u8 }`.
108. **Loop Integration:** In `loop_runner.rs`, intercept the transition after the `Executor` phase. Await `run_compiler_check()`. If exit code is `0`, proceed to QA/Ledger. If `>0`, transition state to `SelfHealing`.
109. **Sniper Prompt Assembly:** Create `src-tauri/src/llm/sniper.rs`. Construct a strict prompt containing: 1) The failing file's current AST/code, 2) The parsed compiler error, 3) Instructions to output ONLY valid JSON containing the exact code replacement.
110. **LLM Invocation:** Call `LlmClient::complete()` using the `executorSpecialist` model profile to generate the patch.
111. **Patch Sanitization:** Pass the Sniper's output through `sanitize_json()` in `src-tauri/src/llm/sanitizer.rs` to strip markdown wrappers.
112. **Lock Bypass:** In `src-tauri/src/workers/conflict.rs`, implement a `force_acquire_lock()` method allowing the Sniper agent to bypass the standard queue and lock the broken file synchronously.
113. **File Overwrite:** Apply the JSON patch to the local file system via `std::fs::write` and immediately release the lock.
114. **Recursive Healing:** Recursively loop `run_compiler_check()`. Cap the recursion at `max_attempts` (default 3) to prevent infinite API spend.
115. **Fallback Escalation:** If `max_attempts` is breached, emit a `station_update` with status `Failed`, halting the Swarm and prompting human intervention.
116. **Tauri IPC:** Expose `#[tauri::command] pub async fn manual_compiler_override(workspace_id: String)` in `orchestrator/commands.rs` to let the user unblock the Swarm.
117. **Frontend State:** In `src/stores/useSwarmStore.ts`, update `NodeStatus` to accept `'Healing'` and `'AwaitingHumanFix'`.
118. **UI Animation:** In `src/features/swarm/NodeIndicator.tsx`, implement a CSS keyframe animation (pulsing amber with a wrench icon) for the `Healing` status.
119. **Live Telemetry:** Emit a Tauri event `compiler_diagnostics_stream` containing the raw `stderr` buffer.
120. **Terminal UI:** In `src/features/terminal/TerminalPanel.tsx`, add a hidden "Compiler Output" tab that tails the `compiler_diagnostics_stream` event when active.
121. **Chat UI Notification:** In `src/features/chat/ChatGraph.tsx`, inject a `ChatBubble` stating: `"🚨 Compiler Error Detected in {file}. Sniper Agent deployed (Attempt {n}/3)..."`
122. **Success Broadcast:** On successful compilation, emit a UI event: `"✅ Compilation successful. Codebase stabilized."`
123. **Ledger Annotation:** In `src-tauri/src/llm/ledger.rs`, append the resolved error and patch summary to `global_architecture_ledger.md` so the Planner avoids repeating the syntax error in future chunks.
124. **Settings Toggle:** Add a toggle in `src/features/Settings.tsx` to enable/disable the "Self-Healing Compiler Loop".
125. **Command Registration:** Register `manual_compiler_override` in the `tauri::generate_handler!` macro in `main.rs`.

---

### Epic 2: Acoustic Command Interface (Vibecoding by Voice) (Steps 126-150)
*Objective: Integrate offline, native voice-to-text directly into the Rust backend to bypass keyboard bottlenecks.*

126. **Cargo.toml Update:** Add `cpal` (low-latency audio I/O), `hound` (WAV encoding), `rubato` (audio resampling), and `whisper-rs` (Whisper C++ bindings).
127. **OS Permissions:** Update `tauri.conf.json` to include macOS `NSMicrophoneUsageDescription` and Windows microphone manifests.
128. **Audio Module:** Create a new directory `src-tauri/src/audio/` containing `mod.rs`, `capture.rs`, and `transcribe.rs`.
129. **Model Initialization:** In `main.rs`, write a startup hook that checks `app_cache_dir()` for `ggml-base.en.bin`. If missing, asynchronously download it via `reqwest`.
130. **Context Setup:** In `transcribe.rs`, initialize `WhisperContext` and `WhisperState` into a globally managed `Arc<Mutex<>>` on boot.
131. **Audio Capture:** In `capture.rs`, use `cpal` to build `start_audio_stream()`, targeting the default input device at 1 channel (mono), `f32` format.
132. **Ring Buffer:** Pipe the `cpal` stream into a thread-safe `crossbeam-channel` or `std::sync::mpsc` buffer.
133. **Resampling:** Implement a Tokio background task that consumes the buffer, uses `rubato` to resample the audio to exactly 16kHz, and temporarily encodes it to WAV via `hound`.
134. **Voice Activity Detection (VAD):** Implement a simple RMS (Root Mean Square) amplitude calculator in the stream. If RMS drops below a threshold for 1.5s, trigger auto-stop.
135. **Tauri IPC (Start):** Expose `#[tauri::command] pub async fn start_voice_dictation() -> Result<(), String>`.
136. **Tauri IPC (Stop):** Expose `#[tauri::command] pub async fn stop_and_transcribe_audio() -> Result<String, String>`.
137. **Inference Execution:** In `stop_and_transcribe_audio`, pass the 16kHz PCM data to `WhisperState::full()`, extract the tokens, and concatenate the transcript.
138. **Volume Telemetry:** Emit a Tauri event `dictation_volume_level` at 30fps containing the `f32` RMS value to drive frontend UI equalizers.
139. **Frontend Store:** Create `src/stores/useVoiceStore.ts` with state variables: `isRecording`, `isTranscribing`, `transcript`, and `volumeLevel`.
140. **UI Component:** Create `src/features/chat/VoiceInput.tsx` featuring a microphone button (using `lucide-react`).
141. **Push-to-Talk Logic:** Bind `onMouseDown` to `start_voice_dictation` and `onMouseUp` to `stop_and_transcribe_audio`.
142. **Waveform Animation:** In `VoiceInput.tsx`, map the `dictation_volume_level` event to CSS `transform: scale()` on a radial gradient behind the mic icon.
143. **Origin Node Binding:** Automatically append the resolved transcript string from Rust to the `prompt` state variable in `SwarmDashboard.tsx`.
144. **Hardware Acceleration:** Ensure `whisper-rs` is built with `--features coreml` on macOS and `--features cuda` on Windows in `Cargo.toml`.
145. **IPC (Cancel):** Expose `#[tauri::command] pub async fn cancel_voice_dictation()` to flush the buffer and abort inference.
146. **Grammar Correction Pipeline:** Create an optional sub-routine in `transcribe.rs` that pipes the raw Whisper output through a fast, local LLM prompt: "Format this raw dictation into a clean developer prompt."
147. **Settings UI:** Add a "Voice Model" selector in `Settings.tsx` to let users choose between `tiny.en`, `base.en`, or `small.en` models.
148. **Auto-Deploy Toggle:** Add a setting "Auto-Deploy Swarm on Voice Stop" to immediately trigger the orchestrator once transcription completes.
149. **Resource Cleanup:** Implement an `impl Drop` or hook into `WindowEvent::Destroyed` to ensure the `cpal` stream releases the OS microphone lock on app exit.
150. **Command Registration:** Register `start_voice_dictation`, `stop_and_transcribe_audio`, and `cancel_voice_dictation` in `main.rs`.

---

### Epic 3: Vectorized Swarm Memory (Long-Term Context) (Steps 151-175)
*Objective: Eliminate Swarm amnesia by persisting architectural decisions in a local vector database for semantic retrieval (RAG).*

151. **Cargo.toml Update:** Add `lancedb` (embedded vector DB), `arrow-array`, `candle-core`, `candle-nn`, and `tokenizers`.
152. **Database Initialization:** Create `src-tauri/src/db/vector.rs`. Initialize a LanceDB connection pool pointing to `.golutra/vectors` inside the active workspace.
153. **Schema Definition:** Define an Arrow Schema for the `SwarmMemory` table: `chunk_id` (String), `file_path` (String), `text` (String), `vector` (FixedSizeList<Float32, 384>).
154. **Embeddings Engine:** Create `src-tauri/src/llm/embeddings.rs`. Initialize the `sentence-transformers/all-MiniLM-L6-v2` model using Candle on application startup.
155. **Vector Generation:** Implement `generate_embedding(text: &str) -> Vec<f32>` in `embeddings.rs` that tokenizes the string and passes it through the Candle ONNX model.
156. **Chunking Utility:** Create `src-tauri/src/llm/memory_indexer.rs`. Implement a recursive text splitter that chunks `global_architecture_ledger.md` and source code into 512-token segments with 50-token overlaps.
157. **Orchestrator Hook:** In `loop_runner.rs`, add a `MemoryIngestion` phase immediately after the `QA` station finishes its chunk.
158. **Background Indexing:** Spawn a `tokio::task` that passes the newly written files and ledger updates to `memory_indexer.rs`, generates embeddings, and `INSERT`s them into LanceDB.
159. **Tauri IPC (Query):** Expose `#[tauri::command] pub async fn query_swarm_memory(workspace_id: String, query: String, limit: u32) -> Result<Vec<String>, String>`.
160. **Search Execution:** Inside `query_swarm_memory`, embed the `query` string, then execute a vector similarity search (`lancedb::query().nearest_to(vector).limit(limit)`).
161. **Planner RAG Integration:** In `src-tauri/src/orchestrator/nodes.rs`, modify `run_planner()`. Before hitting the OpenRouter API, call `query_swarm_memory` using the `sprint_desc` as the query.
162. **Prompt Injection:** Inject the retrieved LanceDB text chunks into the Planner's `system_prompt` wrapped inside `<Historical_Context>` XML tags.
163. **Commander RAG Integration:** Allow the Commander node to search past routing JSONs in LanceDB to maintain consistency in how files are mapped to `wizard_clusters`.
164. **Frontend Store:** Create `src/stores/useMemoryStore.ts` to manage `retrievedContexts` and `vectorDbStats`.
165. **UI Component:** Build `src/features/swarm/MemoryViewer.tsx`, a collapsible side-panel that displays the historical chunks the Planner currently recalled.
166. **Telemetry Event:** Emit a `memory_retrieved` Tauri event containing the text and cosine-similarity score of the pulled vectors.
167. **Visual Feedback:** In `SwarmVisualizer.tsx`, render a glowing "Synapse" animation connecting the `MemoryViewer` to the `Planner` node during RAG retrieval.
168. **Tauri IPC (Re-index):** Expose `#[tauri::command] pub async fn force_reindex_workspace(workspace_id: String)`.
169. **Tauri IPC (Clear):** Expose `#[tauri::command] pub async fn clear_vector_memory(workspace_id: String)` to drop the LanceDB table.
170. **Chat UI Integration:** Update `execute_agent_chat` in `main.py` (or Rust equivalent) so if a user asks "How did we implement auth?", it triggers a RAG search before replying.
171. **Garbage Collection:** Implement a Rust cron-like task in `vector.rs` that deletes embeddings linked to files that have been removed from the file system.
172. **LRU Cache:** Implement an LRU cache in `embeddings.rs` to avoid re-embedding identical text chunks (like the unaltered top half of the ledger).
173. **Cost Ignorance:** Update cost tracking telemetry to register `0` API cost for local `candle-core` embedding generations.
174. **Settings UI:** Add a "Vector Memory" tab in `Settings.tsx` to view DB size, manually trigger re-indexes, or wipe memory.
175. **Command Registration:** Register all memory IPC commands in the `tauri::generate_handler!` macro.

---

### Epic 4: Visual State Time-Travel (Git-Backed Redb) (Steps 176-200)
*Objective: Build an instant undo/rollback slider leveraging high-performance text diffing in Redb.*

176. **Cargo.toml Update:** Add the `similar` crate for Myers diffing algorithms, and `zstd` for extreme compression of diff payloads.
177. **Redb Schema Extension:** In `src-tauri/src/db/store.rs`, define `FILE_SNAPSHOTS: TableDefinition<(u64, &str), &[u8]>` mapping `(Timestamp, Filepath)` -> `Zstd Compressed Diff`.
178. **Timeline Schema:** Define `SNAPSHOT_TIMELINE: TableDefinition<u64, &str>` mapping `Timestamp` -> `Commit Message` (e.g., "Sprint 2: Backend Auth").
179. **History Module:** Create `src-tauri/src/workers/history.rs` to encapsulate diff generation and patching logic.
180. **Pre-Write Hook:** In `src-tauri/src/workers/conflict.rs`, intercept `attempt_lock`. Before a worker writes, read the current file state from disk.
181. **Diff Generation:** In `history.rs`, use `similar::TextDiff` to compute a unified diff between the existing disk content and the incoming LLM code.
182. **Storage Execution:** Compress the diff via `zstd`, serialize it, and insert it into `FILE_SNAPSHOTS` keyed by the active `Swarm` timestamp.
183. **Checkpointing:** In `loop_runner.rs`, generate a master `run_timestamp` at the start of each `SprintChunk`. Insert a summary into `SNAPSHOT_TIMELINE`.
184. **Tauri IPC (Timeline):** Expose `#[tauri::command] pub async fn get_snapshot_timeline(workspace_id: String) -> Result<Vec<CommitNode>, String>`.
185. **Tauri IPC (Revert):** Expose `#[tauri::command] pub async fn revert_to_snapshot(workspace_id: String, timestamp: u64) -> Result<(), String>`.
186. **Revert Logic:** Inside `revert_to_snapshot`, query `FILE_SNAPSHOTS` for all diffs between "now" and the `target_timestamp`. Apply reverse-patches to the physical files via `std::fs::write`.
187. **FS Watcher Suspension:** Temporarily disable the `notify` file watcher during a revert to prevent a flood of `file_changed` events crashing the UI.
188. **Frontend Store:** Create `src/stores/useTimeTravelStore.ts` in Zustand to manage `timeline: CommitNode[]` and `currentScrubIndex: number`.
189. **UI Component:** Build `src/features/swarm/ScrubSlider.tsx` featuring a horizontal `<input type="range">` mapped from `0` to `timeline.length - 1`.
190. **Scrubbing Logic:** Add a debounced `onChangeEnd` handler to the slider that invokes `Bridge.revertToSnapshot(timeline[index].timestamp)`.
191. **Monaco Editor Sync:** When a revert completes, emit a `workspace_files_restored` Tauri event. The frontend must catch this, invalidate `fileContentsCache`, and force Monaco to reload the active file.
192. **Diff Preview IPC:** Create `#[tauri::command] pub async fn preview_snapshot_diff(timestamp: u64)` to fetch the exact code changes without physically altering the file system.
193. **Visual Diff UI:** Build `src/features/workspace/DiffModal.tsx` utilizing `@monaco-editor/react`'s `<DiffEditor />` to show the user what will change before they drop the slider.
194. **Orchestrator Safety:** Lock the `ScrubSlider` (`disabled={true}`) whenever `useSwarmStore.active_state` is not `Idle`, `Complete`, or `Failed`.
195. **Panic Button:** Add an "Undo Swarm Run" button to `SwarmDashboard.tsx` that instantly fetches the timestamp from the start of the current run and executes `revert_to_snapshot`.
196. **Task Abortion:** Update `loop_runner.rs` so that if a revert is triggered while the Swarm is running, all Tokio `JoinHandle`s are immediately `abort()`ed.
197. **Ledger Rollback:** Ensure `global_architecture_ledger.md` is strictly included in the snapshot diffs, preventing the AI from retaining memories of the reverted future.
198. **Redb Garbage Collection:** Implement a startup task in `store.rs` that prunes snapshots older than 14 days to prevent `.redb` file bloat.
199. **Error Boundary:** Wrap the `revert_to_snapshot` patching logic in a transaction. If a patch fails to apply cleanly, abort the file write and emit an error Toast to the UI.
200. **Command Registration:** Register `get_snapshot_timeline`, `revert_to_snapshot`, and `preview_snapshot_diff` in `main.rs` to finalize the Phase 11-20 Architectural Blueprint.