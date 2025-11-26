use nom::{
    character::complete::{i64, space1},
    multi::separated_list1,
    IResult,
};
use rayon::prelude::*;

pub fn part1(input: String) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .par_iter()
        .map(|line| parse_readings(line).unwrap().1)
        .map(|readings| predict_next(&readings))
        .sum::<i64>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .par_iter()
        .map(|line| parse_readings(line).unwrap().1)
        .map(|readings| predict_previous(&readings))
        .sum::<i64>()
        .to_string()
}

fn parse_readings(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, i64)(input)
}

fn predict_next(readings: &[i64]) -> i64 {
    // If all readings are 0, then the next reading is 0.
    if readings.iter().all(|&r| r == 0) {
        return 0;
    }

    // If not, then we create a new vector of the differences between each consecutive reading.
    let mut diffs = Vec::with_capacity(readings.len() - 1);
    for i in 1..readings.len() {
        diffs.push(readings[i] - readings[i - 1]);
    }

    let prediction = predict_next(&diffs);

    // We then add the prediction to the last reading to get the next reading.
    readings[readings.len() - 1] + prediction
}

fn predict_previous(readings: &[i64]) -> i64 {
    // If all readings are 0, then the previous reading is 0.
    if readings.iter().all(|&r| r == 0) {
        return 0;
    }

    // If not, then we create a new vector of the differences between each consecutive reading.
    let mut diffs = Vec::with_capacity(readings.len() - 1);
    for i in 1..readings.len() {
        diffs.push(readings[i] - readings[i - 1]);
    }

    let prediction = predict_previous(&diffs);

    readings[0] - prediction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_next() {
        let readings = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict_next(&readings), 18);
        let readings = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(predict_next(&readings), 28);
        let readings = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_next(&readings), 68);
    }

    #[test]
    fn test_predict_previous() {
        let readings = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict_previous(&readings), -3);
        let readings = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(predict_previous(&readings), 0);
        let readings = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_previous(&readings), 5);
    }
}
