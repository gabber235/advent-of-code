use std::{
    array::from_fn,
    fmt::{Display, Formatter},
    u32,
};

use nom::{bytes::complete::tag, IResult};

pub fn part1(input: String) -> String {
    let input = input.trim();
    input.split(",").map(holiday_hash).sum::<u32>().to_string()
}

pub fn part2(input: String) -> String {
    let input = input.trim();
    let instructions = parse_input(input).unwrap().1;

    let mut boxes: [Box; 256] = from_fn(|id| Box {
        id: id as u8,
        lenses: vec![],
    });

    for instruction in instructions.iter() {
        instruction.apply(&mut boxes);
    }

    boxes
        .iter()
        .filter(|box_| !box_.lenses.is_empty())
        .map(|box_| box_.focus_power())
        .sum::<u32>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    nom::multi::separated_list1(tag(","), parse_instruction)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, label) = nom::character::complete::alpha1(input)?;
    let (input, operation) = nom::branch::alt((
        nom::bytes::complete::tag("="),
        nom::bytes::complete::tag("-"),
    ))(input)?;

    if operation == "=" {
        let (input, focal_length) = nom::character::complete::digit1(input)?;
        let focal_length = focal_length.parse().unwrap();
        Ok((
            input,
            Instruction::Equal(Lens {
                focal_length,
                label: label.to_string(),
            }),
        ))
    } else {
        Ok((
            input,
            Instruction::Remove {
                label: label.to_string(),
            },
        ))
    }
}

#[derive(Debug)]
struct Box {
    id: u8,
    lenses: Vec<Lens>,
}

impl Display for Box {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box {}: ", self.id)?;
        for lens in self.lenses.iter() {
            write!(f, "{} ", lens)?;
        }

        Ok(())
    }
}

impl Box {
    fn add_lens(&mut self, lens: Lens) {
        // If the lens with the same label is already in the box, replace the old one
        if let Some(index) = self.lenses.iter().position(|l| l.label == lens.label) {
            self.lenses[index] = lens;
        } else {
            self.lenses.push(lens);
        }
    }

    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|lens| lens.label != label);
    }

    fn focus_power(&self) -> u32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| (self.id as u32 + 1) * (i as u32 + 1) * (lens.focal_length as u32))
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Lens {
    focal_length: u8,
    label: String,
}

impl Display for Lens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

#[derive(Debug)]
enum Instruction {
    Equal(Lens),
    Remove { label: String },
}

impl Instruction {
    fn apply(&self, boxes: &mut [Box; 256]) {
        match self {
            Instruction::Equal(lens) => {
                let hash = holiday_hash(&lens.label);
                let container = &mut boxes[hash as usize];
                container.add_lens(lens.clone());
            }
            Instruction::Remove { label } => {
                let hash = holiday_hash(label);
                let container = &mut boxes[hash as usize];
                container.remove_lens(label);
            }
        }
    }
}

pub fn holiday_hash(input: &str) -> u32 {
    let mut hash = 0u32;
    for c in input.chars() {
        if !c.is_ascii() {
            panic!("Non-ascii character in input: {}", c);
        }

        hash = (hash + c as u32) * 17 % 256;
    }
    hash
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_holiday_hash(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(holiday_hash(input), expected);
    }
}
