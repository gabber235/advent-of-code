import { lines } from "../../utils";

export function part1(input: string): number | string {
  const grid = lines(input);
  const height = grid.length;
  const width = grid[0].length;

  let startCol = 0;
  for (let col = 0; col < width; col++) {
    if (grid[0][col] === "S") {
      startCol = col;
      break;
    }
  }

  let beams = new Set<number>([startCol]);
  let splitCount = 0;

  for (let row = 1; row < height && beams.size > 0; row++) {
    const newBeams = new Set<number>();

    for (const col of beams) {
      if (col < 0 || col >= width) continue;

      const cell = grid[row][col];
      if (cell === "^") {
        splitCount++;
        if (col - 1 >= 0) newBeams.add(col - 1);
        if (col + 1 < width) newBeams.add(col + 1);
      } else {
        newBeams.add(col);
      }
    }

    beams = newBeams;
  }

  return splitCount;
}

export function part2(input: string): number | string {
  const grid = lines(input);
  const height = grid.length;
  const width = grid[0].length;

  let startCol = 0;
  for (let col = 0; col < width; col++) {
    if (grid[0][col] === "S") {
      startCol = col;
      break;
    }
  }

  let timelines = new Map<number, number>([[startCol, 1]]);

  for (let row = 1; row < height && timelines.size > 0; row++) {
    const newTimelines = new Map<number, number>();

    for (const [col, count] of timelines) {
      if (col < 0 || col >= width) continue;

      const cell = grid[row][col];
      if (cell === "^") {
        if (col - 1 >= 0) {
          newTimelines.set(col - 1, (newTimelines.get(col - 1) ?? 0) + count);
        }
        if (col + 1 < width) {
          newTimelines.set(col + 1, (newTimelines.get(col + 1) ?? 0) + count);
        }
      } else {
        newTimelines.set(col, (newTimelines.get(col) ?? 0) + count);
      }
    }

    timelines = newTimelines;
  }

  let total = 0;
  for (const count of timelines.values()) {
    total += count;
  }
  return total;
}
