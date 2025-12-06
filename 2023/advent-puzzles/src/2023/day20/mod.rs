use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    IResult, Parser,
};

pub fn part1(input: String) -> String {
    let blueprints = parse_input(&input).unwrap().1;
    let mut modules = construct_modules(&blueprints);

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        press_button(&mut modules, |trigger| match trigger.pulse {
            Pulse::High => {
                high_count += 1;
            }
            Pulse::Low => {
                low_count += 1;
            }
        });
    }

    format!(
        "low: {} high: {}, combi: {}",
        low_count,
        high_count,
        low_count * high_count
    )
}

pub fn part2(input: String) -> String {
    let blueprints = parse_input(&input).unwrap().1;
    let mut modules = construct_modules(&blueprints);

    let parents = find_parents(&blueprints, "rx");
    if parents.len() != 1 {
        panic!("rx parents: {:?}", parents);
    }

    let rq_parent = parents[0].clone();
    let grandparents = find_parents(&blueprints, &rq_parent);

    let mut button_presses = 0;

    let mut grandparents_high: HashMap<String, u64> = HashMap::new();

    while grandparents_high.len() < grandparents.len() {
        button_presses += 1;
        press_button(&mut modules, |trigger| {
            if grandparents.contains(&trigger.from) && trigger.pulse == Pulse::High {
                if !grandparents_high.contains_key(&trigger.from) {
                    grandparents_high.insert(trigger.from.clone(), button_presses);
                }
            }
        });
    }

    // Find the lowest common multiple of the grandparents_high values

    let lcm = grandparents_high
        .values()
        .copied()
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap();

    format!("button presses: {}", lcm)
}

fn find_parents(blueprints: &[ModuleBlueprint], name: &str) -> Vec<String> {
    blueprints
        .iter()
        .filter(|b| b.outputs.contains(&name.to_string()))
        .map(|b| b.name.clone())
        .collect()
}

fn press_button(
    modules: &mut HashMap<String, Box<dyn Module>>,
    mut before_trigger: impl FnMut(&Trigger),
) {
    let mut triggers = VecDeque::from([Trigger {
        from: "button".to_string(),
        to: "broadcaster".to_string(),
        pulse: Pulse::Low,
    }]);

    while let Some(trigger) = triggers.pop_front() {
        before_trigger(&trigger);

        let Some(module) = modules.get_mut(&trigger.to) else {
            continue;
        };
        let output = module.process(trigger.from, trigger.pulse);
        if let Some(output) = output {
            for target in module.outputs() {
                triggers.push_back(Trigger {
                    from: module.name().to_string(),
                    to: target.to_string(),
                    pulse: output,
                });
            }
        }
    }
}

#[derive(Debug)]
struct Trigger {
    from: String,
    to: String,
    pulse: Pulse,
}

impl Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}> {} ", self.from, self.pulse, self.to)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::High => write!(f, "high"),
            Pulse::Low => write!(f, "low"),
        }
    }
}

fn construct_modules(blueprints: &[ModuleBlueprint]) -> HashMap<String, Box<dyn Module>> {
    blueprints
        .iter()
        .map(|blueprint| {
            let module: Box<dyn Module> = match blueprint.type_ {
                ModuleType::Broadcast => Box::new(Broadcast::new(blueprint)),
                ModuleType::FlipFlop => Box::new(FlipFlop::new(blueprint)),
                ModuleType::Conjunction => Box::new(Conjunction::new(blueprint, blueprints)),
            };
            (blueprint.name.clone(), module)
        })
        .collect()
}

trait Module {
    fn process(&mut self, from: String, input: Pulse) -> Option<Pulse>;
    fn name(&self) -> &str;
    fn outputs(&self) -> &[String];
}

struct Broadcast {
    name: String,
    outputs: Vec<String>,
}

impl Broadcast {
    fn new(module_blueprint: &ModuleBlueprint) -> Self {
        Self {
            name: module_blueprint.name.clone(),
            outputs: module_blueprint.outputs.clone(),
        }
    }
}

impl Module for Broadcast {
    fn process(&mut self, _from: String, input: Pulse) -> Option<Pulse> {
        match input {
            Pulse::High => Some(Pulse::High),
            Pulse::Low => Some(Pulse::Low),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn outputs(&self) -> &[String] {
        &self.outputs
    }
}

struct FlipFlop {
    name: String,
    outputs: Vec<String>,
    state: bool,
}

impl FlipFlop {
    fn new(module_blueprint: &ModuleBlueprint) -> Self {
        Self {
            name: module_blueprint.name.clone(),
            outputs: module_blueprint.outputs.clone(),
            state: false,
        }
    }
}

impl Module for FlipFlop {
    fn process(&mut self, _from: String, input: Pulse) -> Option<Pulse> {
        match input {
            Pulse::High => None,
            Pulse::Low => {
                self.state = !self.state;
                let pulse = if self.state { Pulse::High } else { Pulse::Low };
                Some(pulse)
            }
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn outputs(&self) -> &[String] {
        &self.outputs
    }
}

struct Conjunction {
    name: String,
    outputs: Vec<String>,
    history: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new(module_blueprint: &ModuleBlueprint, blueprints: &[ModuleBlueprint]) -> Self {
        let history = blueprints
            .iter()
            .filter(|b| b.outputs.contains(&module_blueprint.name))
            .map(|b| (b.name.clone(), Pulse::Low))
            .collect();
        Self {
            name: module_blueprint.name.clone(),
            outputs: module_blueprint.outputs.clone(),
            history,
        }
    }
}

impl Module for Conjunction {
    fn process(&mut self, from: String, input: Pulse) -> Option<Pulse> {
        self.history.insert(from, input);
        if self.history.values().all(|p| p == &Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn outputs(&self) -> &[String] {
        &self.outputs
    }
}

#[derive(Debug)]
struct ModuleBlueprint {
    name: String,
    type_: ModuleType,
    outputs: Vec<String>,
}

#[derive(Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

fn parse_input(input: &str) -> IResult<&str, Vec<ModuleBlueprint>> {
    separated_list1(newline, parse_module)(input)
}

fn parse_module(input: &str) -> IResult<&str, ModuleBlueprint> {
    let (input, type_) = parse_module_type(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((
        input,
        ModuleBlueprint {
            name: name.to_string(),
            type_,
            outputs: outputs.iter().map(|s| s.to_string()).collect(),
        },
    ))
}

fn parse_module_type(input: &str) -> IResult<&str, ModuleType> {
    alt((
        tag("%").map(|_| ModuleType::FlipFlop),
        tag("&").map(|_| ModuleType::Conjunction),
        tag("").map(|_| ModuleType::Broadcast),
    ))(input)
}
