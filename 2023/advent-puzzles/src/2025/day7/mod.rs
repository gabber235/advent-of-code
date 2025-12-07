use std::collections::{BTreeMap, BTreeSet};

pub fn part1(input: String) -> String {
    let lines = input.lines().collect::<Vec<_>>();

    let start_column = lines[0].find('S').unwrap();

    let mut columns = BTreeSet::new();
    columns.insert(start_column);
    let mut splits = 0;
    let width = lines[0].len();

    for y in 1..lines.len() {
        let mut new_columns = BTreeSet::new();
        for x in columns.iter() {
            let Some(cell) = lines[y].chars().nth(*x) else {
                continue;
            };
            match cell {
                '.' => {
                    new_columns.insert(*x);
                }
                '^' => {
                    splits += 1;
                    if *x > 0 {
                        new_columns.insert(x - 1);
                    }
                    if x + 1 < width {
                        new_columns.insert(x + 1);
                    }
                }
                _ => panic!("Invalid cell type: {}", cell),
            }
        }
        columns = new_columns;
    }

    format!("{}", splits)
}

pub fn part2(input: String) -> String {
    let lines = input.lines().collect::<Vec<_>>();

    let start_column = lines[0].find('S').unwrap();

    let mut columns = BTreeMap::new();
    columns.insert(start_column, 1u64);
    let width = lines[0].len();

    for y in 1..lines.len() {
        let mut new_columns = BTreeMap::new();
        for (x, count) in columns.iter() {
            let Some(cell) = lines[y].chars().nth(*x) else {
                continue;
            };
            match cell {
                '.' => {
                    let new_count = new_columns.get(x).unwrap_or(&0) + *count;
                    new_columns.insert(*x, new_count);
                }
                '^' => {
                    if *x > 0 {
                        let left_count = new_columns.get(&(x - 1)).unwrap_or(&0);
                        new_columns.insert(x - 1, left_count + *count);
                    }
                    if x + 1 < width {
                        let right_count = new_columns.get(&(x + 1)).unwrap_or(&0);
                        new_columns.insert(x + 1, right_count + *count);
                    }
                }
                _ => panic!("Invalid cell type: {}", cell),
            }
        }
        columns = new_columns;
    }

    columns.values().sum::<u64>().to_string()
}
