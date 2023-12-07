use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::u64;

use itertools::Itertools;
use nom::{
    character::complete::{one_of, space1, u64},
    multi::count,
    IResult,
};

pub fn part1(input: String) -> String {
    part1::part1(input)
}

pub(crate) mod part1 {
    use super::*;

    pub fn part1(input: String) -> String {
        let hands = input
            .lines()
            .map(|line| parse_hand(line).unwrap().1)
            .sorted()
            .enumerate()
            .map(|(i, hand)| ((i + 1) as u64) * hand.bid)
            .collect::<Vec<_>>();

        format!("{}", hands.iter().sum::<u64>())
    }

    #[derive(Debug, Eq, PartialEq, Clone, Copy, Ord)]
    enum Card {
        Joker,
        Number(u8),
        Ten,
        Queen,
        King,
        Ace,
    }

    impl FromStr for Card {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "T" => Ok(Card::Ten),
                "J" => Ok(Card::Joker),
                "Q" => Ok(Card::Queen),
                "K" => Ok(Card::King),
                "A" => Ok(Card::Ace),
                _ => Ok(Card::Number(s.parse().map_err(|_| ())?)),
            }
        }
    }
    impl Display for Card {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Card::Number(n) => write!(f, "{}", n),
                Card::Ten => write!(f, "T"),
                Card::Joker => write!(f, "J"),
                Card::Queen => write!(f, "Q"),
                Card::King => write!(f, "K"),
                Card::Ace => write!(f, "A"),
            }
        }
    }
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPairs,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(Debug, PartialEq, Eq, Ord)]
    struct Hand {
        cards: [Card; 5],
        bid: u64,
        hand_type: HandType,
    }

    impl Display for Hand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for card in self.cards.iter() {
                write!(f, "{}", card)?;
            }
            write!(f, " {} ({:?})", self.bid, self.hand_type)
        }
    }

    fn parse_hand(s: &str) -> IResult<&str, Hand> {
        let (s, cards) = count(parse_card, 5)(s)?;
        let cards = vec_cards_to_array(cards);
        let hand_type = HandType::from_cards(&cards);
        let (s, _) = space1(s)?;
        let (s, bid) = u64(s)?;
        Ok((
            s,
            Hand {
                cards,
                hand_type,
                bid,
            },
        ))
    }

    fn parse_card(s: &str) -> IResult<&str, Card> {
        let (s, card) = one_of("23456789TJQKA")(s)?;
        Ok((s, card.to_string().parse().unwrap()))
    }

    fn vec_cards_to_array(cards: Vec<Card>) -> [Card; 5] {
        let mut array = [Card::Number(0); 5];
        for (i, card) in cards.into_iter().enumerate() {
            array[i] = card;
        }
        array
    }

    impl Card {
        fn value(&self) -> u8 {
            match self {
                Card::Number(n) => *n,
                Card::Joker => 11,
                Card::Ten => 10,
                Card::Queen => 12,
                Card::King => 13,
                Card::Ace => 14,
            }
        }
    }

    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.value().partial_cmp(&other.value())
        }
    }

    impl HandType {
        fn from_cards(cards: &[Card; 5]) -> HandType {
            let mut counts = [0; 15];
            for card in cards {
                counts[card.value() as usize] += 1;
            }

            let mut groups_amount = [0; 6];
            for count in counts.iter() {
                groups_amount[*count as usize] += 1;
            }

            if groups_amount[5] == 1 {
                HandType::FiveOfAKind
            } else if groups_amount[4] == 1 {
                HandType::FourOfAKind
            } else if groups_amount[3] == 1 && groups_amount[2] == 1 {
                HandType::FullHouse
            } else if groups_amount[3] == 1 {
                HandType::ThreeOfAKind
            } else if groups_amount[2] == 2 {
                HandType::TwoPairs
            } else if groups_amount[2] == 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            // If the hand are different types, compare by hand type
            if self.hand_type != other.hand_type {
                return self.hand_type.partial_cmp(&other.hand_type);
            }

            // If the hand are the same type, compare by cards from first to last
            // until one is greater than the other
            for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                match card1.partial_cmp(card2) {
                    Some(std::cmp::Ordering::Equal) => continue,
                    Some(ordering) => return Some(ordering),
                    None => return None,
                }
            }

            // If all cards are equal, compare by bid
            self.bid.partial_cmp(&other.bid)
        }
    }
}

pub fn part2(input: String) -> String {
    part2::part2(input)
}

mod part2 {
    use std::collections::HashMap;

    use super::*;

    pub fn part2(input: String) -> String {
        let hands = input
            .lines()
            .map(|line| parse_hand(line).unwrap().1)
            .sorted()
            .enumerate()
            .map(|(i, hand)| ((i + 1) as u64) * hand.bid)
            .collect::<Vec<_>>();

        format!("{}", hands.iter().sum::<u64>())
    }

    #[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, Hash)]
    enum Card {
        Joker,
        Number(u8),
        Ten,
        Queen,
        King,
        Ace,
    }

    impl FromStr for Card {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "T" => Ok(Card::Ten),
                "J" => Ok(Card::Joker),
                "Q" => Ok(Card::Queen),
                "K" => Ok(Card::King),
                "A" => Ok(Card::Ace),
                _ => Ok(Card::Number(s.parse().map_err(|_| ())?)),
            }
        }
    }
    impl Display for Card {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Card::Number(n) => write!(f, "{}", n),
                Card::Ten => write!(f, "T"),
                Card::Joker => write!(f, "J"),
                Card::Queen => write!(f, "Q"),
                Card::King => write!(f, "K"),
                Card::Ace => write!(f, "A"),
            }
        }
    }
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPairs,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(Debug, PartialEq, Eq, Ord)]
    struct Hand {
        cards: [Card; 5],
        bid: u64,
        hand_type: HandType,
    }

    impl Display for Hand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for card in self.cards.iter() {
                write!(f, "{}", card)?;
            }
            write!(f, " {} ({:?})", self.bid, self.hand_type)
        }
    }

    fn parse_hand(s: &str) -> IResult<&str, Hand> {
        let (s, cards) = count(parse_card, 5)(s)?;
        let cards = vec_cards_to_array(cards);
        let hand_type = HandType::from_cards(&cards);
        let (s, _) = space1(s)?;
        let (s, bid) = u64(s)?;
        Ok((
            s,
            Hand {
                cards,
                hand_type,
                bid,
            },
        ))
    }

    fn parse_card(s: &str) -> IResult<&str, Card> {
        let (s, card) = one_of("23456789TJQKA")(s)?;
        Ok((s, card.to_string().parse().unwrap()))
    }

    fn vec_cards_to_array(cards: Vec<Card>) -> [Card; 5] {
        let mut array = [Card::Number(0); 5];
        for (i, card) in cards.into_iter().enumerate() {
            array[i] = card;
        }
        array
    }

    impl Card {
        fn value(&self) -> u8 {
            match self {
                Card::Joker => 1,
                Card::Number(n) => *n,
                Card::Ten => 10,
                Card::Queen => 12,
                Card::King => 13,
                Card::Ace => 14,
            }
        }
    }

    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.value().partial_cmp(&other.value())
        }
    }

    impl HandType {
        fn from_cards(cards: &[Card; 5]) -> HandType {
            let counts = cards.iter().counts_by(|card| *card);

            let jokers = counts.get(&Card::Joker).copied().unwrap_or(0);
            let mut counts_without_jokers = counts
                .iter()
                .filter(|(card, _)| **card != Card::Joker)
                .map(|(card, count)| (*card, *count))
                .collect::<HashMap<_, _>>();
            let max_type = counts_without_jokers
                .iter()
                .max_by(|(_, count1), (_, count2)| count1.cmp(count2));

            if let Some((card, count)) = max_type {
                counts_without_jokers.insert(*card, count + jokers);
            } else {
                counts_without_jokers.insert(Card::Joker, jokers);
            }

            find_hand_type(&counts_without_jokers)
        }
    }

    fn find_hand_type(counts: &HashMap<Card, usize>) -> HandType {
        let max_count = counts.values().max().copied().unwrap_or(0);

        match max_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counts.len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if counts.len() == 3 {
                    HandType::TwoPairs
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("Invalid card count {} ({:?})", max_count, counts),
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            // If the hand are different types, compare by hand type
            if self.hand_type != other.hand_type {
                return self.hand_type.partial_cmp(&other.hand_type);
            }

            // If the hand are the same type, compare by cards from first to last
            // until one is greater than the other
            for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                match card1.partial_cmp(card2) {
                    Some(std::cmp::Ordering::Equal) => continue,
                    Some(ordering) => return Some(ordering),
                    None => return None,
                }
            }

            // If all cards are equal, compare by bid
            self.bid.partial_cmp(&other.bid)
        }
    }
}
