export function gcd(a: number, b: number): number {
  while (b !== 0) {
    [a, b] = [b, a % b];
  }
  return Math.abs(a);
}

export function lcm(a: number, b: number): number {
  return Math.abs(a * b) / gcd(a, b);
}

export function sum(arr: number[]): number {
  return arr.reduce((a, b) => a + b, 0);
}

export function product(arr: number[]): number {
  return arr.reduce((a, b) => a * b, 1);
}

export function* range(start: number, end: number, step: number = 1): Generator<number> {
  if (step > 0) {
    for (let i = start; i < end; i += step) {
      yield i;
    }
  } else if (step < 0) {
    for (let i = start; i > end; i += step) {
      yield i;
    }
  }
}

export function mod(n: number, m: number): number {
  return ((n % m) + m) % m;
}

export function clamp(n: number, min: number, max: number): number {
  return Math.min(Math.max(n, min), max);
}

