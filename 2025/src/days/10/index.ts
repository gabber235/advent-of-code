import { lines } from "../../utils";

interface Machine {
  targetLights: number[];
  targetJoltage: number[];
  buttons: number[][];
}

function parseMachine(line: string): Machine {
  const indicatorMatch = line.match(/\[([.#]+)\]/);
  if (!indicatorMatch) throw new Error("Invalid line format");

  const targetLights = indicatorMatch[1].split("").map((c) => (c === "#" ? 1 : 0));

  const buttonMatches = line.matchAll(/\(([^)]+)\)/g);
  const buttons: number[][] = [];

  for (const match of buttonMatches) {
    const indices = match[1].split(",").map(Number);
    buttons.push(indices);
  }

  const joltageMatch = line.match(/\{([^}]+)\}/);
  const targetJoltage = joltageMatch ? joltageMatch[1].split(",").map(Number) : [];

  return { targetLights, targetJoltage, buttons };
}

function solveMinPressesGF2(machine: Machine): number {
  const target = machine.targetLights;
  const buttons = machine.buttons;
  const numLights = target.length;
  const numButtons = buttons.length;

  // Build the augmented matrix for Gaussian elimination over GF(2)
  // Each row represents a light, each column a button, plus the target column
  const matrix: number[][] = [];
  for (let light = 0; light < numLights; light++) {
    const row: number[] = [];
    for (let btn = 0; btn < numButtons; btn++) {
      row.push(buttons[btn].includes(light) ? 1 : 0);
    }
    row.push(target[light]);
    matrix.push(row);
  }

  // Gaussian elimination over GF(2) to find RREF
  const pivotCols: number[] = [];
  let pivotRow = 0;

  for (let col = 0; col < numButtons && pivotRow < numLights; col++) {
    // Find pivot in this column
    let found = -1;
    for (let row = pivotRow; row < numLights; row++) {
      if (matrix[row][col] === 1) {
        found = row;
        break;
      }
    }

    if (found === -1) continue;

    // Swap rows
    [matrix[pivotRow], matrix[found]] = [matrix[found], matrix[pivotRow]];
    pivotCols.push(col);

    // Eliminate all other 1s in this column
    for (let row = 0; row < numLights; row++) {
      if (row !== pivotRow && matrix[row][col] === 1) {
        for (let c = 0; c <= numButtons; c++) {
          matrix[row][c] ^= matrix[pivotRow][c];
        }
      }
    }

    pivotRow++;
  }

  // Check for inconsistency (row with all zeros except last column)
  for (let row = pivotRow; row < numLights; row++) {
    if (matrix[row][numButtons] === 1) {
      return Infinity; // No solution
    }
  }

  // Find free variables (columns that are not pivot columns)
  const freeVars: number[] = [];
  for (let col = 0; col < numButtons; col++) {
    if (!pivotCols.includes(col)) {
      freeVars.push(col);
    }
  }

  const numFree = freeVars.length;

  // Enumerate all 2^numFree combinations of free variables to find minimum weight solution
  let minPresses = Infinity;

  for (let mask = 0; mask < 1 << numFree; mask++) {
    const solution: number[] = new Array(numButtons).fill(0);

    // Set free variables according to mask
    for (let i = 0; i < numFree; i++) {
      solution[freeVars[i]] = (mask >> i) & 1;
    }

    // Back-substitute to find pivot variables
    for (let i = pivotCols.length - 1; i >= 0; i--) {
      const col = pivotCols[i];
      let val = matrix[i][numButtons];
      for (let c = col + 1; c < numButtons; c++) {
        val ^= matrix[i][c] * solution[c];
      }
      solution[col] = val;
    }

    // Count number of presses
    const presses = solution.reduce((a, b) => a + b, 0);
    if (presses < minPresses) {
      minPresses = presses;
    }
  }

  return minPresses;
}

function solveMinPressesInteger(machine: Machine): number {
  const target = machine.targetJoltage;
  const buttons = machine.buttons;
  const numCounters = target.length;
  const numButtons = buttons.length;

  // Build the matrix A where A[counter][button] = 1 if button affects counter
  const A: number[][] = [];
  for (let counter = 0; counter < numCounters; counter++) {
    const row: number[] = [];
    for (let btn = 0; btn < numButtons; btn++) {
      row.push(buttons[btn].includes(counter) ? 1 : 0);
    }
    A.push(row);
  }

  // The maximum value any button needs to be pressed is bounded by max(target)
  // because each button press adds at least 1 to at least one counter
  const maxPresses = Math.max(...target);

  // Use BFS/dynamic programming approach
  // State: current counter values
  // But this can be exponential in state space...

  // Better approach: Since we have numCounters constraints and numButtons variables,
  // and the system is typically underdetermined (more buttons than counters),
  // we can enumerate over subsets of buttons of size equal to numCounters

  // Actually, let's use a direct bounded search approach
  // For small number of free variables after Gaussian elimination, enumerate
  // For larger, use ILP-style branch and bound

  // Build augmented matrix [A | target]
  const matrix: number[][] = A.map((row, i) => [...row, target[i]]);

  // Gaussian elimination to find RREF (using integer arithmetic to avoid floating point issues)
  const pivotCols: number[] = [];
  let pivotRow = 0;

  for (let col = 0; col < numButtons && pivotRow < numCounters; col++) {
    // Find pivot
    let found = -1;
    for (let row = pivotRow; row < numCounters; row++) {
      if (matrix[row][col] !== 0) {
        found = row;
        break;
      }
    }

    if (found === -1) continue;

    // Swap rows
    [matrix[pivotRow], matrix[found]] = [matrix[found], matrix[pivotRow]];
    pivotCols.push(col);

    // Eliminate all other entries in this column
    for (let row = 0; row < numCounters; row++) {
      if (row !== pivotRow && matrix[row][col] !== 0) {
        // Scale both rows to eliminate
        const a = matrix[pivotRow][col];
        const b = matrix[row][col];
        for (let c = 0; c <= numButtons; c++) {
          matrix[row][c] = matrix[row][c] * a - matrix[pivotRow][c] * b;
        }
      }
    }

    pivotRow++;
  }

  // Now matrix is in echelon form (not necessarily reduced)
  // Check for inconsistency
  for (let row = pivotRow; row < numCounters; row++) {
    let allZero = true;
    for (let col = 0; col < numButtons; col++) {
      if (matrix[row][col] !== 0) {
        allZero = false;
        break;
      }
    }
    if (allZero && matrix[row][numButtons] !== 0) {
      return Infinity;
    }
  }

  // Find free variables
  const freeVars: number[] = [];
  for (let col = 0; col < numButtons; col++) {
    if (!pivotCols.includes(col)) {
      freeVars.push(col);
    }
  }

  // For a solution, given values of free variables, solve for pivot variables
  // We need to handle the non-reduced echelon form
  const getSolutionFromFree = (freeValues: number[]): number[] | null => {
    const solution: number[] = new Array(numButtons).fill(0);

    // Set free variables
    for (let i = 0; i < freeVars.length; i++) {
      solution[freeVars[i]] = freeValues[i];
    }

    // Back-substitute to find pivot variables
    for (let i = pivotCols.length - 1; i >= 0; i--) {
      const col = pivotCols[i];
      let rhs = matrix[i][numButtons];
      for (let c = col + 1; c < numButtons; c++) {
        rhs -= matrix[i][c] * solution[c];
      }
      // solution[col] = rhs / matrix[i][col]
      if (rhs % matrix[i][col] !== 0) {
        return null; // Not an integer solution
      }
      solution[col] = rhs / matrix[i][col];
    }

    // Check all non-negative
    for (let j = 0; j < numButtons; j++) {
      if (solution[j] < 0) {
        return null;
      }
    }

    return solution;
  };

  // Enumerate all combinations of free variable values
  // Free variables can range from 0 to maxPresses
  let minPresses = Infinity;

  const enumerate = (depth: number, freeValues: number[]): void => {
    if (depth === freeVars.length) {
      const sol = getSolutionFromFree(freeValues);
      if (sol) {
        const total = sol.reduce((a, b) => a + b, 0);
        if (total < minPresses) {
          minPresses = total;
        }
      }
      return;
    }

    // Pruning: current sum of free values already set
    const currentFreeSum = freeValues.reduce((a, b) => a + b, 0);
    if (currentFreeSum >= minPresses) return;

    for (let v = 0; v <= maxPresses; v++) {
      if (currentFreeSum + v >= minPresses) break;
      freeValues.push(v);
      enumerate(depth + 1, freeValues);
      freeValues.pop();
    }
  };

  enumerate(0, []);

  return minPresses;
}

export function part1(input: string): number | string {
  const machines = lines(input).map(parseMachine);
  let total = 0;

  for (const machine of machines) {
    const presses = solveMinPressesGF2(machine);
    if (presses === Infinity) {
      throw new Error("No solution found for a machine");
    }
    total += presses;
  }

  return total;
}

export function part2(input: string): number | string {
  const machines = lines(input).map(parseMachine);
  let total = 0;

  for (const machine of machines) {
    const presses = solveMinPressesInteger(machine);
    if (presses === Infinity) {
      throw new Error("No solution found for a machine");
    }
    total += presses;
  }

  return total;
}
