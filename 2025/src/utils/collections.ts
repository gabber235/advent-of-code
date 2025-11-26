export class Counter<T> extends Map<T, number> {
  constructor(iterable?: Iterable<T>) {
    super();
    if (iterable) {
      for (const item of iterable) {
        this.add(item);
      }
    }
  }

  add(item: T, count: number = 1): void {
    this.set(item, (this.get(item) ?? 0) + count);
  }

  subtract(item: T, count: number = 1): void {
    const current = this.get(item) ?? 0;
    const newCount = current - count;
    if (newCount <= 0) {
      this.delete(item);
    } else {
      this.set(item, newCount);
    }
  }

  mostCommon(n?: number): Array<[T, number]> {
    const entries = Array.from(this.entries());
    entries.sort((a, b) => b[1] - a[1]);
    return n ? entries.slice(0, n) : entries;
  }
}

export class DefaultMap<K, V> extends Map<K, V> {
  constructor(private defaultFactory: (key: K) => V) {
    super();
  }

  get(key: K): V {
    if (!this.has(key)) {
      this.set(key, this.defaultFactory(key));
    }
    return super.get(key)!;
  }
}

export class PriorityQueue<T> {
  private heap: T[] = [];
  private compare: (a: T, b: T) => number;

  constructor(compare: (a: T, b: T) => number = (a, b) => (a as number) - (b as number)) {
    this.compare = compare;
  }

  push(item: T): void {
    this.heap.push(item);
    this.bubbleUp(this.heap.length - 1);
  }

  pop(): T | undefined {
    if (this.heap.length === 0) return undefined;
    if (this.heap.length === 1) return this.heap.pop();

    const top = this.heap[0];
    this.heap[0] = this.heap.pop()!;
    this.bubbleDown(0);
    return top;
  }

  peek(): T | undefined {
    return this.heap[0];
  }

  get size(): number {
    return this.heap.length;
  }

  get empty(): boolean {
    return this.heap.length === 0;
  }

  private bubbleUp(index: number): void {
    while (index > 0) {
      const parent = Math.floor((index - 1) / 2);
      if (this.compare(this.heap[index], this.heap[parent]) >= 0) break;
      [this.heap[index], this.heap[parent]] = [this.heap[parent], this.heap[index]];
      index = parent;
    }
  }

  private bubbleDown(index: number): void {
    while (true) {
      let smallest = index;
      const left = 2 * index + 1;
      const right = 2 * index + 2;

      if (left < this.heap.length && this.compare(this.heap[left], this.heap[smallest]) < 0) {
        smallest = left;
      }
      if (right < this.heap.length && this.compare(this.heap[right], this.heap[smallest]) < 0) {
        smallest = right;
      }

      if (smallest === index) break;
      [this.heap[index], this.heap[smallest]] = [this.heap[smallest], this.heap[index]];
      index = smallest;
    }
  }
}

export function memoize<Args extends unknown[], Return>(
  fn: (...args: Args) => Return
): (...args: Args) => Return {
  const cache = new Map<string, Return>();
  return (...args: Args): Return => {
    const key = JSON.stringify(args);
    if (cache.has(key)) {
      return cache.get(key)!;
    }
    const result = fn(...args);
    cache.set(key, result);
    return result;
  };
}

