use std::{collections::HashMap, ops::Range, str::FromStr, u64};

use nom::{
    bytes::complete::{tag, take},
    character::complete::{alpha1, digit1, newline},
    multi::separated_list1,
    IResult, Parser,
};

pub fn part1(input: String) -> String {
    let (parts, workflows) = parse_input(&input);

    let workflows: HashMap<String, Workflow> =
        workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    let sections = find_allowed(
        &workflows.get("in").expect("No workflow named 'in'"),
        &workflows,
        vec![],
    );

    parts
        .iter()
        .filter(|p| is_part_accepted(p, &sections))
        .map(|p| p.rating())
        .sum::<u64>()
        .to_string()
}

fn is_part_accepted(part: &Part, sections: &[Section]) -> bool {
    sections.iter().any(|s| s.contains(part))
}

pub fn part2(input: String) -> String {
    let (_, workflows) = parse_input(&input);

    let workflows: HashMap<String, Workflow> =
        workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    let sections = find_allowed(
        &workflows.get("in").expect("No workflow named 'in'"),
        &workflows,
        vec![],
    );

    sections.iter().map(|s| s.count()).sum::<u64>().to_string()
}

fn find_allowed(
    workflow: &Workflow,
    workflows: &HashMap<String, Workflow>,
    conditions: Vec<Condition>,
) -> Vec<Section> {
    let allowed = workflow
        .rules
        .iter()
        .enumerate()
        .flat_map(|(index, rule)| {
            let additional_condtions = workflow
                .rules
                .iter()
                .take(index)
                .flat_map(|r| match r {
                    Rule::Condition { condition, .. } => Some(condition.invert()),
                    _ => None,
                })
                .collect::<Vec<_>>();

            let mut new_conditions = conditions.clone();
            new_conditions.extend(additional_condtions);
            if let Rule::Condition { condition, .. } = rule {
                new_conditions.push(condition.clone());
            }

            find_allowed_for_target(rule.target(), workflows, new_conditions)
        })
        .collect::<Vec<_>>();

    allowed
}

fn find_allowed_for_target(
    target: &Target,
    workflows: &HashMap<String, Workflow>,
    conditions: Vec<Condition>,
) -> Vec<Section> {
    match target {
        Target::Workflow(name) => {
            let workflow = workflows.get(name).expect("No workflow with that name");
            find_allowed(workflow, workflows, conditions)
        }
        Target::Rejected => vec![],
        Target::Accepted => vec![find_allowed_with_conditions(&conditions)],
    }
}

fn find_allowed_with_conditions(conditions: &[Condition]) -> Section {
    let x = find_allowed_with_conditions_in_category(conditions, Category::X);
    let m = find_allowed_with_conditions_in_category(conditions, Category::M);
    let a = find_allowed_with_conditions_in_category(conditions, Category::A);
    let s = find_allowed_with_conditions_in_category(conditions, Category::S);

    Section { x, m, a, s }
}

fn find_allowed_with_conditions_in_category(
    conditions: &[Condition],
    category: Category,
) -> Range<u16> {
    let applicable_conditions = conditions
        .iter()
        .filter(|c| c.category == category)
        .collect::<Vec<_>>();

    let min = applicable_conditions
        .iter()
        .filter(|c| c.operator == Operator::GreaterThan)
        .map(|c| c.value + 1)
        .max()
        .unwrap_or(1);

    let max = applicable_conditions
        .iter()
        .filter(|c| c.operator == Operator::LessThan)
        .map(|c| c.value)
        .min()
        .unwrap_or(4001);

    min..max
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Section {
    x: Range<u16>,
    m: Range<u16>,
    a: Range<u16>,
    s: Range<u16>,
}

impl Section {
    fn contains(&self, part: &Part) -> bool {
        self.x.contains(&part.x)
            && self.m.contains(&part.m)
            && self.a.contains(&part.a)
            && self.s.contains(&part.s)
    }

    fn count(&self) -> u64 {
        let x: u64 = self.x.end as u64 - self.x.start as u64;
        let m: u64 = self.m.end as u64 - self.m.start as u64;
        let a: u64 = self.a.end as u64 - self.a.start as u64;
        let s: u64 = self.s.end as u64 - self.s.start as u64;

        x * m * a * s
    }
}

trait Intersection {
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
}

impl Intersection for Section {
    fn intersection(&self, other: &Section) -> Option<Section> {
        let x = self.x.intersection(&other.x)?;
        let m = self.m.intersection(&other.m)?;
        let a = self.a.intersection(&other.a)?;
        let s = self.s.intersection(&other.s)?;

        Some(Section { x, m, a, s })
    }
}

impl Intersection for Range<u16> {
    fn intersection(&self, other: &Range<u16>) -> Option<Range<u16>> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start >= end {
            None
        } else {
            Some(start..end)
        }
    }
}

#[derive(Debug)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn rating(&self) -> u64 {
        (self.x + self.m + self.a + self.s) as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Condition {
    category: Category,
    operator: Operator,
    value: u16,
}

impl Condition {
    fn invert(&self) -> Self {
        let new_operator = match self.operator {
            Operator::LessThan => Operator::GreaterThan,
            Operator::GreaterThan => Operator::LessThan,
        };
        Self {
            category: self.category.clone(),
            operator: new_operator,
            value: match self.operator {
                Operator::LessThan => self.value - 1,
                Operator::GreaterThan => self.value + 1,
            },
        }
    }
}

#[derive(Debug)]
enum Rule {
    Condition {
        condition: Condition,
        target: Target,
    },
    Direct {
        target: Target,
    },
}

impl Rule {
    fn target(&self) -> &Target {
        match self {
            Self::Condition { target, .. } => target,
            Self::Direct { target } => target,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operator {
    LessThan,
    GreaterThan,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Self::LessThan),
            ">" => Ok(Self::GreaterThan),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Target {
    Workflow(String),
    Rejected,
    Accepted,
}

impl FromStr for Target {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Rejected),
            "A" => Ok(Self::Accepted),
            _ => Ok(Self::Workflow(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn parse_input(input: &str) -> (Vec<Part>, Vec<Workflow>) {
    let split = input.split("\n\n").collect::<Vec<_>>();

    if split.len() != 2 {
        panic!("Invalid input");
    }

    let workflows = parse_workflows(split[0]).unwrap().1;
    let parts = parse_parts(split[1]).unwrap().1;

    (parts, workflows)
}

fn parse_workflows(input: &str) -> IResult<&str, Vec<Workflow>> {
    separated_list1(newline, parse_workflow)(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, rules) = separated_list1(tag(","), parse_rule)(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((
        input,
        Workflow {
            name: name.to_string(),
            rules,
        },
    ))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, first) = alpha1(input)?;

    let Ok(category) = first.parse::<Category>() else {
        return Ok((
            input,
            Rule::Direct {
                target: first.parse::<Target>().expect("Invalid target"),
            },
        ));
    };

    let (input, operator) = take(1usize)
        .map(|s: &str| s.parse::<Operator>().expect("Invalid operator"))
        .parse(input)?;

    let (input, value) = digit1
        .map(|s: &str| s.parse::<u16>().expect("Invalid value"))
        .parse(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, target) = alpha1
        .map(|s: &str| s.parse::<Target>().expect("Invalid target"))
        .parse(input)?;

    Ok((
        input,
        Rule::Condition {
            condition: Condition {
                category,
                operator,
                value,
            },
            target,
        },
    ))
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(newline, parse_part)(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = digit1
        .map(|s: &str| s.parse::<u16>().expect("Invalid x"))
        .parse(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = digit1
        .map(|s: &str| s.parse::<u16>().expect("Invalid m"))
        .parse(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = digit1
        .map(|s: &str| s.parse::<u16>().expect("Invalid a"))
        .parse(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = digit1
        .map(|s: &str| s.parse::<u16>().expect("Invalid s"))
        .parse(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, Part { x, m, a, s }))
}
