use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn part1(input: String) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .par_iter()
        .map(|line| {
            let (assignment, broken) = parse_input(line);
            count_possible_assignments(assignment, &broken)
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .par_iter()
        .map(|line| {
            let (assignment, broken) = parse_input(line);
            let (assignment, broken) = duplicate5(assignment, broken);
            count_possible_assignments(&assignment, &broken)
        })
        .sum::<usize>()
        .to_string()
}

pub fn duplicate5(assignemnt: &str, broken: Vec<usize>) -> (String, Vec<usize>) {
    let mut new_assignment = assignemnt.to_string();
    let mut new_broken = broken.clone();

    for _ in 0..4 {
        new_assignment.push_str(format!("?{}", assignemnt).as_str());
        new_broken.extend(broken.iter());
    }

    (new_assignment, new_broken)
}

pub fn parse_input(input: &str) -> (&str, Vec<usize>) {
    let mut split = input.split(' ');

    let assignment = split.next().unwrap();
    let broken = split
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (assignment, broken)
}

pub fn count_possible_assignments(assignment: &str, groups: &[usize]) -> usize {
    let mut found_solutions = vec![vec![None; assignment.len()]; groups.len()];
    let count = count_assignments(assignment, groups, 0, 0, &mut found_solutions);

    count
}

fn count_assignments(
    pattern: &str,
    groups: &[usize],
    char_index: usize,
    group_index: usize,
    found_solutions: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    // If we've found all the groups, we want no more groups to exists
    if group_index == groups.len() {
        if slice(pattern, char_index, pattern.len())
            .chars()
            .all(|c| c != '#')
        {
            return 1;
        } else {
            return 0;
        }
    }

    // If we've reached the end of the pattern, we want no more groups to exists
    if char_index >= pattern.len() {
        return 0;
    }

    // If we've already found a solution for this group and char_index, return it
    if let Some(solution) = found_solutions[group_index][char_index] {
        return solution;
    }

    // Try to find the group at the next char_index
    let mut count = 0;
    if pattern.chars().nth(char_index) != Some('#') {
        count += count_assignments(
            pattern,
            groups,
            char_index + 1,
            group_index,
            found_solutions,
        );
    }

    // Try to find the group at the current char_index
    let group_size = groups[group_index];
    let space_left = char_index + group_size <= pattern.len();
    let no_seperator = slice(pattern, char_index, char_index + group_size)
        .chars()
        .all(|c| c != '.');
    let seperator_after = space_left && pattern.chars().nth(char_index + group_size) != Some('#');

    if space_left && no_seperator && seperator_after {
        count += count_assignments(
            pattern,
            groups,
            char_index + group_size + 1,
            group_index + 1,
            found_solutions,
        );
    }

    // Save the solution for this group and char_index
    found_solutions[group_index][char_index] = Some(count);

    count
}

fn slice(pattern: &str, start: usize, end: usize) -> &str {
    let end = end.min(pattern.len());
    let start = start.min(pattern.len()).min(end);
    &pattern[start..end]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_count_possible_assignments(#[case] input: &str, #[case] expected: usize) {
        let (assignment, broken) = parse_input(input);
        let count = count_possible_assignments(assignment, &broken);

        assert_eq!(count, expected);
    }
}
