# Advent of Code Solutions

A Rust implementation of solutions for [Advent of Code](https://adventofcode.com) programming challenges.

## Overview

This project contains solutions to Advent of Code problems, organized by year and day. Each solution is implemented in
Rust with a clean, modular structure that separates part 1 and part 2 solutions.

## Project Structure

```
src/
├── main.rs                 # CLI entry point
├── lib.rs                  # Library interface
└── problems/
    └── year_2015/          # Solutions for 2015
        ├── day_1/          # Day 1 solutions
        │   ├── mod.rs      # Day module with solve() function
        │   ├── part1.rs    # Part 1 solution
        │   ├── part2.rs    # Part 2 solution
        │   └── input.txt   # Problem input (gitignored)
        ├── day_2/
        │   ├── mod.rs
        │   ├── part1.rs
        │   ├── part2.rs
        │   └── input.txt
        └── ...             # Additional days
```

## Features

-   **Modular Design**: Each day's solution is self-contained with separate modules for part 1 and part 2
-   **CLI Interface**: Run solutions from the command line with year and day parameters
-   **Input Management**: Each day has its own input file for easy testing
-   **Clean Output**: Solutions print results for both parts in a readable format

## Usage

### Running Solutions

Run a specific day's solution:

```bash
# Run day 1 of 2015 (default)
cargo run

# Run specific day
cargo run 3

# Run specific day and year
cargo run 5 2015
```

### Adding New Solutions

1. Create a new day directory under the appropriate year:

    ```bash
    mkdir -p src/problems/year_2015/day_6
    ```

2. Create the module files:

    - `mod.rs`: Main solve function that calls part1 and part2
    - `part1.rs`: Part 1 solution
    - `part2.rs`: Part 2 solution
    - `input.txt`: Problem input data

3. Add the day module to the year's `mod.rs` file

4. Update the match statement in `src/lib.rs` to include the new day

### Example Solution Structure

```rust
// mod.rs
mod part1;
mod part2;

pub fn solve(input: &str) {
    let part1_result = part1::solve(input);
    let part2_result = part2::solve(input);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

// part1.rs
pub fn solve(input: &str) -> i32 {
    // Part 1 solution logic
}

// part2.rs
pub fn solve(input: &str) -> i32 {
    // Part 2 solution logic
}
```
