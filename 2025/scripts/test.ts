import { readdir } from "fs/promises";
import { join } from "path";

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

async function testDay(day: number): Promise<boolean> {
  const dayStr = String(day).padStart(2, "0");
  const dayDir = `src/days/${dayStr}`;

  const examplePath = join(dayDir, "example.txt");
  const expectedPath = join(dayDir, "expected.json");

  let example: string;
  let expected: { part1: string | number; part2: string | number };

  try {
    const exampleFile = Bun.file(examplePath);
    example = await exampleFile.text();
  } catch (error) {
    console.log(colorize(`⚠️  Day ${day}: No example.txt found, skipping`, "yellow"));
    return true;
  }

  try {
    const expectedFile = Bun.file(expectedPath);
    const expectedText = await expectedFile.text();
    expected = JSON.parse(expectedText);
  } catch (error) {
    console.log(colorize(`⚠️  Day ${day}: No expected.json found, skipping`, "yellow"));
    return true;
  }

  let solution: { part1: (input: string) => number | string; part2: (input: string) => number | string };
  try {
    solution = await import(`../src/days/${dayStr}/index.ts`);
  } catch (error) {
    console.log(colorize(`❌ Day ${day}: Could not load solution`, "red"));
    return false;
  }

  const results: { part: string; passed: boolean; expected: string; got: string }[] = [];

  const testPart = (part: "part1" | "part2") => {
    try {
      const result = String(solution[part](example));
      const expectedStr = String(expected[part]);
      const passed = result === expectedStr;
      results.push({ part, passed, expected: expectedStr, got: result });
    } catch (error) {
      results.push({ part, passed: false, expected: String(expected[part]), got: `Error: ${error}` });
    }
  };

  testPart("part1");
  testPart("part2");

  const allPassed = results.every((r) => r.passed);

  if (allPassed) {
    console.log(colorize(`✅ Day ${day}: All tests passed`, "green"));
  } else {
    console.log(colorize(`❌ Day ${day}: Tests failed`, "red"));
    for (const result of results) {
      if (result.passed) {
        console.log(`   ${colorize(`✓ Part ${result.part.slice(-1)}:`, "green")} ${result.got}`);
      } else {
        console.log(`   ${colorize(`✗ Part ${result.part.slice(-1)}:`, "red")} expected ${colorize(result.expected, "yellow")}, got ${colorize(result.got, "red")}`);
      }
    }
  }

  return allPassed;
}

const dayArg = process.argv[2];

if (dayArg) {
  const day = parseInt(dayArg, 10);
  if (isNaN(day) || day < 1 || day > 25) {
    console.error(colorize("Error: Day must be a number between 1 and 25", "red"));
    process.exit(1);
  }
  const passed = await testDay(day);
  process.exit(passed ? 0 : 1);
} else {
  const daysDir = "src/days";
  let dayDirs: string[];
  try {
    dayDirs = await readdir(daysDir);
  } catch (error) {
    console.error(colorize(`Error reading ${daysDir}: ${error}`, "red"));
    process.exit(1);
  }

  const days = dayDirs
    .filter((dir) => /^\d{2}$/.test(dir))
    .map((dir) => parseInt(dir, 10))
    .sort((a, b) => a - b);

  if (days.length === 0) {
    console.log(colorize("No days found to test", "yellow"));
    process.exit(0);
  }

  console.log(colorize(`\n🧪 Running tests for ${days.length} day(s)\n`, "cyan"));

  let passedCount = 0;
  for (const day of days) {
    if (await testDay(day)) {
      passedCount++;
    }
  }

  console.log(colorize(`\n${passedCount}/${days.length} day(s) passed`, passedCount === days.length ? "green" : "yellow"));
  process.exit(passedCount === days.length ? 0 : 1);
}

