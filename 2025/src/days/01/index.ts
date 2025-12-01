import { lines } from "../../utils";

export function part1(input: string): number | string {
  const rotations = lines(input);
  let position = 50;
  let zeroCount = 0;

  for (const rotation of rotations) {
    const direction = rotation[0];
    const distance = parseInt(rotation.slice(1), 10);

    if (direction === "L") {
      position = (((position - distance) % 100) + 100) % 100;
    } else {
      position = (position + distance) % 100;
    }

    if (position === 0) {
      zeroCount++;
    }
  }

  return zeroCount;
}

export function part2(input: string): number | string {
  const rotations = lines(input);
  let position = 50;
  let zeroCount = 0;

  for (const rotation of rotations) {
    const direction = rotation[0];
    const distance = parseInt(rotation.slice(1), 10);

    if (direction === "L") {
      const fullRotations = Math.floor(distance / 100);
      const remainder = distance % 100;
      const crossesZero = position > 0 && remainder >= position ? 1 : 0;
      zeroCount += fullRotations + crossesZero;
      position = (((position - distance) % 100) + 100) % 100;
    } else {
      const fullRotations = Math.floor(distance / 100);
      const remainder = distance % 100;
      const crossesZero = position + remainder >= 100 ? 1 : 0;
      zeroCount += fullRotations + crossesZero;
      position = (position + distance) % 100;
    }
  }

  return zeroCount;
}
