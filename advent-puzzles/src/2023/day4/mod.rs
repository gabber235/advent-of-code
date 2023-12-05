use std::u32;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    multi::separated_list1,
    IResult,
};

pub fn part1(input: String) -> String {
    let cards = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect::<Vec<_>>();

    let score = cards.iter().map(|c| c.get_score()).sum::<u32>();

    score.to_string()
}

pub fn part2(input: String) -> String {
    let cards = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect::<Vec<_>>();

    let mut amounts = vec![1; cards.len()];

    for i in 0..cards.len() {
        let winnings = cards[i].matching_numbers();

        let amount = amounts[i];
        for j in 0..winnings {
            amounts[i + j as usize + 1] += amount;
        }
    }

    let score = amounts.iter().sum::<u32>();

    score.to_string()
}

#[derive(Debug)]
struct Card {
    index: usize,
    winning_numbers: Vec<u8>,
    card_numbers: Vec<u8>,
}

impl Card {
    fn new(index: usize, winning_numbers: Vec<u8>, card_numbers: Vec<u8>) -> Self {
        Self {
            index,
            winning_numbers,
            card_numbers,
        }
    }

    fn matching_numbers(&self) -> u32 {
        self.card_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }

    fn get_score(&self) -> u32 {
        let winnings = self.matching_numbers();

        if winnings == 0 {
            return 0;
        }

        2_u32.pow(winnings - 1)
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = parse_card_id(input)?;
    let (input, numbers_list) = separated_list1(tag(" | "), parse_numbers)(input)?;

    if numbers_list.len() != 2 {
        panic!("Invalid card: {:?}, {}", numbers_list, input);
    }

    let winning_numbers = numbers_list[0].clone();
    let card_numbers = numbers_list[1].clone();

    Ok((input, Card::new(id, winning_numbers, card_numbers)))
}

fn parse_card_id(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = take_while_space(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_while_space(input)?;

    Ok((input, id.parse().unwrap()))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, numbers) = separated_list1(take_while_space, digit1)(input.trim())?;

    let numbers = numbers
        .iter()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u8>>();

    Ok((input, numbers))
}

fn take_while_space(input: &str) -> IResult<&str, &str> {
    take_while1(|c| c == ' ')(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card_id() {
        assert_eq!(
            parse_card_id("Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Ok(("41 48 83 86 17 | 83 86  6 31 17  9 48 53", 1))
        );
    }
}
