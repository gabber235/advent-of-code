export function part1(input: string): number | string {
  const rows = input.split("\n").filter((line) => line.length > 0);
  const operatorRow = rows[rows.length - 1];
  const numberRows = rows.slice(0, -1);

  const problems: { numbers: number[]; operator: string }[] = [];
  let currentProblem: { numbers: number[]; operator: string } | null = null;
  let currentNumberStr: string[] = [];

  const maxLen = Math.max(operatorRow.length, ...numberRows.map((r) => r.length));

  for (let col = 0; col <= maxLen; col++) {
    const operatorChar = col < operatorRow.length ? operatorRow[col] : " ";
    const isOperator = operatorChar === "*" || operatorChar === "+";

    const allNumberRowsSpace = numberRows.every((row) => col >= row.length || row[col] === " ");
    const isSeparator = col === maxLen || (allNumberRowsSpace && operatorChar === " ");

    if (isSeparator) {
      if (currentProblem !== null) {
        for (let row = 0; row < numberRows.length; row++) {
          const numStr = currentNumberStr[row]?.trim();
          if (numStr && numStr.length > 0) {
            currentProblem.numbers.push(parseInt(numStr, 10));
          }
        }
        if (currentProblem.numbers.length > 0 && currentProblem.operator) {
          problems.push(currentProblem);
        }
        currentProblem = null;
        currentNumberStr = [];
      }
    } else {
      if (currentProblem === null) {
        currentProblem = { numbers: [], operator: "" };
        currentNumberStr = numberRows.map(() => "");
      }

      for (let row = 0; row < numberRows.length; row++) {
        const char = col < numberRows[row].length ? numberRows[row][col] : " ";
        currentNumberStr[row] += char;
      }

      if (isOperator) {
        currentProblem.operator = operatorChar;
      }
    }
  }

  let grandTotal = 0;
  for (const problem of problems) {
    let result: number;
    if (problem.operator === "+") {
      result = problem.numbers.reduce((a, b) => a + b, 0);
    } else {
      result = problem.numbers.reduce((a, b) => a * b, 1);
    }
    grandTotal += result;
  }

  return grandTotal;
}

export function part2(input: string): number | string {
  const rows = input.split("\n").filter((line) => line.length > 0);
  const operatorRow = rows[rows.length - 1];
  const numberRows = rows.slice(0, -1);

  const problems: { numbers: number[]; operator: string }[] = [];
  let currentProblem: { numbers: number[]; operator: string } | null = null;
  let currentColumnDigits: string[] = [];

  const maxLen = Math.max(operatorRow.length, ...numberRows.map((r) => r.length));

  for (let col = 0; col <= maxLen; col++) {
    const operatorChar = col < operatorRow.length ? operatorRow[col] : " ";
    const isOperator = operatorChar === "*" || operatorChar === "+";

    const allNumberRowsSpace = numberRows.every((row) => col >= row.length || row[col] === " ");
    const isSeparator = col === maxLen || (allNumberRowsSpace && operatorChar === " ");

    if (isSeparator) {
      if (currentProblem !== null) {
        if (currentColumnDigits.some((d) => d !== " ")) {
          const num = parseInt(currentColumnDigits.join("").trim(), 10);
          if (!isNaN(num)) {
            currentProblem.numbers.push(num);
          }
        }
        if (currentProblem.numbers.length > 0 && currentProblem.operator) {
          problems.push(currentProblem);
        }
        currentProblem = null;
        currentColumnDigits = [];
      }
    } else {
      if (currentProblem === null) {
        currentProblem = { numbers: [], operator: "" };
        currentColumnDigits = [];
      }

      const columnChars = numberRows.map((row) => (col < row.length ? row[col] : " "));
      const allSpaces = columnChars.every((c) => c === " ");

      if (!allSpaces) {
        if (currentColumnDigits.length > 0 && currentColumnDigits.some((d) => d !== " ")) {
          const num = parseInt(currentColumnDigits.join("").trim(), 10);
          if (!isNaN(num)) {
            currentProblem.numbers.push(num);
          }
        }
        currentColumnDigits = columnChars;
      }

      if (isOperator) {
        currentProblem.operator = operatorChar;
      }
    }
  }

  let grandTotal = 0;
  for (const problem of problems) {
    const reversedNumbers = [...problem.numbers].reverse();
    let result: number;
    if (problem.operator === "+") {
      result = reversedNumbers.reduce((a, b) => a + b, 0);
    } else {
      result = reversedNumbers.reduce((a, b) => a * b, 1);
    }
    grandTotal += result;
  }

  return grandTotal;
}
