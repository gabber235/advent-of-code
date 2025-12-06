use crate::utils::parsing::numbers;
use array2d::Array2D;

pub fn part1(input: String) -> String {
    let (grid, operators) = parse_part1(input);
    grid.columns_iter()
        .enumerate()
        .map(|(col, column)| {
            let operator = &operators[col];
            match operator {
                Operator::Add => column.sum::<i64>(),
                Operator::Multiply => column.product::<i64>(),
            }
        })
        .sum::<i64>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (grid, operators) = parse_part2(input);
    grid.iter()
        .enumerate()
        .map(|(col, column)| {
            let operator = &operators[col];
            match operator {
                Operator::Add => column.iter().sum::<i64>(),
                Operator::Multiply => column.iter().product::<i64>(),
            }
        })
        .sum::<i64>()
        .to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
}

fn parse_part1(input: String) -> (Array2D<i64>, Vec<Operator>) {
    let mut lines = input.lines().collect::<Vec<_>>();
    let last = lines.pop().unwrap();
    let operators = parse_operators(last);

    let rows = lines.iter().map(|line| numbers(line)).collect::<Vec<_>>();
    let grid = Array2D::from_rows(&rows).unwrap();
    (grid, operators)
}

fn parse_part2(input: String) -> (Vec<Vec<i64>>, Vec<Operator>) {
    let mut lines = input.lines().collect::<Vec<_>>();
    let last = lines.pop().unwrap();
    let operators = parse_operators(last);

    let length = lines.iter().map(|l| l.len()).max().unwrap();

    let mut char = 0;
    let mut only_whitespace;
    let mut number;
    let mut column = Vec::new();
    let mut columns = Vec::with_capacity(length);

    while char < length {
        number = 0;
        only_whitespace = true;
        for line in &lines {
            if line.is_empty() {
                continue;
            }
            let Some(c) = line.chars().nth(char) else {
                continue;
            };
            if c.is_whitespace() {
                continue;
            }
            number = number * 10 + c.to_digit(10).unwrap() as i64;
            only_whitespace = false;
        }
        if !only_whitespace {
            column.push(number);
        } else {
            columns.push(column.clone());
            column.clear();
        }
        char += 1;
    }

    columns.push(column.clone());

    (columns, operators)
}

fn parse_operators(input: &str) -> Vec<Operator> {
    input
        .chars()
        .filter_map(|c| match c {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Multiply),
            _ => None,
        })
        .collect()
}
