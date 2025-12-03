use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| bigest_sub_number(line, 2))
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| bigest_sub_number(line, 12))
        .sum::<u64>()
        .to_string()
}

fn bigest_sub_number(num: &str, max_len: usize) -> u64 {
    let mut selected_digits = vec![0; max_len];

    let digits = num
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    for i in digits.len() - max_len..digits.len() {
        selected_digits[i - (digits.len() - max_len)] = digits[i];
    }

    for i in (0..digits.len() - max_len).rev() {
        let digit = digits[i];
        let first_selected_digit = selected_digits[0];

        if digit < first_selected_digit {
            continue;
        }

        remove_first_increasing_digit(&mut selected_digits);
        selected_digits.insert(0, digit);
    }

    selected_digits
        .iter()
        .fold(0, |acc, &digit| acc * 10 + digit)
}

fn remove_first_increasing_digit(digits: &mut Vec<u64>) {
    for i in 0..digits.len() - 1 {
        if digits[i] < digits[i + 1] {
            digits.remove(i);
            return;
        }
    }
    digits.remove(digits.len() - 1);
}
