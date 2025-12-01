use advent_derive::*;
use advent_puzzles::{clear_all_caches_and_stats, print_memoize_stats};
use chrono::Datelike;
use clap::Parser;
use colored::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Parser)]
#[clap(name = "Advent of code", version = "1.0", author = "gabber235")]
enum Cli {
    Run {
        #[clap(value_enum)]
        year: Years,
        #[clap(value_enum, short, long, default_value = "today")]
        day: Days,
        #[clap(value_enum, short, long, default_value = "both")]
        part: Part,
        #[clap(short, long)]
        example: bool,
    },
    Bench {
        #[clap(value_enum)]
        year: Years,
        #[clap(value_enum, short, long, default_value = "today")]
        day: Days,
    },
    Test {
        #[clap(value_enum)]
        year: Years,
        #[clap(value_enum, short, long)]
        day: Option<Days>,
    },
    New {
        #[clap(short, long)]
        year: u16,
        #[clap(short, long)]
        day: u8,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, clap::ValueEnum)]
enum Part {
    #[clap(name = "1")]
    Part1,
    #[clap(name = "2")]
    Part2,
    #[clap(name = "both")]
    Both,
}

years_enum!();

days_enum!();

#[derive(Debug, Deserialize, Default)]
struct ExpectedAnswers {
    #[serde(default)]
    example: PartAnswers,
    #[serde(default)]
    real: Option<PartAnswers>,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct PartAnswers {
    #[serde(default)]
    part1: String,
    #[serde(default)]
    part2: String,
}

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Run {
            year,
            day,
            part,
            example,
        } => handle_run(year, day, part, example),
        Cli::Bench { year, day } => handle_bench(year, day),
        Cli::Test { year, day } => handle_test(year, day),
        Cli::New { year, day } => handle_new(year, day),
    }
}

fn handle_run(year: Years, day: Days, part: Part, example: bool) {
    let year_num: u16 = year.into();
    let day_num: u8 = day.into();

    println!(
        "\n\n{} {} {}",
        "▶".cyan(),
        format!("Year {} Day {}", year_num, day_num).cyan().bold(),
        if example {
            "(example)".yellow()
        } else {
            "(real input)".normal()
        }
    );
    println!("{}", "─".repeat(40).dimmed());

    match part {
        Part::Part1 => {
            let input = find_input(year_num, day_num, example, false);
            run_part(year_num, day_num, false, &input, "Part 1");
        }
        Part::Part2 => {
            let input = find_input(year_num, day_num, example, true);
            run_part(year_num, day_num, true, &input, "Part 2");
        }
        Part::Both => {
            let input1 = find_input(year_num, day_num, example, false);
            run_part(year_num, day_num, false, &input1, "Part 1");

            let input2 = find_input(year_num, day_num, example, true);
            run_part(year_num, day_num, true, &input2, "Part 2");
        }
    }
}

fn run_part(year: u16, day: u8, part: bool, input: &str, label: &str) {
    clear_all_caches_and_stats();
    let start = Instant::now();
    let result = advent_puzzles::run_day(year, day, part, input.to_string());
    let elapsed = start.elapsed();

    let time_str = format_duration(elapsed);
    println!(
        "{}: {} {}",
        label.bold(),
        result.green().bold(),
        format!("({})", time_str).dimmed()
    );
    print_memoize_stats();
}

fn format_duration(duration: std::time::Duration) -> String {
    let nanos = duration.as_nanos();
    if nanos < 1_000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.2}µs", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.2}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", nanos as f64 / 1_000_000_000.0)
    }
}

fn handle_bench(year: Years, day: Days) {
    let year: u16 = year.into();
    let day: u8 = day.into();
    let input = find_input(year, day, false, false);

    let mut criterion = criterion::Criterion::default()
        .without_plots()
        .warm_up_time(std::time::Duration::from_millis(2000))
        .measurement_time(std::time::Duration::from_millis(20000))
        .with_output_color(true);

    let mut group = criterion.benchmark_group(format!("{} day {}", year, day));

    group.bench_function("part 1", |b| {
        b.iter(|| advent_puzzles::run_day(year, day, false, input.clone()))
    });

    group.bench_function("part 2", |b| {
        b.iter(|| advent_puzzles::run_day(year, day, true, input.clone()))
    });

    group.finish();

    criterion.final_summary();
}

fn handle_test(year: Years, day: Option<Days>) {
    let year_num: u16 = year.into();

    match day {
        Some(d) => {
            let day_num: u8 = d.into();
            test_single_day(year_num, day_num);
        }
        None => {
            test_all_days(year_num);
        }
    }
}

fn test_single_day(year: u16, day: u8) {
    println!(
        "{} {} {}",
        "🧪".cyan(),
        format!("Testing Year {} Day {}", year, day).cyan().bold(),
        ""
    );
    println!("{}", "─".repeat(40).dimmed());

    let (passed, failed) = run_day_tests(year, day, true);

    println!("{}", "─".repeat(40).dimmed());
    print_test_summary(passed, failed);
}

fn test_all_days(year: u16) {
    println!(
        "{} {}",
        "🧪".cyan(),
        format!("Testing all days for Year {}", year).cyan().bold()
    );
    println!("{}", "═".repeat(40).dimmed());

    let year_path = format!("advent-puzzles/src/{}", year);
    if !Path::new(&year_path).exists() {
        eprintln!("{}", format!("Year {} not found", year).red());
        std::process::exit(1);
    }

    let mut total_passed = 0;
    let mut total_failed = 0;
    let mut days_with_tests = 0;

    for day in 1..=25 {
        let day_path = format!("advent-puzzles/src/{}/day{}", year, day);
        if Path::new(&day_path).exists() {
            let expected_path = format!("advent-puzzles/src/{}/day{}/expected.toml", year, day);
            if !Path::new(&expected_path).exists() {
                continue;
            }
            if days_with_tests > 0 {
                println!();
            }
            println!("{}", format!("Day {}", day).cyan().bold());
            let (passed, failed) = run_day_tests(year, day, false);
            total_passed += passed;
            total_failed += failed;
            days_with_tests += 1;
        }
    }

    if days_with_tests == 0 {
        println!("\n{}", "No days with expected.toml found".yellow());
    }

    println!("\n{}", "═".repeat(40).dimmed());
    print_test_summary(total_passed, total_failed);
}

fn run_day_tests(year: u16, day: u8, verbose: bool) -> (usize, usize) {
    let expected_path = format!("advent-puzzles/src/{}/day{}/expected.toml", year, day);
    let expected: ExpectedAnswers = if Path::new(&expected_path).exists() {
        let content = fs::read_to_string(&expected_path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        if verbose {
            println!("{}", "  No expected.toml found, skipping".yellow());
        }
        return (0, 0);
    };

    let mut passed = 0;
    let mut failed = 0;

    if !expected.example.part1.is_empty() {
        let input = find_input(year, day, true, false);
        if input.is_empty() {
            println!("  {} {}", "Part 1:".bold(), "No example input".yellow());
        } else {
            clear_all_caches_and_stats();
            let result = advent_puzzles::run_day(year, day, false, input);
            if result == expected.example.part1 {
                println!(
                    "  {} {} {}",
                    "Part 1:".bold(),
                    "PASS".green().bold(),
                    format!("({})", result).dimmed()
                );
                passed += 1;
            } else {
                println!(
                    "  {} {} expected {}, got {}",
                    "Part 1:".bold(),
                    "FAIL".red().bold(),
                    expected.example.part1.yellow(),
                    result.red()
                );
                failed += 1;
            }
            print_memoize_stats();
        }
    } else if verbose {
        println!("  {} {}", "Part 1:".bold(), "No expected value".dimmed());
    }

    if !expected.example.part2.is_empty() {
        let input = find_input(year, day, true, true);
        if input.is_empty() {
            println!("  {} {}", "Part 2:".bold(), "No example input".yellow());
        } else {
            clear_all_caches_and_stats();
            let result = advent_puzzles::run_day(year, day, true, input);
            if result == expected.example.part2 {
                println!(
                    "  {} {} {}",
                    "Part 2:".bold(),
                    "PASS".green().bold(),
                    format!("({})", result).dimmed()
                );
                passed += 1;
            } else {
                println!(
                    "  {} {} expected {}, got {}",
                    "Part 2:".bold(),
                    "FAIL".red().bold(),
                    expected.example.part2.yellow(),
                    result.red()
                );
                failed += 1;
            }
            print_memoize_stats();
        }
    } else if verbose {
        println!("  {} {}", "Part 2:".bold(), "No expected value".dimmed());
    }

    if let Some(real) = expected.real {
        if verbose {
            println!("\n  {}", "Real input:".dimmed());
        }

        if !real.part1.is_empty() {
            clear_all_caches_and_stats();
            let input = find_input(year, day, false, false);
            let result = advent_puzzles::run_day(year, day, false, input);
            if result == real.part1 {
                println!(
                    "  {} {} {}",
                    "Part 1 (real):".bold(),
                    "PASS".green().bold(),
                    format!("({})", result).dimmed()
                );
                passed += 1;
            } else {
                println!(
                    "  {} {} expected {}, got {}",
                    "Part 1 (real):".bold(),
                    "FAIL".red().bold(),
                    real.part1.yellow(),
                    result.red()
                );
                failed += 1;
            }
            print_memoize_stats();
        }

        if !real.part2.is_empty() {
            clear_all_caches_and_stats();
            let input = find_input(year, day, false, true);
            let result = advent_puzzles::run_day(year, day, true, input);
            if result == real.part2 {
                println!(
                    "  {} {} {}",
                    "Part 2 (real):".bold(),
                    "PASS".green().bold(),
                    format!("({})", result).dimmed()
                );
                passed += 1;
            } else {
                println!(
                    "  {} {} expected {}, got {}",
                    "Part 2 (real):".bold(),
                    "FAIL".red().bold(),
                    real.part2.yellow(),
                    result.red()
                );
                failed += 1;
            }
            print_memoize_stats();
        }
    }

    (passed, failed)
}

fn print_test_summary(passed: usize, failed: usize) {
    let total = passed + failed;
    if failed == 0 {
        println!(
            "{} {}",
            "✓".green().bold(),
            format!("All {} tests passed!", total).green().bold()
        );
    } else {
        println!(
            "{} {} passed, {} failed",
            "✗".red().bold(),
            passed.to_string().green(),
            failed.to_string().red().bold()
        );
    }
}

fn handle_new(year: u16, day: u8) {
    if !(1..=25).contains(&day) {
        eprintln!("{}", "Day must be between 1 and 25".red());
        std::process::exit(1);
    }

    let day_path = format!("advent-puzzles/src/{}/day{}", year, day);

    if Path::new(&day_path).exists() {
        eprintln!(
            "{}",
            format!("Day {} for year {} already exists", day, year).red()
        );
        std::process::exit(1);
    }

    let year_path = format!("advent-puzzles/src/{}", year);
    if !Path::new(&year_path).exists() {
        fs::create_dir_all(&year_path).expect("Failed to create year directory");
        println!(
            "{} {}",
            "✓".green(),
            format!("Created year directory: {}", year_path)
        );
    }

    fs::create_dir_all(&day_path).expect("Failed to create day directory");

    let template_dir = Path::new("advent-puzzles/example_day");

    let mod_content =
        fs::read_to_string(template_dir.join("mod.rs")).expect("Failed to read template mod.rs");
    fs::write(format!("{}/mod.rs", day_path), mod_content).expect("Failed to write mod.rs");

    let input_content = fs::read_to_string(template_dir.join("input.txt")).unwrap_or_default();
    fs::write(format!("{}/input.txt", day_path), input_content).expect("Failed to write input.txt");

    let example_content = fs::read_to_string(template_dir.join("example.txt")).unwrap_or_default();
    fs::write(format!("{}/example.txt", day_path), example_content)
        .expect("Failed to write example.txt");

    let expected_content =
        fs::read_to_string(template_dir.join("expected.toml")).unwrap_or_default();
    fs::write(format!("{}/expected.toml", day_path), expected_content)
        .expect("Failed to write expected.toml");

    println!(
        "{} {}",
        "✓".green().bold(),
        format!("Created new day {} for year {} at {}", day, year, day_path).green()
    );
    println!(
        "{}",
        "  Don't forget to rebuild for the macros to pick up the new day!".dimmed()
    );
}

fn find_input(year: u16, day: u8, example: bool, part2: bool) -> String {
    if example {
        if part2 {
            let example2_path = format!("advent-puzzles/src/{}/day{}/example_2.txt", year, day);
            if Path::new(&example2_path).exists() {
                return fs::read_to_string(&example2_path).unwrap_or_default();
            }
        }
        let example_path = format!("advent-puzzles/src/{}/day{}/example.txt", year, day);
        fs::read_to_string(&example_path).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                format!("Warning: example.txt not found for day {}", day).yellow()
            );
            String::new()
        })
    } else {
        let input_path = format!("advent-puzzles/src/{}/day{}/input.txt", year, day);
        fs::read_to_string(&input_path).unwrap_or_else(|_| {
            eprintln!(
                "{}",
                format!("Warning: input.txt not found for day {}", day).yellow()
            );
            String::new()
        })
    }
}

fn todays_day() -> u8 {
    let now = chrono::Utc::now().date_naive();
    if now.month() != 12 {
        panic!("It's not December!");
    }
    now.day() as u8
}
