use std::collections::HashMap;

pub fn part1(input: String) -> String {
    let map = parse_map(&input);

    let numbers = parse_numbers(&input, &map);

    let sum = numbers
        .iter()
        .filter(|n| is_connected(&map, &n))
        .map(|n| n.value)
        .sum::<u32>();

    format!("{sum}")
}

pub fn part2(input: String) -> String {
    let map = parse_map(&input);

    let numbers = parse_numbers(&input, &map);

    let ratios = map
        .symbols
        .iter()
        .filter(|(_, c)| **c == '*')
        .flat_map(|(p, _)| find_connecting_gears(&numbers, p))
        .map(|(n1, n2)| n1.value * n2.value)
        .sum::<u32>();

    format!("{ratios}")
}

fn parse_map(input: &str) -> Map {
    let width = input.find('\n').unwrap();
    let symbols = input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .filter(|(_, c)| !c.is_digit(10))
        .filter(|(_, c)| *c != '.')
        .map(|(i, c)| (Point::from_index(i, width), c))
        .collect();
    Map { width, symbols }
}

fn is_connected(map: &Map, number: &Number) -> bool {
    for x in -1..=(number.width as i32) {
        for y in -1..=1 {
            let point = Point::new(number.position.x + x, number.position.y + y);
            if map.symbols.contains_key(&point) {
                return true;
            }
        }
    }
    false
}

fn find_connecting_gears(numbers: &[Number], point: &Point) -> Option<(Number, Number)> {
    let neighbors = point.neighbors();

    let connecting_numbers = numbers
        .iter()
        .filter(|n| neighbors.iter().any(|p| n.is_inside(p)))
        .collect::<Vec<_>>();

    if connecting_numbers.len() != 2 {
        return None;
    }

    let n1 = connecting_numbers[0];
    let n2 = connecting_numbers[1];

    return Some((n1.clone(), n2.clone()));
}

fn parse_numbers(input: &str, map: &Map) -> Vec<Number> {
    let digits = input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .collect::<Vec<_>>();

    let mut numbers = Vec::new();
    let mut current: Option<Number> = None;

    for (i, c) in digits {
        if let Some(mut n) = current {
            if n.position.index(map.width) + n.width == i {
                n.width += 1;
                n.value = n.value * 10 + c.to_digit(10).unwrap();
                current = Some(n);
                continue;
            } else {
                numbers.push(n);
            }
        }

        current = Some(Number {
            value: c.to_digit(10).unwrap(),
            position: Point::from_index(i, map.width),
            width: 1,
        });
    }

    if let Some(n) = current {
        numbers.push(n);
    }

    numbers
}

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    position: Point,
    width: usize,
}

impl Number {
    fn is_inside(&self, point: &Point) -> bool {
        point.x >= self.position.x
            && point.x < self.position.x + self.width as i32
            && point.y == self.position.y
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    symbols: HashMap<Point, char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn from_index(index: usize, width: usize) -> Self {
        Self {
            x: (index % width) as i32,
            y: (index / width) as i32,
        }
    }

    fn index(&self, width: usize) -> usize {
        (self.y as usize) * width + (self.x as usize)
    }

    fn neighbors(&self) -> Vec<Self> {
        vec![
            Self::new(self.x - 1, self.y - 1),
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y - 1),
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x - 1, self.y + 1),
            Self::new(self.x, self.y + 1),
            Self::new(self.x + 1, self.y + 1),
        ]
    }
}
