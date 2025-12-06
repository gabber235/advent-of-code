import { blocks, lines } from "../../utils";

export function part1(input: string): number | string {
  const [rangesBlock, ingredientsBlock] = blocks(input);

  const ranges = lines(rangesBlock).map((line) => {
    const [start, end] = line.split("-").map(Number);
    return { start, end };
  });

  const ingredients = lines(ingredientsBlock).map(Number);

  let freshCount = 0;
  for (const id of ingredients) {
    const isFresh = ranges.some((range) => id >= range.start && id <= range.end);
    if (isFresh) {
      freshCount++;
    }
  }

  return freshCount;
}

export function part2(input: string): number | string {
  const [rangesBlock] = blocks(input);

  const ranges = lines(rangesBlock).map((line) => {
    const [start, end] = line.split("-").map(Number);
    return { start, end };
  });

  ranges.sort((a, b) => a.start - b.start);

  const merged: { start: number; end: number }[] = [];
  for (const range of ranges) {
    if (merged.length === 0 || merged[merged.length - 1].end < range.start - 1) {
      merged.push({ start: range.start, end: range.end });
    } else {
      merged[merged.length - 1].end = Math.max(merged[merged.length - 1].end, range.end);
    }
  }

  let totalFresh = 0;
  for (const range of merged) {
    totalFresh += range.end - range.start + 1;
  }

  return totalFresh;
}
