use advent_derive::*;

pub mod utils;

year_declerations!();

pub fn run_day(year: u16, day: u8, part: bool, input: String) -> String {
    day_invocations!(year, day, part, input)
}
