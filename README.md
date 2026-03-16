# REM

REM is a next-generation modal text editor engineered to bridge the gap between the approachability of mainstream IDEs and the raw efficiency of terminal-based modal editors.

## Table of Contents
1. [The Vision](#the-vision)
2. [Architectural Foundations](#architectural-foundations)
3. [Development Roadmap](#development-roadmap)
4. [License](#license)

## The Vision
Modern software development forces a compromise:
1. Accept the high latency and massive memory footprint of Electron-based editors (e.g., VSCode) to gain an intuitive user interface and a rich plugin ecosystem.
2. Invest months memorizing complex keybindings to harness the speed, efficiency, and low latency of modal editors (e.g., Neovim, Helix) running in the terminal.

REM eliminates this compromise. It is designed from the ground up to offer a progressive learning curve for modal editing, wrapped in a high-performance, strictly sandboxed, and highly concurrent architecture. 

## Architectural Foundations
REM is entirely written in Rust to guarantee memory safety without a garbage collector, ensuring sub-millisecond latency and predictable performance. 

The architecture strictly follows a Client-Server paradigm:
* **Headless Core:** The core engine manages text manipulation, file I/O, and asynchronous tasks in a detached environment.
* **RPC Protocol:** The frontend and the core communicate strictly via message-passing (RPC), ensuring the UI thread is never blocked by heavy computations.
* **Agnostic Views:** REM treats both the Terminal User Interface (TUI) and the GPU-accelerated Graphical User Interface (GUI) as first-class citizens. 
* **Piece Table Engine:** The text buffer is backed by a highly optimized Piece Table. This append-only data structure guarantees instant loading of gigabyte-sized files, zero-allocation edits, and an inherently infinite undo/redo tree.

## Development Roadmap

### Phase 1: Core Engine & Data Structures
- [x] Design the agnostic Client-Server architecture (Core vs. View).
- [x] Define the JSON-RPC message protocol for cross-thread communication.
- [x] Implement the Piece Table data structure for `O(1)` file loading and zero-allocation edits.
- [ ] Implement an infinite Undo/Redo tree leveraging Piece Table immutability.
- [ ] Add robust UTF-8 and multi-byte character support.

### Phase 2: Terminal User Interface (TUI)
- [x] Setup the raw-mode terminal interface using `crossterm` and `ratatui`.
- [x] Establish asynchronous message-passing channels between TUI and Core.
- [ ] Implement 2D coordinate mapping (Row/Column) for accurate cursor rendering.
- [ ] Build a robust viewport scrolling system.

### Phase 3: The Modal State Machine
- [ ] Implement the core Finite State Machine (Normal, Insert, Visual, Command modes).
- [ ] Parse basic motions (`h`, `j`, `k`, `l`, `w`, `b`, `e`) and operators (`d`, `c`, `y`).
- [ ] Develop the "Ghost Motions" UX: real-time visual hints mapping standard IDE inputs to modal commands.

### Phase 4: Graphical User Interface (GUI)
- [ ] Scaffold a GPU-accelerated frontend (e.g., using `wgpu` or `egui`).
- [ ] Implement a custom font shaper for code ligatures and pixel-perfect text rendering.
- [ ] Ensure parity between TUI and GUI RPC implementations.

### Phase 5: Asynchronous Intelligence
- [ ] Build an Actor-based background daemon for Language Server Protocol (LSP) integration.
- [ ] Implement 100% Lazy-Loading for LSPs (activating strictly on-demand).
- [ ] Integrate an embedded database (SQLite) for aggressive syntax and AST caching.

### Phase 6: Sandboxed Plugin Ecosystem
- [ ] Embed a WebAssembly runtime (e.g., Wasmtime) within the Core.
- [ ] Design a strict, manifest-based permission system for file system and network access.
- [ ] Expose the REM API bindings to Wasm plugins.

### Phase 7: Native Collaboration (Live Share)
- [ ] Translate Piece Table mutations into Conflict-free Replicated Data Types (CRDTs).
- [ ] Implement a Peer-to-Peer network layer for zero-latency state synchronization.
- [ ] Render remote multi-cursors seamlessly within the View.

## License
REM is distributed under the GNU General Public License v3.0 (GPLv3).
