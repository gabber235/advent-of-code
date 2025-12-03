import { lines } from "../../utils";

function maxKDigits(bank: string, k: number): bigint {
  const n = bank.length;
  let result = "";
  let startPos = 0;

  for (let i = 0; i < k; i++) {
    const endPos = n - (k - i);

    let maxDigit = "0";
    let maxPos = startPos;
    for (let j = startPos; j <= endPos; j++) {
      if (bank[j] > maxDigit) {
        maxDigit = bank[j];
        maxPos = j;
      }
    }

    result += maxDigit;
    startPos = maxPos + 1;
  }

  console.log(result);
  return BigInt(result);
}

export function part1(input: string): number | string {
  const banks = lines(input);
  const total = banks.reduce((sum, bank) => sum + maxKDigits(bank, 2), 0n);
  return total.toString();
}

export function part2(input: string): number | string {
  const banks = lines(input);
  const total = banks.reduce((sum, bank) => sum + maxKDigits(bank, 12), 0n);
  return total.toString();
}
