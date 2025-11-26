# Advent of Code 2025

Every year I do something fun for advent of code, sometimes learning a new language, sometimes trying to be fast or make fast solutions. Since this year I don't have a lot of time, I want to do something else fun. As AI agents have become much better, this year I want to vibecode 100% of advent of code, to see how good AI's have gotten, and how much help they still need.

A feature-rich TypeScript + Bun setup for solving Advent of Code puzzles efficiently.

## Quick Start

### Setup

```bash
bun install
```

### Create a New Day

```bash
bun new <day>
```

This creates:
- `src/days/XX/index.ts` - Solution template
- `src/days/XX/input.txt` - Your puzzle input (paste it here)
- `src/days/XX/example.txt` - Example input from the problem
- `src/days/XX/expected.json` - Expected outputs for testing

### Run a Solution

```bash
bun day <day>
```

Run with example input:
```bash
bun day <day> --example
```

### Test Solutions

Test all days:
```bash
bun test
```

Test a specific day:
```bash
bun test <day>
```

### Linting

Check for linting issues:
```bash
bun lint
```

Auto-fix linting issues:
```bash
bun lint:fix
```

## Project Structure

```
src/
├── days/
│   └── 01/              # Each day gets its own folder
│       ├── index.ts     # Your solution
│       ├── input.txt    # Puzzle input
│       ├── example.txt  # Example input
│       └── expected.json # Expected outputs
├── utils/               # Utility functions
│   ├── parsing.ts      # Input parsing helpers
│   ├── math.ts         # Math utilities
│   ├── grid.ts         # 2D grid operations
│   └── collections.ts  # Data structures
└── runner.ts           # Solution runner
```

## Utilities

### Parsing (`utils/parsing.ts`)

```typescript
import { lines, numbers, grid, blocks, words } from "../utils";

lines(input)      // Split into lines
numbers(input)    // Extract all numbers
grid(input)       // Parse into 2D char array
blocks(input)     // Split by double newlines
words(line)       // Split line into words
```

### Math (`utils/math.ts`)

```typescript
import { gcd, lcm, sum, product, range, mod, clamp } from "../utils";

gcd(48, 18)              // 6
lcm(4, 6)                // 12
sum([1, 2, 3])           // 6
product([2, 3, 4])       // 24
Array.from(range(0, 5))  // [0, 1, 2, 3, 4]
mod(-1, 5)               // 4 (proper modulo)
clamp(15, 0, 10)         // 10
```

### Grid (`utils/grid.ts`)

```typescript
import { Grid } from "../utils";

const grid = Grid.fromString(input);
grid.get(x, y)                    // Get value at position
grid.set(x, y, value)             // Set value
grid.neighbors4(x, y)             // Get 4-direction neighbors
grid.neighbors8(x, y)             // Get 8-direction neighbors
grid.find((val, x, y) => ...)     // Find first matching cell
grid.findAll('X')                 // Find all positions of 'X'
grid.forEach((val, x, y) => ...)  // Iterate all cells
grid.clone()                      // Deep copy
grid.bfs([0, 0], [10, 10])       // Breadth-first search
```

### Collections (`utils/collections.ts`)

```typescript
import { Counter, DefaultMap, PriorityQueue, memoize } from "../utils";

const counter = new Counter(['a', 'b', 'a']);
counter.add('a');                 // Increment count
counter.mostCommon(2)              // Get top 2

const map = new DefaultMap(() => []);
map.get('key').push(1);           // Auto-creates array if missing

const pq = new PriorityQueue((a, b) => a - b);
pq.push(5);
pq.pop();                         // Returns smallest

const memoized = memoize((n: number) => expensive(n));
```

## Solution Template

Each day's solution follows this structure:

```typescript
import { lines } from "../../utils";

export function part1(input: string): number | string {
  const data = lines(input);
  return 0;
}

export function part2(input: string): number | string {
  const data = lines(input);
  return 0;
}
```

## Tips

- Use `bun day <day> --example` to test with example input while developing
- Fill in `expected.json` with the example outputs to enable automated testing
- All utilities are available via `import { ... } from "../utils"`
- The runner shows execution time for each part
- Grid utilities are perfect for 2D puzzles (mazes, pathfinding, etc.)

Happy coding! 🎄

