# Advent of Code Harness

A Rust-based framework for efficiently solving Advent of Code puzzles with automatic discovery, testing, and benchmarking.

## Features

- **Automatic day/year discovery** via proc macros - no manual registration needed
- **Timing on every run** with colored output
- **Test framework** with expected answers validation
- **Multiple example inputs** support (automatic `example_2.txt` for part 2)
- **Benchmarking** with Criterion
- **Utility library** for common AoC patterns

## CLI Commands

### Run a solution

```bash
# Run both parts with real input
cargo run --release -- run 2023 -d 1

# Run with example input
cargo run --release -- run 2023 -d 1 --example

# Run specific part
cargo run --release -- run 2023 -d 1 -p 1
cargo run --release -- run 2023 -d 1 -p 2

# Run today's puzzle (during December)
cargo run --release -- run 2023
```

### Test solutions

```bash
# Test a specific day against expected.toml
cargo run --release -- test 2023 -d 1

# Test all days for a year
cargo run --release -- test 2023
```

### Benchmark

```bash
cargo run --release -- bench 2023 -d 1
```

### Create a new day

```bash
cargo run --release -- new -y 2023 -d 1
```

This creates:
- `advent-puzzles/src/2023/day1/mod.rs` - Solution template
- `advent-puzzles/src/2023/day1/input.txt` - For puzzle input
- `advent-puzzles/src/2023/day1/example.txt` - For example input
- `advent-puzzles/src/2023/day1/expected.toml` - For expected answers

## Project Structure

```
.
├── src/
│   └── main.rs              # CLI runner
├── advent-derive/           # Proc macros for auto-discovery
└── advent-puzzles/
    ├── src/
    │   ├── lib.rs
    │   ├── utils/           # Utility modules
    │   │   ├── counter.rs   # Frequency counting
    │   │   ├── direction.rs # Cardinal directions
    │   │   ├── map.rs       # 2D grid helpers
    │   │   ├── math.rs      # GCD, LCM, etc.
    │   │   ├── parsing.rs   # Input parsing helpers
    │   │   ├── point.rs     # 2D points
    │   │   └── point_3d.rs  # 3D points
    │   └── <year>/
    │       └── day<N>/
    │           ├── mod.rs
    │           ├── input.txt
    │           ├── example.txt
    │           ├── example_2.txt  # Optional: separate example for part 2
    │           └── expected.toml
    └── example_day/         # Template for new days
```

## Solution Interface

Each day exports two functions:

```rust
pub fn part1(input: String) -> String {
    todo!()
}

pub fn part2(input: String) -> String {
    todo!()
}
```

## Expected Answers Format

`expected.toml` supports both example and real input validation:

```toml
[example]
part1 = "142"
part2 = "281"

# Optional: regression testing for real input
[real]
part1 = "54632"
part2 = "54019"
```

## Multiple Example Inputs

Some days have different examples for part 1 and part 2. Place the part 2 example in `example_2.txt` and it will be automatically used when running part 2 with `--example`.

## Utility Library

### Parsing (`utils::parsing`)

```rust
use advent_puzzles::utils::parsing::*;

numbers("pos: 10, -20")       // vec![10, -20]
blocks("a\nb\n\nc\nd")        // vec!["a\nb", "c\nd"]
grid_chars("ab\ncd")          // vec![vec!['a','b'], vec!['c','d']]
grid_digits("12\n34")         // vec![vec![1,2], vec![3,4]]
words("hello world")          // vec!["hello", "world"]
lines_with_numbers(input)     // Vec<Vec<i64>> per line
```

### Math (`utils::math`)

```rust
use advent_puzzles::utils::math::*;

gcd(12, 8)                    // 4
lcm(4, 6)                     // 12
lcm_many([4, 6, 8])           // 24
gcd_many([12, 18, 24])        // 6
mod_positive(-1, 3)           // 2 (handles negatives correctly)
```

### Counter (`utils::counter`)

```rust
use advent_puzzles::utils::counter::Counter;

let counter: Counter<char> = "aabbbcccc".chars().collect();
counter.get(&'a')             // 2
counter.most_common(2)        // vec![('c', 4), ('b', 3)]
counter.total()               // 9
```

### Grid Utilities (`utils::map`, `utils::point`, `utils::direction`)

```rust
use advent_puzzles::utils::{point::Point, direction::Direction, map::*};
use array2d::Array2D;

let grid: Array2D<char> = Array2D::generate_map(input, |_pos, c| c)?;
let value = grid.get_point(&Point::new(1, 2));
let neighbors = point.neighbours_within_map(&grid);
let next = point.move_in_direction(Direction::North);
```

## Workflow

1. `cargo run -- new -y 2023 -d 5` - Create day scaffold
2. Paste puzzle input into `input.txt`
3. Paste example into `example.txt`
4. Add expected example answers to `expected.toml`
5. Implement `part1`
6. `cargo run --release -- run 2023 -d 5 --example` - Test with example
7. `cargo run --release -- test 2023 -d 5` - Verify against expected
8. `cargo run --release -- run 2023 -d 5` - Run with real input
9. Repeat for `part2`
