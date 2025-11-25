pub fn part1(input: String) -> String {
    let commands = parse_input(&input).unwrap();
    let mut position = Position::default();

    for command in commands {
        position.update_part1(&command);
    }

    println!("Position: {:?}", position);

    format!("{}", position.x * position.depth)
}

pub fn part2(input: String) -> String {
    let commands = parse_input(&input).unwrap();
    let mut position = Position::default();

    for command in commands {
        position.update_part2(&command);
    }

    format!("{}", position.x * position.depth)
}

use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
enum CommandParseError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(CommandParseError::InvalidCommand(s.to_string()));
        }

        let value = parts[1]
            .parse::<i32>()
            .map_err(|_| CommandParseError::InvalidValue(parts[1].to_string()))?;

        match parts[0] {
            "forward" => Ok(Command::Forward(value)),
            "up" => Ok(Command::Up(value)),
            "down" => Ok(Command::Down(value)),
            _ => Err(CommandParseError::InvalidCommand(parts[0].to_string())),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Command>, CommandParseError> {
    input.lines().map(str::parse).collect()
}

#[derive(Debug)]
struct Position {
    x: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn update_part1(&mut self, command: &Command) {
        match command {
            Command::Forward(x) => self.x += x,
            Command::Up(x) => self.depth -= x,
            Command::Down(x) => self.depth += x,
        }
    }
    fn update_part2(&mut self, command: &Command) {
        match command {
            Command::Forward(x) => {
                self.x += x;
                self.depth += self.aim * x;
            }
            Command::Up(x) => self.aim -= x,
            Command::Down(x) => self.aim += x,
        }
    }
}
impl Default for Position {
    fn default() -> Self {
        Position {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }
}
