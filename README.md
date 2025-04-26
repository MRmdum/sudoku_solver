# Rust Sudoku Solver

A fast and intelligent Sudoku solver written in Rust, focused on solving puzzles **logically** — without relying on brute-force guessing or trial-and-error methods.

---

## Features

- **Pure Logical Solving**: Implements human-like solving techniques (e.g., naked singles, hidden singles, locked candidates...)
- **Sudoku Generator**: Generate Sudokus to test on
- **Detailed Progress Tracking**: Step-by-step solution path (coming soon)

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70+ recommended)

### Installation

Clone the repository:

```bash
git clone https://github.com/MRmdum/sudoku_solver
cd sudoku_solver
cargo build --release
cargo run --release
```