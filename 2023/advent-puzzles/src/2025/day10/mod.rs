use good_lp::solvers::microlp::microlp;
use good_lp::{constraint, variable, Expression, ProblemVariables, Solution, SolverModel};
use nom::{
    branch::alt,
    character::complete::{char, digit1, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
use priority_queue::PriorityQueue;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Machine {
    pub indicator: Indicator,
    pub buttons: Vec<Button>,
    pub joltage: JoltageRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Indicator {
    pub pattern: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub wiring: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JoltageRequirement {
    pub values: Vec<u32>,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_indicator(input: &str) -> IResult<&str, Indicator> {
    let (input, pattern_str) = delimited(
        char('['),
        nom::bytes::complete::take_while1(|c| c == '.' || c == '#'),
        char(']'),
    )(input)?;

    let pattern = pattern_str.chars().map(|c| c == '#').collect();
    Ok((input, Indicator { pattern }))
}

fn parse_button(input: &str) -> IResult<&str, Button> {
    let (input, wiring) = delimited(
        char('('),
        separated_list1(char(','), parse_number),
        char(')'),
    )(input)?;

    Ok((input, Button { wiring }))
}

fn parse_single_button(input: &str) -> IResult<&str, Button> {
    let (input, wiring) = delimited(char('('), map(parse_number, |n| vec![n]), char(')'))(input)?;

    Ok((input, Button { wiring }))
}

fn parse_any_button(input: &str) -> IResult<&str, Button> {
    alt((parse_button, parse_single_button))(input)
}

fn parse_joltage(input: &str) -> IResult<&str, JoltageRequirement> {
    let (input, values) = delimited(
        char('{'),
        separated_list1(char(','), parse_number),
        char('}'),
    )(input)?;

    Ok((input, JoltageRequirement { values }))
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, indicator) = parse_indicator(input)?;
    let (input, _) = space1(input)?;

    let (input, buttons) = separated_list1(space1, parse_any_button)(input)?;

    let (input, _) = space1(input)?;
    let (input, joltage) = parse_joltage(input)?;

    Ok((
        input,
        Machine {
            indicator,
            buttons,
            joltage,
        },
    ))
}

pub fn parse_machines(input: &str) -> Result<Vec<Machine>, String> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            parse_machine(line.trim())
                .map(|(_, machine)| machine)
                .map_err(|e| format!("Failed to parse line '{}': {:?}", line, e))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IndicatorState {
    indicator: Indicator,
    presses: u32,
}

impl IndicatorState {
    fn new(machine: &Machine) -> Self {
        IndicatorState {
            indicator: Indicator {
                pattern: vec![false; machine.indicator.pattern.len()],
            },
            presses: 0,
        }
    }

    fn press(&self, button: &Button) -> IndicatorState {
        let mut state = self.clone();
        state.presses += 1;

        for &index in &button.wiring {
            if let Some(value) = state.indicator.pattern.get_mut(index as usize) {
                *value = !*value;
            }
        }

        state
    }
}

pub fn solve_lights(machine: &Machine) -> u32 {
    let mut states = PriorityQueue::new();
    states.push(IndicatorState::new(machine), 0i32);
    let mut seen: HashSet<Indicator> = HashSet::new();

    while !states.is_empty() {
        let (current, _) = states.pop().unwrap();

        for button in &machine.buttons {
            let next = current.press(button);

            if next.indicator == machine.indicator {
                return next.presses;
            }

            if seen.contains(&next.indicator) {
                continue;
            }

            seen.insert(next.indicator.clone());
            let presses = next.presses.clone();
            states.push(next, -(presses as i32));
        }
    }

    panic!("Could not solve machine {:?}", machine)
}

pub fn part1(input: String) -> String {
    let machines = parse_machines(&input).unwrap();
    machines
        .par_iter()
        .map(solve_lights)
        .sum::<u32>()
        .to_string()
}

pub fn solve_joltage(machine: &Machine) -> u32 {
    let mut vars = ProblemVariables::new();

    let presses: Vec<_> = machine
        .buttons
        .iter()
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    let total_presses: Expression = presses.iter().sum();
    let mut problem = vars.minimise(total_presses).using(microlp);

    for (counter_idx, &target) in machine.joltage.values.iter().enumerate() {
        let counter_sum: Expression = machine
            .buttons
            .iter()
            .enumerate()
            .filter(|(_, btn)| btn.wiring.contains(&(counter_idx as u32)))
            .map(|(i, _)| presses[i])
            .sum();
        problem = problem.with(constraint!(counter_sum == target as i32));
    }

    let solution = problem.solve().expect("No solution found");
    presses
        .iter()
        .map(|&p| solution.value(p).round() as u32)
        .sum()
}

pub fn part2(input: String) -> String {
    let machines = parse_machines(&input).unwrap();
    machines
        .par_iter()
        .map(solve_joltage)
        .sum::<u32>()
        .to_string()
}
