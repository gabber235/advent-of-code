use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

pub fn part1(input: String) -> String {
    let games = input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .collect::<Vec<Game>>();

    let valid_games = games
        .into_iter()
        .filter(|game| game.is_valid())
        .collect::<Vec<Game>>();

    return format!(
        "{}",
        valid_games.into_iter().map(|game| game.id).sum::<usize>()
    );
}

pub fn part2(input: String) -> String {
    let games = input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .collect::<Vec<Game>>();

    let power = games
        .into_iter()
        .map(|game| game.minimum_cubes())
        .map(|(red, green, blue)| red * green * blue)
        .sum::<usize>();

    return format!("Power: {}", power);
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    // A game is valid if in each round the number of red<=12, green<=13, blue<=14
    fn is_valid(&self) -> bool {
        self.rounds.iter().all(|round| {
            let (red, green, blue) = round.cubes();
            red <= 12 && green <= 13 && blue <= 14
        })
    }

    fn minimum_cubes(&self) -> (usize, usize, usize) {
        self.rounds.iter().fold(
            (usize::MIN, usize::MIN, usize::MIN),
            |(red, green, blue), round| {
                let (r, g, b) = round.cubes();
                (red.max(r), green.max(g), blue.max(b))
            },
        )
    }
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

impl Round {
    fn cubes(&self) -> (usize, usize, usize) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        self.cubes.iter().for_each(|cube| match cube {
            Cube::Red(n) => red += n,
            Cube::Green(n) => green += n,
            Cube::Blue(n) => blue += n,
        });
        (red, green, blue)
    }
}

#[derive(Debug)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, game) = map(
        tuple((
            preceded(tag("Game "), digit1),
            preceded(tag(": "), parse_rounds),
        )),
        |(id, rounds)| Game {
            id: id.parse::<usize>().unwrap(),
            rounds,
        },
    )(input)?;

    Ok((input, game))
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
    let (input, rounds) = map(separated_list1(tag("; "), parse_round), |rounds| {
        rounds.into_iter().collect()
    })(input)?;

    Ok((input, rounds))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = map(separated_list1(tag(", "), parse_cube), |cubes| Round {
        cubes,
    })(input)?;

    Ok((input, cubes))
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, digit) = digit1(input)?;
    let (input, cube) = alt((
        map(tag(" red"), |_| Cube::Red(digit.parse::<usize>().unwrap())),
        map(tag(" green"), |_| {
            Cube::Green(digit.parse::<usize>().unwrap())
        }),
        map(tag(" blue"), |_| {
            Cube::Blue(digit.parse::<usize>().unwrap())
        }),
    ))(input)?;
    Ok((input, cube))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let (input, game) = parse_game(input).unwrap();
        println!("input: {}, game: {:?}", input, game);
        assert_eq!(input, "");
        assert_eq!(game.id, 4);
        assert_eq!(game.rounds.len(), 3);
        assert_eq!(game.rounds[0].cubes.len(), 3);
        assert_eq!(game.rounds[1].cubes.len(), 2);
        assert_eq!(game.rounds[2].cubes.len(), 3);
    }

    #[test]
    fn test_minimum_cubes() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let (input, game) = parse_game(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(game.minimum_cubes(), (14, 3, 15));
    }
}
