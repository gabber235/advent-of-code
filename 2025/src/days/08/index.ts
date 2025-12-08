import { lines } from "../../utils";

type Point = [number, number, number];

class UnionFind {
  parent: number[];
  rank: number[];
  size: number[];

  constructor(n: number) {
    this.parent = Array.from({ length: n }, (_, i) => i);
    this.rank = new Array(n).fill(0);
    this.size = new Array(n).fill(1);
  }

  find(x: number): number {
    if (this.parent[x] !== x) {
      this.parent[x] = this.find(this.parent[x]);
    }
    return this.parent[x];
  }

  union(x: number, y: number): boolean {
    const rootX = this.find(x);
    const rootY = this.find(y);

    if (rootX === rootY) return false;

    if (this.rank[rootX] < this.rank[rootY]) {
      this.parent[rootX] = rootY;
      this.size[rootY] += this.size[rootX];
    } else if (this.rank[rootX] > this.rank[rootY]) {
      this.parent[rootY] = rootX;
      this.size[rootX] += this.size[rootY];
    } else {
      this.parent[rootY] = rootX;
      this.size[rootX] += this.size[rootY];
      this.rank[rootX]++;
    }
    return true;
  }

  getSize(x: number): number {
    return this.size[this.find(x)];
  }

  getCircuitSizes(): number[] {
    const sizes = new Map<number, number>();
    for (let i = 0; i < this.parent.length; i++) {
      const root = this.find(i);
      if (!sizes.has(root)) {
        sizes.set(root, this.size[root]);
      }
    }
    return Array.from(sizes.values()).sort((a, b) => b - a);
  }
}

function parsePoints(input: string): Point[] {
  return lines(input).map((line) => {
    const [x, y, z] = line.split(",").map(Number);
    return [x, y, z] as Point;
  });
}

function distanceSquared(a: Point, b: Point): number {
  const dx = a[0] - b[0];
  const dy = a[1] - b[1];
  const dz = a[2] - b[2];
  return dx * dx + dy * dy + dz * dz;
}

function getAllPairDistances(points: Point[]): { i: number; j: number; dist: number }[] {
  const pairs: { i: number; j: number; dist: number }[] = [];
  for (let i = 0; i < points.length; i++) {
    for (let j = i + 1; j < points.length; j++) {
      pairs.push({ i, j, dist: distanceSquared(points[i], points[j]) });
    }
  }
  return pairs.sort((a, b) => a.dist - b.dist);
}

function solve(input: string, numConnections: number): number {
  const points = parsePoints(input);
  const pairs = getAllPairDistances(points);
  const uf = new UnionFind(points.length);

  let connections = 0;
  for (const pair of pairs) {
    if (connections >= numConnections) break;
    uf.union(pair.i, pair.j);
    connections++;
  }

  const sizes = uf.getCircuitSizes();
  return sizes[0] * sizes[1] * sizes[2];
}

export function part1(input: string): number | string {
  const points = parsePoints(input);
  const numConnections = points.length <= 20 ? 10 : 1000;
  return solve(input, numConnections);
}

export function part2(input: string): number | string {
  const points = parsePoints(input);
  const pairs = getAllPairDistances(points);
  const uf = new UnionFind(points.length);

  for (const pair of pairs) {
    const rootI = uf.find(pair.i);
    const rootJ = uf.find(pair.j);

    if (rootI !== rootJ) {
      uf.union(pair.i, pair.j);

      if (uf.getSize(pair.i) === points.length) {
        return points[pair.i][0] * points[pair.j][0];
      }
    }
  }

  return 0;
}
