import { DAY_TEMPLATE } from "../src/template";
import { mkdir } from "fs/promises";
import { join } from "path";

const dayArg = process.argv[2];

if (!dayArg) {
  console.error("Usage: bun new <day>");
  process.exit(1);
}

const day = parseInt(dayArg, 10);
if (isNaN(day) || day < 1 || day > 25) {
  console.error("Error: Day must be a number between 1 and 25");
  process.exit(1);
}

const dayStr = String(day).padStart(2, "0");
const dayDir = `src/days/${dayStr}`;

try {
  await mkdir(dayDir, { recursive: true });
} catch (error) {
  console.error(`Error creating directory: ${error}`);
  process.exit(1);
}

const indexPath = `${dayDir}/index.ts`;
const inputPath = `${dayDir}/input.txt`;
const examplePath = `${dayDir}/example.txt`;
const expectedPath = `${dayDir}/expected.json`;

try {
  await Bun.write(indexPath, DAY_TEMPLATE);
  await Bun.write(inputPath, "");
  await Bun.write(examplePath, "");
  await Bun.write(expectedPath, JSON.stringify({ part1: "", part2: "" }, null, 2) + "\n");

  console.log(`✅ Created day ${day} files:`);
  console.log(`   - ${indexPath}`);
  console.log(`   - ${inputPath}`);
  console.log(`   - ${examplePath}`);
  console.log(`   - ${expectedPath}`);
} catch (error) {
  console.error(`Error creating files: ${error}`);
  process.exit(1);
}

