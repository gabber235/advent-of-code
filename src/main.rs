use advent_derive::*;
use chrono::Datelike;
use clap::Parser;

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
