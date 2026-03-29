# FlowMind Ultra Architectural Guide

Welcome to the definitive architectural manifesto of **FlowMind Ultra**, a multi-node, parallel-processing LLM agent swarm enclosed entirely within a native Rust + React (Tauri) application structure.

## 1. System Topology
FlowMind Ultra is a robust, local-first application designed to build software autonomously using a specialized set of cognitive LLM Nodes, routed safely into terminal-based Execution Worker clusters.

- **Frontend**: React + TypeScript + Tailwind (via Vite).
- **Backend Core**: Rust (Tauri).
- **Embedded Database**: Redb (Native Rust KV Store).
- **Terminal Emulator Engine**: `portable-pty` & `tokio` (Rust proxy) bound to `xterm.js` via IPC streams.

## 2. Global State & The IPC Bridge
State replication between Rust and React is managed strictly via `Zustand` stores dynamically reacting to Tauri IPC emitted events.

- `useSwarmStore.ts`: Listens to `station_update` to animate the 7-Node Swarm orchestrator pipeline.
- `useWorkerStore.ts`: Listens to `workers_status` identifying active Task loads dynamically pushed down by asynchronous Parallel Thread Pools (`ClusterManager`).
- `FileTree.tsx`: Listens to global lock constraints to visualize File Collisions and active agent overrides.

## 3. The Cognitive Node Engine (Swarm AI Pipeline)
FlowMind Ultra moves context strictly through 7 isolated stations utilizing a rigid State Machine pipeline.

1. **Origin**: Handles the raw human prompt initialization.
2. **SpecFactory**: Generates the master `prd.md`.
3. **Overseer**: Dissects the PRD into bounded `SprintChunks` natively parsed as JSON arrays.
4. **Planner**: Transforms the Sprint description into a Topological Graph structure.
5. **Commander**: Evaluates the Topological Graph into strict Execution Routes (Wizard Clusters vs Specialist Pairs). It outputs the deployment configuration.
6. **Executor**: Translates Commander's routes into deployable CLI outbox tasks targeting the `WorkerTask` framework.
7. **QA**: Review validation checks and failure self-healing iteration loops.

## 4. Master Async Orchestrator (`loop_runner.rs`)
The sequence is driven asynchronously by a detached `tokio::spawn` loop bridging the `LlmClient`, Redb Ledgers, and `Terminal Session` queues.

### Checkpoints and Pauses
A strict `Commander Checkpoint` requires explicit authorization by you (the human). Once `AwaitingApproval` is emitted, the orchestration loop awaits `Notify::notified()` hooked to `approve_commander_plan` before letting agents touch ANY files.

## 5. Sub-Systems

### The Terminal Engine Matrix
Instead of basic internal string operations, Agents actively execute commands in a real shell initialized with `portable-pty`.
- The `TerminalManager` (Global Dashboard UI PTY).
- The `ExecutionWorker` pools (Up to 3 concurrent parallel backend PTY processes `W1`, `W2`, `W3`).

### Lock Handling & Conflict Managers
FlowMind strictly prohibits active collision. `ClusterManager` utilizes a global `ConflictManager` hashset. If Agent 1 is modifying `package.json`, and Agent 2 wants to edit `package.json`, Agent 2's task is pushed back onto the polling queue until Agent 1 releases the lock.

### Manual Override Checks
Activating "Manual Override" strictly pauses the dispatch queue lock in `ClusterManager`, allowing developers to directly interface with individual agent sub-shells if they get hung up on interactive prompts.

## 6. Cleanup Protocol
On application teardown (`WindowEvent::Destroyed`), FlowMind aggressively fires closure events cascading down from `TerminalManager::kill_all()` and `ClusterManager::cleanup()`. Each active child process receives an explicit `SIGKILL`, utterly eliminating the threat of zombie background AI systems persisting.

---
**Status:** All 10 Core Architectural Phases Implemented & Functional.
