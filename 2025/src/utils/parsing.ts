export function lines(input: string): string[] {
  return input.trim().split("\n");
}

export function numbers(input: string): number[] {
  const matches = input.match(/-?\d+/g);
  return matches ? matches.map(Number) : [];
}

export function grid(input: string): string[][] {
  return lines(input).map((line) => line.split(""));
}

export function blocks(input: string): string[] {
  return input.trim().split("\n\n").filter((block) => block.length > 0);
}

export function words(line: string): string[] {
  return line.trim().split(/\s+/);
}

