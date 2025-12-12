# Advent of Code 2025

Learning Rust through Advent of Code 2025. Completed all 23 puzzles!

## AI Assistance

I used [Claude Code](https://github.com/anthropics/claude-code) to help solve 19 out of 23 puzzles. Of the 4 puzzles I completed without AI:

- 3 of them I could have finished with more time
- Day 10 Part 2 I had zero shot at solving on my own (linear algebra over integers with Gaussian elimination)

## Progress

| Day | Part 1 | Part 2 |
|-----|--------|--------|
| 1   | ✅     | ✅     |
| 2   | ✅     | ✅     |
| 3   | ✅     | ✅     |
| 4   | ✅     | ✅     |
| 5   | ✅     | ✅     |
| 6   | ✅     | ✅     |
| 7   | ✅     | ✅     |
| 8   | ✅     | ✅     |
| 9   | ✅     | ✅     |
| 10  | ✅     | ✅     |
| 11  | ✅     | ✅     |
| 12  | ✅     | -      |

## Structure

- `src/bin/` - Daily solutions (e.g., `day01-part1.rs`, `day12.rs`)
- `src/input/` - Puzzle inputs
- `learning/` - Rust learning exercises

## Running Solutions

```bash
cargo run --bin day01-part1
cargo run --bin day01-part2
cargo run --bin day12
# etc.
```

## Testing

```bash
cargo test
```
