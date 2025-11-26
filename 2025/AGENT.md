# Agent Instructions for Advent of Code 2025

This document provides context and guidelines for AI agents working on this codebase.

## Project Overview

This is an Advent of Code 2025 solution repository built with TypeScript and Bun. The project is designed for rapid puzzle solving with a comprehensive utility library and automated tooling.

## Project Structure

```
src/
├── days/              # Daily puzzle solutions
│   └── XX/           # Two-digit day number (01-25)
│       ├── index.ts  # Solution with part1() and part2() functions
│       ├── input.txt # Puzzle input
│       ├── example.txt # Example input from problem
│       └── expected.json # Expected outputs for testing
├── utils/            # Utility functions (parsing, math, grid, collections)
└── runner.ts         # Solution runner with timing

scripts/
├── new.ts            # Scaffold new day
└── test.ts           # Test solutions against examples
```

## Key Conventions

### Solution Functions

Each day's `index.ts` must export:
- `part1(input: string): number | string` - Solution for part 1
- `part2(input: string): number | string` - Solution for part 2

The input is the raw puzzle input as a string. Use utilities from `../utils` to parse it.

### Utility Usage

All utilities are available via:
```typescript
import { lines, numbers, Grid, Counter, gcd, lcm } from "../utils";
```

Common patterns:
- `lines(input)` - Split input into lines
- `numbers(input)` - Extract all numbers from input
- `Grid.fromString(input)` - Parse 2D grid
- `new Counter(array)` - Count occurrences
- `gcd(a, b)` / `lcm(a, b)` - Math operations

### Code Style

- **No comments describing what code does** - Code should be self-explanatory
- **Only TODOs and public documentation** - Comments allowed for TODOs and function/class docs
- **Guard clauses over nested ifs** - Prefer early returns
- **Avoid code duplication** - Reuse existing utilities and classes

### Testing Workflow

**CRITICAL: Solutions must work correctly. Follow this workflow:**

1. **Test with example first**: Always test your solution against `example.txt` with expected outputs in `expected.json`
   - Run `bun test <day>` to verify the example produces the expected results
   - If the test fails, fix your solution before proceeding
2. **Run with actual input**: Once the example test passes, run `bun day <day>` to get the final answer with the actual puzzle input

Expected format in `expected.json`: `{"part1": "expected1", "part2": "expected2"}`

## Common Patterns

### Parsing Input

```typescript
const data = lines(input);                    // Array of lines
const nums = numbers(input);                  // Array of numbers
const grid = Grid.fromString(input);          // 2D grid
const blocks = blocks(input);                 // Split by double newline
```

### Grid Operations

```typescript
const grid = Grid.fromString(input);
grid.get(x, y);                               // Get value
grid.neighbors4(x, y);                         // 4-direction neighbors
grid.bfs([0, 0], [10, 10]);                   // Pathfinding
```

### Counting

```typescript
const counter = new Counter(array);
counter.add(item);
counter.mostCommon(5);                        // Top 5 most common
```

## CLI Commands

- `bun new <day>` - Create new day structure
- `bun day <day>` - Run solution
- `bun day <day> --example` - Run with example input
- `bun test` - Test all days
- `bun test <day>` - Test specific day
- `bun lint` - Run linter
- `bun lint:fix` - Fix linting issues

## When Adding New Days

1. Use `bun new <day>` to scaffold
2. Paste puzzle input into `input.txt`
3. Paste example input into `example.txt`
4. Add expected outputs to `expected.json`
5. Implement `part1()` and `part2()` in `index.ts`
6. **Test with example**: Run `bun test <day>` to verify the solution works with the example input
7. **Run actual input**: Once tests pass, run `bun day <day>` to get the final answer

## Contributing to the Utility Library

When solving puzzles, if you create general-purpose utilities that aren't specific to a single challenge, add them to the appropriate file in `src/utils/`:

- **Parsing utilities** → `src/utils/parsing.ts`
- **Math utilities** → `src/utils/math.ts`
- **Grid utilities** → `src/utils/grid.ts`
- **Collection/data structure utilities** → `src/utils/collections.ts`

Guidelines:
- Keep utilities generic and reusable across different puzzles
- Add TypeScript types for all parameters and returns
- Follow existing code style and patterns
- Export new utilities from `src/utils/index.ts`
- Consider backward compatibility when modifying existing utilities

