type Point = [number, number];

export class Grid<T> {
  private data: T[][];
  public readonly width: number;
  public readonly height: number;

  constructor(data: T[][]) {
    this.data = data;
    this.height = data.length;
    this.width = data[0]?.length ?? 0;
  }

  static fromString(input: string): Grid<string> {
    const lines = input.trim().split("\n");
    const data = lines.map((line) => line.split(""));
    return new Grid(data);
  }

  get(x: number, y: number): T | undefined {
    if (y < 0 || y >= this.height || x < 0 || x >= this.width) {
      return undefined;
    }
    return this.data[y][x];
  }

  set(x: number, y: number, value: T): void {
    if (y >= 0 && y < this.height && x >= 0 && x < this.width) {
      this.data[y][x] = value;
    }
  }

  neighbors4(x: number, y: number): Point[] {
    const dirs: Point[] = [
      [0, -1],
      [1, 0],
      [0, 1],
      [-1, 0],
    ];
    return dirs
      .map(([dx, dy]) => [x + dx, y + dy] as Point)
      .filter(([nx, ny]) => this.get(nx, ny) !== undefined);
  }

  neighbors8(x: number, y: number): Point[] {
    const dirs: Point[] = [
      [-1, -1],
      [0, -1],
      [1, -1],
      [-1, 0],
      [1, 0],
      [-1, 1],
      [0, 1],
      [1, 1],
    ];
    return dirs
      .map(([dx, dy]) => [x + dx, y + dy] as Point)
      .filter(([nx, ny]) => this.get(nx, ny) !== undefined);
  }

  find(predicate: (value: T, x: number, y: number) => boolean): Point | undefined {
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        if (predicate(this.data[y][x], x, y)) {
          return [x, y];
        }
      }
    }
    return undefined;
  }

  findAll(value: T): Point[] {
    const points: Point[] = [];
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        if (this.data[y][x] === value) {
          points.push([x, y]);
        }
      }
    }
    return points;
  }

  forEach(callback: (value: T, x: number, y: number) => void): void {
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        callback(this.data[y][x], x, y);
      }
    }
  }

  clone(): Grid<T> {
    const newData = this.data.map((row) => [...row]);
    return new Grid(newData);
  }

  bfs(
    start: Point,
    goal: Point | ((value: T, x: number, y: number) => boolean),
    getNeighbors: (x: number, y: number) => Point[] = (x, y) => this.neighbors4(x, y)
  ): { path: Point[]; distance: number } | null {
    const queue: Array<{ point: Point; path: Point[] }> = [{ point: start, path: [start] }];
    const visited = new Set<string>();
    visited.add(`${start[0]},${start[1]}`);

    const isGoal = (x: number, y: number): boolean => {
      if (typeof goal === "function") {
        const value = this.get(x, y);
        return value !== undefined && goal(value, x, y);
      }
      return x === goal[0] && y === goal[1];
    };

    while (queue.length > 0) {
      const { point, path } = queue.shift()!;
      const [x, y] = point;

      if (isGoal(x, y)) {
        return { path, distance: path.length - 1 };
      }

      for (const neighbor of getNeighbors(x, y)) {
        const key = `${neighbor[0]},${neighbor[1]}`;
        if (!visited.has(key)) {
          visited.add(key);
          queue.push({ point: neighbor, path: [...path, neighbor] });
        }
      }
    }

    return null;
  }
}

