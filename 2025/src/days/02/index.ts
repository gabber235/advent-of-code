export function part1(input: string): number | string {
  const ranges = input
    .trim()
    .split(",")
    .filter((r) => r.length > 0);

  let sum = 0;

  for (const range of ranges) {
    const [startStr, endStr] = range.split("-");
    const start = parseInt(startStr, 10);
    const end = parseInt(endStr, 10);

    for (let id = start; id <= end; id++) {
      if (isInvalidId(id)) {
        sum += id;
      }
    }
  }

  return sum;
}

function isInvalidId(id: number): boolean {
  const str = id.toString();
  const len = str.length;

  if (len % 2 !== 0) {
    return false;
  }

  const half = len / 2;
  const firstHalf = str.substring(0, half);
  const secondHalf = str.substring(half);

  return firstHalf === secondHalf;
}

function isInvalidIdPart2(id: number): boolean {
  const str = id.toString();
  const len = str.length;

  for (let patternLen = 1; patternLen <= len / 2; patternLen++) {
    if (len % patternLen !== 0) {
      continue;
    }

    const pattern = str.substring(0, patternLen);
    let matches = true;

    for (let i = patternLen; i < len; i += patternLen) {
      if (str.substring(i, i + patternLen) !== pattern) {
        matches = false;
        break;
      }
    }

    if (matches) {
      return true;
    }
  }

  return false;
}

export function part2(input: string): number | string {
  const ranges = input
    .trim()
    .split(",")
    .filter((r) => r.length > 0);

  let sum = 0;

  for (const range of ranges) {
    const [startStr, endStr] = range.split("-");
    const start = parseInt(startStr, 10);
    const end = parseInt(endStr, 10);

    for (let id = start; id <= end; id++) {
      if (isInvalidIdPart2(id)) {
        sum += id;
      }
    }
  }

  return sum;
}
