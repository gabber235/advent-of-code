use regex::Regex;
use std::sync::LazyLock;

static NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-?\d+").unwrap());

pub fn numbers(input: &str) -> Vec<i64> {
    NUMBER_REGEX
        .find_iter(input)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

pub fn numbers_usize(input: &str) -> Vec<usize> {
    NUMBER_REGEX
        .find_iter(input)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

pub fn blocks(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

pub fn grid_chars(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn grid_digits(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect()
}

pub fn words(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

pub fn first_number(input: &str) -> Option<i64> {
    NUMBER_REGEX
        .find(input)
        .and_then(|m| m.as_str().parse().ok())
}

pub fn lines_with_numbers(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(numbers).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers() {
        assert_eq!(numbers("1 2 3"), vec![1, 2, 3]);
        assert_eq!(numbers("pos: 10, -20, 30"), vec![10, -20, 30]);
        assert_eq!(numbers("no numbers here"), Vec::<i64>::new());
        assert_eq!(numbers("mixed: abc123def-456ghi"), vec![123, -456]);
    }

    #[test]
    fn test_blocks() {
        let input = "block1\nline2\n\nblock2\nline2";
        let result = blocks(input);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "block1\nline2");
        assert_eq!(result[1], "block2\nline2");
    }

    #[test]
    fn test_grid_chars() {
        let input = "abc\ndef";
        let result = grid_chars(input);
        assert_eq!(result, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
    }

    #[test]
    fn test_grid_digits() {
        let input = "123\n456";
        let result = grid_digits(input);
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_words() {
        assert_eq!(words("hello world  test"), vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_first_number() {
        assert_eq!(first_number("abc 42 def 100"), Some(42));
        assert_eq!(first_number("no numbers"), None);
    }
}
