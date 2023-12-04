use std::{cmp::PartialEq, collections::HashMap, fmt};

pub fn run() {
    let input = include_str!("./input.txt");
    let parsed_input = parse_input(input);
    let sum = parse_games(parsed_input);
    println!("2-1: {}", sum);
}

#[derive(PartialEq)]
struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}

impl CubeSet {
    fn is_possible(&self, upper_limit: &Self) -> bool {
        self.red <= upper_limit.red
            && self.blue <= upper_limit.blue
            && self.green <= upper_limit.green
    }
}

impl fmt::Debug for CubeSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "red: {}, green: {}, blue: {}",
            self.red, self.green, self.blue
        )
    }
}

#[derive(PartialEq)]
struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn is_possible(&self, upper_limit_cube_set: &CubeSet) -> bool {
        self.cube_sets
            .iter()
            .all(|cube_set| cube_set.is_possible(upper_limit_cube_set))
    }
}

fn parse_games(input: Vec<&str>) -> usize {
    let upper_limit = CubeSet {
        red: 12,
        blue: 14,
        green: 13,
    };
    input.iter().fold(0, |acc, &line| {
        if !line.contains(':') {
            return acc;
        }

        let game = parse_line(line);
        match game.is_possible(&upper_limit) {
            true => acc + game.id,
            false => acc,
        }
    })
}

fn parse_line(line: &str) -> Game {
    let game_separator_index = line.find(':').unwrap();
    let id = line[5..game_separator_index].parse::<usize>().unwrap();
    let cube_sets_raw: Vec<&str> = line[game_separator_index + 2..].split("; ").collect();
    let cube_sets = cube_sets_raw
        .into_iter()
        .map(|cube_set_raw| {
            let cube_entries_raw: Vec<&str> = cube_set_raw.split(", ").collect();
            let mut cube_entries: HashMap<&str, usize> = HashMap::new();
            for &entry in cube_entries_raw.iter() {
                let color_separator_index = entry.find(' ').unwrap();
                let amount = entry[..color_separator_index].parse::<usize>().unwrap();
                let color = &entry[color_separator_index + 1..];
                cube_entries.insert(color, amount);
            }
            CubeSet {
                red: cube_entries.get("red").cloned().unwrap_or(0),
                blue: cube_entries.get("blue").cloned().unwrap_or(0),
                green: cube_entries.get("green").cloned().unwrap_or(0),
            }
        })
        .collect();
    Game { id, cube_sets }
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut parsed_input: Vec<&str> = vec![];

    for line in input.lines() {
        parsed_input.push(line);
    }

    parsed_input
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 11: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game {
        id: 11,
        cube_sets: vec![
            CubeSet { red: 4, blue: 3, green: 0},
            CubeSet { red: 1, blue: 6, green: 2},
            CubeSet { red: 0, blue: 0, green: 2},
        ]
    })]
    fn test_parse_line(#[case] input: &str, #[case] expected: Game) {
        let output = parse_line(input);
        assert!(output == expected);
    }

    #[rstest]
    #[case(CubeSet { red: 4, blue: 3, green: 2 }, CubeSet { red: 12, blue: 14, green: 13 }, true)]
    #[case(CubeSet { red: 20, blue: 3, green: 2 }, CubeSet { red: 12, blue: 14, green: 13 }, false)]
    fn test_is_possible(
        #[case] cube_set: CubeSet,
        #[case] upper_limit: CubeSet,
        #[case] possible: bool,
    ) {
        assert_eq!(cube_set.is_possible(&upper_limit), possible);
    }

    #[rstest]
    #[case(
        r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#,
        8
    )]
    fn test_parse_games(#[case] input: &str, #[case] sum: usize) {
        let parsed_input = parse_input(input);
        assert_eq!(parse_games(parsed_input), sum);
    }
}
