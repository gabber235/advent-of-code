pub fn part1(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let max_bit_count = lines.iter().map(|line| line.len()).max().unwrap();

    let mut gamma = String::new();

    for i in 0..max_bit_count {
        let index = i as u8;
        let nth_bit_set = most_common_nth_bit(&lines, index);
        if nth_bit_set == Bit::One || nth_bit_set == Bit::Both {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }
    println!("Gamma: {}", gamma);
    let epsilon = gamma
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();
    println!("Epsilon: {}", epsilon);

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    format!("{}", gamma * epsilon)
}

pub fn part2(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let max_bit_count = lines.iter().map(|line| line.len()).max().unwrap();

    let oxygen_generator_rating = oxygen_generator_rating(lines.clone(), max_bit_count as u32);
    let co2_scrubber_rating = co2_scrubber_rating(lines, max_bit_count as u32);

    println!("Oxygen Generator Rating: {}", oxygen_generator_rating);
    println!("CO2 Scrubber Rating: {}", co2_scrubber_rating);

    format!("{}", oxygen_generator_rating * co2_scrubber_rating)
}

fn oxygen_generator_rating(mut numbers: Vec<&str>, max_bit_count: u32) -> u32 {
    let mut index = 0;
    while index < max_bit_count && numbers.len() > 1 {
        let nth_bit_set = most_common_nth_bit(&numbers, index as u8);
        if nth_bit_set == Bit::One || nth_bit_set == Bit::Both {
            numbers.retain(|&num| get_nth_char(num, index as usize) == '1');
        } else {
            numbers.retain(|&num| get_nth_char(num, index as usize) != '1');
        }
        index += 1;
    }
    u32::from_str_radix(numbers[0], 2).unwrap()
}

fn co2_scrubber_rating(mut numbers: Vec<&str>, max_bit_count: u32) -> u32 {
    let mut index = 0;
    while index < max_bit_count && numbers.len() > 1 {
        let nth_bit_set = most_common_nth_bit(&numbers, index as u8);
        if nth_bit_set == Bit::One || nth_bit_set == Bit::Both {
            numbers.retain(|&num| get_nth_char(num, index as usize) == '0');
        } else {
            numbers.retain(|&num| get_nth_char(num, index as usize) != '0');
        }
        index += 1;
    }
    u32::from_str_radix(numbers[0], 2).unwrap()
}

#[derive(Debug, PartialEq)]
enum Bit {
    Zero,
    One,
    Both,
}

fn most_common_nth_bit(bin_list: &Vec<&str>, index: u8) -> Bit {
    let ones_count = bin_list
        .iter()
        .filter(|&num| get_nth_char(num, index as usize) == '1')
        .count();

    let zeros_count = bin_list
        .iter()
        .filter(|&num| get_nth_char(num, index as usize) == '0')
        .count();

    if ones_count > zeros_count {
        Bit::One
    } else if zeros_count > ones_count {
        Bit::Zero
    } else {
        Bit::Both
    }
}
fn get_nth_char(s: &str, n: usize) -> char {
    s.chars().nth(n).unwrap_or('0')
}
