use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::value;
use nom::{Err, IResult};

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| combine_first_last_digit(line).unwrap().1)
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let Ok(digit) = combine_first_last_number(line) else {
                panic!("Failed to parse line: {}", line);
            };
            digit.1
        })
        .sum::<u32>()
        .to_string()
}

fn find_all<O, F>(input: &str, mut parser: F) -> IResult<&str, Vec<O>>
where
    F: FnMut(&str) -> IResult<&str, O>,
{
    let mut res = Vec::new();

    for i in 0..input.len() {
        let split = input.split_at(i).1;
        let result = parser(split);
        if let Ok((_, o)) = result {
            res.push(o);
        }
    }

    Ok(("", res))
}

fn combine_first_last_digit(input: &str) -> IResult<&str, u32> {
    let (input, digits) = find_all(input, digit)?;

    if digits.len() < 1 {
        return Err(Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }

    let first = digits[0];
    let last = digits[digits.len() - 1];
    let result = first * 10 + last;

    Ok((input, result))
}

fn combine_first_last_number(input: &str) -> IResult<&str, u32> {
    let (input, digits) = find_all(input, number)?;

    if digits.len() < 1 {
        return Err(Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }

    let first = digits[0];
    let last = digits[digits.len() - 1];
    let result = first * 10 + last;

    Ok((input, result))
}

fn number(input: &str) -> IResult<&str, u32> {
    alt((
        digit,
        value(0, tag("zero")),
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

fn digit(input: &str) -> IResult<&str, u32> {
    let (input, digit) = take(1usize)(input)?;

    let digit = digit.parse::<u32>().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
    })?;

    Ok((input, digit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_first_last_digit() {
        assert_eq!(combine_first_last_digit("12"), Ok(("", 12)));
        assert_eq!(combine_first_last_digit("123"), Ok(("", 13)));
        assert_eq!(combine_first_last_digit("34a"), Ok(("", 34)));
        assert_eq!(combine_first_last_digit("ax3cCsv5adcl4adl"), Ok(("", 34)));
        assert_eq!(combine_first_last_digit("1abc2"), Ok(("", 12)));
        assert_eq!(combine_first_last_digit("pqr3stu8vwx"), Ok(("", 38)));
        assert_eq!(combine_first_last_digit("a1b2c3d4e5f"), Ok(("", 15)));
        assert_eq!(combine_first_last_digit("treb7uchet"), Ok(("", 77)));
    }

    #[test]
    fn test_combine_first_last_number() {
        assert_eq!(combine_first_last_number("two1nine"), Ok(("", 29)));
        assert_eq!(combine_first_last_number("eightwothree"), Ok(("", 83)));
        assert_eq!(combine_first_last_number("abcone2threexyz"), Ok(("", 13)));
        assert_eq!(combine_first_last_number("xtwone3four"), Ok(("", 24)));
        assert_eq!(combine_first_last_number("4nineeightseven2"), Ok(("", 42)));
        assert_eq!(combine_first_last_number("zoneight234"), Ok(("", 14)));
        assert_eq!(combine_first_last_number("7pqrstsixteen"), Ok(("", 76)));
    }
}
