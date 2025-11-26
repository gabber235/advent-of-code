const colors = {
  reset: "\x1b[0m",
  green: "\x1b[32m",
  red: "\x1b[31m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  cyan: "\x1b[36m",
  gray: "\x1b[90m",
};

function colorize(text: string, color: keyof typeof colors): string {
  return `${colors[color]}${text}${colors.reset}`;
}

async function runDay(day: number, useExample: boolean = false) {
  const dayStr = String(day).padStart(2, "0");
  const inputFile = useExample ? "example.txt" : "input.txt";
  const inputPath = `src/days/${dayStr}/${inputFile}`;

  let input: string;
  try {
    const file = Bun.file(inputPath);
    input = await file.text();
  } catch (error) {
    console.error(colorize(`Error: Could not read ${inputPath}`, "red"));
    process.exit(1);
  }

  let solution: { part1: (input: string) => number | string; part2: (input: string) => number | string };
  try {
    solution = await import(`./days/${dayStr}/index.ts`);
  } catch (error) {
    console.error(colorize(`Error: Could not load solution for day ${day}`, "red"));
    process.exit(1);
  }

  console.log(colorize(`\n🎄 Day ${day}${useExample ? " (example)" : ""}`, "cyan"));
  console.log(colorize("─".repeat(40), "gray"));

  const runPart = (part: "part1" | "part2") => {
    const start = performance.now();
    try {
      const result = solution[part](input);
      const elapsed = performance.now() - start;
      const timeStr = elapsed < 1 ? `${(elapsed * 1000).toFixed(2)}μs` : `${elapsed.toFixed(2)}ms`;
      console.log(
        `${colorize(`Part ${part.slice(-1)}:`, "blue")} ${colorize(String(result), "green")} ${colorize(`(${timeStr})`, "gray")}`
      );
      return result;
    } catch (error) {
      const elapsed = performance.now() - start;
      const timeStr = elapsed < 1 ? `${(elapsed * 1000).toFixed(2)}μs` : `${elapsed.toFixed(2)}ms`;
      console.log(
        `${colorize(`Part ${part.slice(-1)}:`, "blue")} ${colorize(`Error: ${error}`, "red")} ${colorize(`(${timeStr})`, "gray")}`
      );
      throw error;
    }
  };

  runPart("part1");
  runPart("part2");
  console.log();
}

const dayArg = process.argv[2];
const useExample = process.argv.includes("--example") || process.argv.includes("-e");

if (!dayArg) {
  console.error(colorize("Usage: bun day <day> [--example]", "red"));
  process.exit(1);
}

const day = parseInt(dayArg, 10);
if (isNaN(day) || day < 1 || day > 25) {
  console.error(colorize("Error: Day must be a number between 1 and 25", "red"));
  process.exit(1);
}

runDay(day, useExample);

