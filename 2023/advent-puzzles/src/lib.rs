pub use advent_derive::memoize;

use advent_derive::*;

pub mod utils;

pub use utils::memoize::{clear_all_caches_and_stats, print_memoize_stats};

year_declerations!();

pub fn run_day(year: u16, day: u8, part: bool, input: String) -> String {
    day_invocations!(year, day, part, input)
}
