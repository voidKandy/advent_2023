use std::{cmp::max, env, fs};

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: cargo run -- <pt1 or pt2>");
        return;
    }

    let input = fs::read_to_string("src/input.txt").unwrap();
    let predicate = Predicate {
        blue: 14,
        red: 12,
        green: 13,
    };
    match args.nth(1).as_deref() {
        Some("pt1") => println!("{}", solve_for_pt_1(&input, predicate)),
        Some("pt2") => println!("{}", solve_for_pt_2(&input)),
        _ => println!("Invalid argument. Use 'pt1' or 'pt2'."),
    }
}

fn solve_for_pt_1(input: &str, predicate: Predicate) -> u32 {
    let lines = input.lines().filter(|l| !l.trim().is_empty());
    let games = lines.map(|l| Game::try_from(l.trim()).unwrap()).collect();
    let games = find_possible_games(games, predicate);
    games.iter().fold(0, |acc, game| acc + game.id)
}

fn solve_for_pt_2(input: &str) -> u64 {
    let lines = input.lines().filter(|l| !l.trim().is_empty());
    let games: Vec<Game> = lines.map(|l| Game::try_from(l.trim()).unwrap()).collect();
    games.into_iter().map(|g| g.power_of_cubes()).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    id: u32,
    blue: u32,
    red: u32,
    green: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    blue: u32,
    red: u32,
    green: u32,
}

impl<'a> TryFrom<&'a str> for Game {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some((id_chunk, blocks_chunk)) = value.split_once(':') {
            let (mut blue, mut red, mut green) = (0, 0, 0);
            let colors = vec!["blue", "red", "green"];

            let id: u32 = id_chunk
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u32>()
                .expect("Failed to get int from ID chunk");

            blocks_chunk.split(';').into_iter().for_each(|reveal| {
                let mut blocks_chunks: Vec<&str> = reveal.split(',').collect();
                let (mut inner_blue, mut inner_red, mut inner_green) = (0, 0, 0);
                colors.iter().for_each(|color| {
                    blocks_chunks.iter_mut().for_each(|ch| {
                        if ch.contains(color) {
                            if let Some(num) = ch
                                .chars()
                                .filter(|c| c.is_numeric())
                                .collect::<String>()
                                .parse::<u32>()
                                .ok()
                            {
                                match color.to_owned() {
                                    "blue" => inner_blue += num,
                                    "green" => inner_green += num,
                                    "red" => inner_red += num,
                                    _ => panic!("No other color value should exist!"),
                                }
                            }
                        }
                    })
                });
                blue = max(blue, inner_blue);
                green = max(green, inner_green);
                red = max(red, inner_red);
            });

            Ok(Game {
                id,
                blue,
                red,
                green,
            })
        } else {
            Err(format!("{} could not be split at :", value).into())
        }
    }
}

impl Predicate {
    pub fn game_possible(&self, game: &Game) -> bool {
        game.blue <= self.blue && game.green <= self.green && game.red <= self.red
    }
}

impl Game {
    pub fn power_of_cubes(&self) -> u64 {
        (self.blue * self.green * self.red) as u64
    }
}

fn find_possible_games(games: Vec<Game>, predicate: Predicate) -> Vec<Game> {
    games
        .into_iter()
        .filter(|g| predicate.game_possible(g))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{solve_for_pt_1, solve_for_pt_2, Game, Predicate};

    #[test]
    fn parse_single_line_into_game() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected_game = Game {
            id: 1,
            blue: 6,
            red: 4,
            green: 2,
        };
        assert_eq!(expected_game, Game::try_from(line).unwrap());
    }

    #[test]
    fn pt_one_case() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n
        ";
        let predicate = Predicate {
            blue: 14,
            green: 13,
            red: 12,
        };
        assert_eq!(8, solve_for_pt_1(input, predicate))
    }

    #[test]
    fn pt_two_test_case() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n
        ";
        assert_eq!(2286, solve_for_pt_2(input));
    }
}
