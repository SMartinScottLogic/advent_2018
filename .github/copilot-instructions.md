# Copilot Instructions for `advent_2018`

## Project Overview
- This is a Rust monorepo for Advent of Code 2018 solutions, organized by day (`day1`, `day2`, ..., `day10`).
- Each day is a separate Rust crate with its own `Cargo.toml`, `src/lib.rs`, and `src/main.rs`.
- Shared utilities are in the `utils/` crate (e.g., `graph.rs`, `math.rs`, `region.rs`, `runner.rs`, `solution.rs`, `grid/`, `point/`).
- The `input/` directory contains input files for each day, with `.full` and `.sample` variants.
- The `xtask/` crate provides automation scripts (see below).

## Key Workflows
- **Setup a new day:**
  - Run `cargo xtask setup dayN` (e.g., `cargo xtask setup day2`) to scaffold a new day's crate. This does not perform git actions.
- **Build all days:**
  - Run `cargo build --workspace` to build all crates.
- **Test all days:**
  - Run `cargo test --workspace` to test all crates.
- **Run a specific day's solution:**
  - `cargo run -p dayN --release`
  - Uses `.sample` file for sample input, and `.full` file for full input.

## Project Conventions
- Each day's crate is independent, but may use the `utils` crate for shared logic.
- Input files are not embedded; read from `input/` at runtime.
- Prefer putting reusable logic in `utils/` rather than duplicating across days.
- Use idiomatic Rust patterns; no project-specific macros or build scripts beyond `xtask`.
- No nonstandard test frameworks; use Rust's built-in test harness.

## Integration Points
- No external services or APIs; all dependencies are Rust crates (see each `Cargo.toml`).
- `xtask` is used for project automation (see `xtask/src/main.rs`).

## Examples
- To solve day 3 with both sample and full input: `cargo run -p day3 --release`
- To add a new day: `cargo xtask setup day11`

## References
- See `README.md` for setup instructions.
- See `utils/` for shared code patterns.
- See `xtask/` for automation logic.

---
If you are unsure about a workflow or convention, check the `README.md`, `xtask/`, or existing day crates for examples.
