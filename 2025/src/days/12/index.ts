import { lines } from "../../utils";

type Orientation = { width: number; height: number; rows: bigint[] };
type Shape = { area: number; orientations: Orientation[]; maxDim: number };
type Region = { width: number; height: number; counts: number[] };

type Coord = readonly [number, number];

function normalizeCells(cells: Coord[]): Coord[] {
  let minX = Infinity;
  let minY = Infinity;
  for (const [x, y] of cells) {
    if (x < minX) minX = x;
    if (y < minY) minY = y;
  }

  const normalized = cells.map(([x, y]) => [x - minX, y - minY] as const);
  normalized.sort((a, b) => a[1] - b[1] || a[0] - b[0]);
  return normalized;
}

function cellsToOrientation(cells: Coord[]): Orientation {
  let maxX = 0;
  let maxY = 0;
  for (const [x, y] of cells) {
    if (x > maxX) maxX = x;
    if (y > maxY) maxY = y;
  }

  const width = maxX + 1;
  const height = maxY + 1;
  const rows = Array<bigint>(height).fill(0n);
  for (const [x, y] of cells) rows[y] |= 1n << BigInt(x);
  return { width, height, rows };
}

function orientationsFromGrid(grid: string[]): Orientation[] {
  const cells: Coord[] = [];
  for (let y = 0; y < grid.length; y++) {
    const row = grid[y];
    for (let x = 0; x < row.length; x++) {
      if (row[x] === "#") cells.push([x, y]);
    }
  }

  const seen = new Map<string, Orientation>();
  for (const flip of [false, true]) {
    for (let rot = 0; rot < 4; rot++) {
      const transformed = cells.map(([x0, y0]) => {
        let x = flip ? -x0 : x0;
        let y = y0;
        if (rot === 1) [x, y] = [y, -x];
        if (rot === 2) [x, y] = [-x, -y];
        if (rot === 3) [x, y] = [-y, x];
        return [x, y] as const;
      });

      const normalized = normalizeCells(transformed);
      const key = normalized.map(([x, y]) => `${x},${y}`).join(";");
      if (seen.has(key)) continue;
      seen.set(key, cellsToOrientation(normalized));
    }
  }

  return [...seen.values()];
}

function parseInput(input: string): { shapes: Shape[]; regions: Region[]; blockSize: number } {
  const data = lines(input);

  const rawGrids: string[][] = [];
  let i = 0;
  while (i < data.length) {
    const m = data[i].trim().match(/^(\d+):$/);
    if (!m) break;
    const idx = Number(m[1]);
    i++;

    const grid: string[] = [];
    while (i < data.length && data[i].trim() !== "") {
      grid.push(data[i].trim());
      i++;
    }

    while (i < data.length && data[i].trim() === "") i++;
    rawGrids[idx] = grid;
  }

  const shapes: Shape[] = rawGrids.map((grid) => {
    const orientations = orientationsFromGrid(grid);
    const area = grid.reduce((sum, row) => sum + [...row].filter((c) => c === "#").length, 0);
    const maxDim = Math.max(...orientations.map((o) => Math.max(o.width, o.height)));
    return { area, orientations, maxDim };
  });

  const blockSize = shapes.reduce((m, s) => Math.max(m, s.maxDim), 0);

  const regions: Region[] = [];
  for (; i < data.length; i++) {
    const line = data[i].trim();
    if (!line) continue;
    const m = line.match(/^(\d+)x(\d+):\s*(.*)$/);
    if (!m) continue;
    const width = Number(m[1]);
    const height = Number(m[2]);
    const counts = m[3].trim().split(/\s+/).filter(Boolean).map(Number);
    regions.push({ width, height, counts });
  }

  return { shapes, regions, blockSize };
}

type Placement = { y: number; rows: bigint[] };

function buildPlacements(shape: Shape, boardWidth: number, boardHeight: number): Placement[] {
  const placements: Placement[] = [];
  for (const o of shape.orientations) {
    if (o.width > boardWidth || o.height > boardHeight) continue;
    for (let y = 0; y <= boardHeight - o.height; y++) {
      for (let x = 0; x <= boardWidth - o.width; x++) {
        const shift = BigInt(x);
        placements.push({ y, rows: o.rows.map((r) => r << shift) });
      }
    }
  }
  return placements;
}

function canFitExactly(width: number, height: number, counts: number[], shapes: Shape[]): boolean {
  const placementsByShape: Placement[][] = Array.from({ length: shapes.length }, () => []);
  const placementCounts = new Array<number>(shapes.length).fill(0);

  for (let s = 0; s < counts.length; s++) {
    if (counts[s] === 0) continue;
    const placements = buildPlacements(shapes[s], width, height);
    if (placements.length === 0) return false;
    placementsByShape[s] = placements;
    placementCounts[s] = placements.length;
  }

  const pieces: number[] = [];
  for (let s = 0; s < counts.length; s++) {
    for (let k = 0; k < counts[s]; k++) pieces.push(s);
  }

  pieces.sort((a, b) => {
    const pa = placementCounts[a];
    const pb = placementCounts[b];
    if (pa !== pb) return pa - pb;
    const aa = shapes[a].area;
    const ab = shapes[b].area;
    if (aa !== ab) return ab - aa;
    return a - b;
  });

  const board = Array<bigint>(height).fill(0n);
  const chosen = new Array<number>(pieces.length).fill(0);
  const memo = new Set<string>();

  const dfs = (idx: number): boolean => {
    if (idx === pieces.length) return true;

    const shapeIdx = pieces[idx];
    const placements = placementsByShape[shapeIdx];
    let start = 0;
    if (idx > 0 && pieces[idx - 1] === shapeIdx) start = chosen[idx - 1];

    const key = `${idx},${start}|${board.join(",")}`;
    if (memo.has(key)) return false;

    for (let pIdx = start; pIdx < placements.length; pIdx++) {
      const p = placements[pIdx];
      let ok = true;
      for (let dy = 0; dy < p.rows.length; dy++) {
        if ((board[p.y + dy] & p.rows[dy]) !== 0n) {
          ok = false;
          break;
        }
      }
      if (!ok) continue;

      for (let dy = 0; dy < p.rows.length; dy++) board[p.y + dy] |= p.rows[dy];
      chosen[idx] = pIdx;
      if (dfs(idx + 1)) return true;
      for (let dy = 0; dy < p.rows.length; dy++) board[p.y + dy] ^= p.rows[dy];
    }

    memo.add(key);
    return false;
  };

  return dfs(0);
}

function canFitRegion(region: Region, shapes: Shape[], blockSize: number): boolean {
  const area = region.width * region.height;
  let pieces = 0;
  let occupied = 0;

  for (let s = 0; s < region.counts.length; s++) {
    const count = region.counts[s];
    if (count === 0) continue;
    pieces += count;
    occupied += count * shapes[s].area;
    const fits = shapes[s].orientations.some((o) => o.width <= region.width && o.height <= region.height);
    if (!fits) return false;
  }

  if (occupied > area) return false;
  if (pieces === 0) return true;

  const blockCapacity = Math.floor(region.width / blockSize) * Math.floor(region.height / blockSize);
  if (pieces <= blockCapacity) return true;

  return canFitExactly(region.width, region.height, region.counts, shapes);
}

export function part1(input: string): number | string {
  const { shapes, regions, blockSize } = parseInput(input);
  let count = 0;
  for (const region of regions) if (canFitRegion(region, shapes, blockSize)) count++;
  return count;
}

export function part2(input: string): number | string {
  return "";
}
