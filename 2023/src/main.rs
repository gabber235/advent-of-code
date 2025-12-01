use advent_derive::*;
use chrono::Datelike;
use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Debug, Parser)]
#[clap(name = "Advent of code", version = "1.0", author = "gabber235")]
enum Cli {
    Run {
        #[clap(value_enum)]
        year: Years,
        #[clap(value_enum, short, long, default_value = "today")]
        day: Days,
        #[clap(value_enum, short, long, default_value = "1")]
        part: Part,
        #[clap(short, long)]
        test: bool,
    },
    Bench {
        #[clap(value_enum)]
        year: Years,
        #[clap(value_enum, short, long, default_value = "today")]
        day: Days,
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
}

impl From<Part> for bool {
    fn from(part: Part) -> Self {
        match part {
            Part::Part1 => false,
            Part::Part2 => true,
        }
    }
}

years_enum!();

days_enum!();

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Run {
            year,
            day,
            part,
            test,
        } => handle_run(year, day, part, test),
        Cli::Bench { year, day } => handle_bench(year, day),
        Cli::New { year, day } => handle_new(year, day),
    }
}

fn handle_run(year: Years, day: Days, part: Part, test: bool) {
    let year: u16 = year.into();
    let day: u8 = day.into();
    let part: bool = part.into();
    let input = find_input(year, day, test);
    let result = advent_puzzles::run_day(year, day, part, input);
    println!("{}", result);
}

fn handle_bench(year: Years, day: Days) {
    let year: u16 = year.into();
    let day: u8 = day.into();
    let input = find_input(year, day, false);

    let mut criterion = criterion::Criterion::default()
        .without_plots()
        .warm_up_time(std::time::Duration::from_millis(2000))
        .measurement_time(std::time::Duration::from_millis(20000))
        .with_output_color(true);

    let mut group = criterion.benchmark_group(format!("{} day {}", year, day));

    group.bench_function("part 1", |b| {
        b.iter(|| advent_puzzles::run_day(year, day, Part::Part1.into(), input.clone()))
    });

    group.bench_function("part 2", |b| {
        b.iter(|| advent_puzzles::run_day(year, day, Part::Part2.into(), input.clone()))
    });

    group.finish();

    criterion.final_summary();
}

fn handle_new(year: u16, day: u8) {
    if !(1..=25).contains(&day) {
        eprintln!("Day must be between 1 and 25");
        std::process::exit(1);
    }

    let day_path = format!("advent-puzzles/src/{}/day{}", year, day);

    if Path::new(&day_path).exists() {
        eprintln!("Day {} for year {} already exists", day, year);
        std::process::exit(1);
    }

    let year_path = format!("advent-puzzles/src/{}", year);
    if !Path::new(&year_path).exists() {
        fs::create_dir_all(&year_path).expect("Failed to create year directory");
        println!("Created year directory: {}", year_path);
    }

    fs::create_dir_all(&day_path).expect("Failed to create day directory");

    let template_dir = Path::new("advent-puzzles/example_day");

    let mod_content =
        fs::read_to_string(template_dir.join("mod.rs")).expect("Failed to read template mod.rs");
    fs::write(format!("{}/mod.rs", day_path), mod_content).expect("Failed to write mod.rs");

    let input_content = fs::read_to_string(template_dir.join("input.txt")).unwrap_or_default();
    fs::write(format!("{}/input.txt", day_path), input_content).expect("Failed to write input.txt");

    let test_input_content =
        fs::read_to_string(template_dir.join("test_input.txt")).unwrap_or_default();
    fs::write(format!("{}/test_input.txt", day_path), test_input_content)
        .expect("Failed to write test_input.txt");

    println!("Created new day {} for year {} at {}", day, year, day_path);
    println!("Don't forget to rebuild the project for the macros to pick up the new day!");
}

fn find_input(year: u16, day: u8, test: bool) -> String {
    let input_path = format!(
        "advent-puzzles/src/{}/day{}/{}input.txt",
        year,
        day,
        if test { "test_" } else { "" }
    );
    std::fs::read_to_string(input_path).unwrap_or("".to_string())
}

fn todays_day() -> u8 {
    let now = chrono::Utc::now().date_naive();
    if now.month() != 12 {
        panic!("It's not december");
    }
    let day = now.day();
    day as u8
}
