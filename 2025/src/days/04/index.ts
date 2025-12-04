import { lines } from "../../utils";

export function part1(input: string): number | string {
  const grid = lines(input).map((line) => line.split(""));
  const rows = grid.length;
  const cols = grid[0].length;

  const directions = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
  ];

  let count = 0;

  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (grid[r][c] !== "@") continue;

      let adjacentRolls = 0;
      for (const [dr, dc] of directions) {
        const nr = r + dr;
        const nc = c + dc;
        if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] === "@") {
          adjacentRolls++;
        }
      }

      if (adjacentRolls < 4) {
        count++;
      }
    }
  }

  return count;
}

export function part2(input: string): number | string {
  const grid = lines(input).map((line) => line.split(""));
  const rows = grid.length;
  const cols = grid[0].length;

  const directions = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
  ];

  const countAdjacentRolls = (r: number, c: number): number => {
    let count = 0;
    for (const [dr, dc] of directions) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] === "@") {
        count++;
      }
    }
    return count;
  };

  let totalRemoved = 0;

  while (true) {
    const toRemove: [number, number][] = [];

    for (let r = 0; r < rows; r++) {
      for (let c = 0; c < cols; c++) {
        if (grid[r][c] === "@" && countAdjacentRolls(r, c) < 4) {
          toRemove.push([r, c]);
        }
      }
    }

    if (toRemove.length === 0) break;

    for (const [r, c] of toRemove) {
      grid[r][c] = ".";
    }
    totalRemoved += toRemove.length;
  }

  return totalRemoved;
}
