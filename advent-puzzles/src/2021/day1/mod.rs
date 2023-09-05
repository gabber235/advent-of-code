use iterslide::SlideIterator;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .slide(2)
        .filter(|window| window[0] < window[1])
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .slide(3)
        .map(|window| window.iter().sum::<i32>())
        .slide(2)
        .filter(|window| window[0] < window[1])
        .count()
        .to_string()
}
