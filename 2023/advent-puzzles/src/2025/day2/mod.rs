use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list1,
    sequence::separated_pair, IResult,
};

fn parse_id_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (first, last)) = separated_pair(u64, tag("-"), u64)(input)?;
    Ok((input, first..=last))
}

fn parse_id_ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(tag(","), parse_id_range)(input)
}

pub fn part1(input: String) -> String {
    let ranges = parse_id_ranges(input.trim()).unwrap().1;
    ranges
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|id| !valid_id_part1(id))
        .sum::<u64>()
        .to_string()
}

fn valid_id_part1(id: &u64) -> bool {
    let string = id.to_string();
    let first_half = &string[..string.len() / 2];
    let second_half = &string[string.len() / 2..];
    first_half != second_half
}

pub fn part2(input: String) -> String {
    let ranges = parse_id_ranges(input.trim()).unwrap().1;
    ranges
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|id| !valid_id_part2(id))
        .sum::<u64>()
        .to_string()
}

pub fn valid_id_part2(id: &u64) -> bool {
    let string = id.to_string();
    let length = string.len();

    for pattern_length in 1..length {
        if length % pattern_length != 0 {
            continue;
        }
        let pattern = &string[0..pattern_length];

        let mut matched = true;
        for i in (pattern_length..length).step_by(pattern_length) {
            if &string[i..i + pattern_length] != pattern {
                matched = false;
                break;
            }
        }
        if matched {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn invalid_ids_in_range(range: RangeInclusive<u64>) -> Vec<u64> {
        range.into_iter().filter(|id| !valid_id_part1(id)).collect()
    }

    #[test]
    fn range_11_22_has_two_invalid() {
        assert_eq!(invalid_ids_in_range(11..=22), vec![11, 22]);
    }

    #[test]
    fn range_95_115_has_one_invalid() {
        assert_eq!(invalid_ids_in_range(95..=115), vec![99]);
    }

    #[test]
    fn range_998_1012_has_one_invalid() {
        assert_eq!(invalid_ids_in_range(998..=1012), vec![1010]);
    }

    #[test]
    fn range_1188511880_1188511890_has_one_invalid() {
        assert_eq!(
            invalid_ids_in_range(1188511880..=1188511890),
            vec![1188511885]
        );
    }

    #[test]
    fn range_222220_222224_has_one_invalid() {
        assert_eq!(invalid_ids_in_range(222220..=222224), vec![222222]);
    }

    #[test]
    fn range_1698522_1698528_has_no_invalid() {
        assert_eq!(invalid_ids_in_range(1698522..=1698528), vec![]);
    }

    #[test]
    fn range_446443_446449_has_one_invalid() {
        assert_eq!(invalid_ids_in_range(446443..=446449), vec![446446]);
    }

    #[test]
    fn range_38593856_38593862_has_one_invalid() {
        assert_eq!(invalid_ids_in_range(38593856..=38593862), vec![38593859]);
    }

    #[test]
    fn range_565653_565659_has_no_invalid() {
        assert_eq!(invalid_ids_in_range(565653..=565659), vec![]);
    }

    #[test]
    fn range_824824821_824824827_has_no_invalid() {
        assert_eq!(invalid_ids_in_range(824824821..=824824827), vec![]);
    }

    #[test]
    fn range_2121212118_2121212124_has_no_invalid() {
        assert_eq!(invalid_ids_in_range(2121212118..=2121212124), vec![]);
    }
}
