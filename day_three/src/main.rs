use std::{char, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let grid = Grid::from_input_string(&input);

    // println!("{:?}", grid);
    println!("{}", grid.get_sum_of_all_part_numbers());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GridSpot {
    value: GridValue,
    positions: Vec<(usize, usize)>,
    adjacencies: Vec<(usize, usize)>,
    is_part: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid(Vec<GridSpot>);

#[derive(Debug, Clone, PartialEq, Eq)]
enum GridValue {
    Number(u64),
    Character(char),
}

impl TryFrom<&str> for GridValue {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(number) = value.parse::<u64>().ok() {
            return Ok(Self::Number(number));
        }
        match value.len() == 1 {
            true => Ok(Self::Character(value.chars().nth(0).unwrap())),
            false => Err(format!(
                "Str: [{}] not parsable to u64, and is longer than one character",
                value
            )
            .into()),
        }
    }
}

impl Into<String> for GridValue {
    fn into(self) -> String {
        match self {
            Self::Number(num) => format!("{}", num),
            Self::Character(char) => format!("{}", char),
        }
    }
}

impl GridValue {
    fn inner_num(&self) -> Option<u64> {
        match self {
            Self::Number(num) => Some(*num),
            _ => None,
        }
    }
    fn is_character(&self) -> bool {
        match self {
            GridValue::Character(_) => true,
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match self {
            GridValue::Number(_) => true,
            _ => false,
        }
    }
}

impl Grid {
    fn from_input_string(input: &str) -> Self {
        let mut grid = Self(vec![]);
        let lines: Vec<&str> = input
            .lines()
            .filter_map(|l| {
                let trim = l.trim();
                if !l.is_empty() {
                    Some(trim)
                } else {
                    None
                }
            })
            .collect();
        let len = lines.len() - 1;
        lines
            .into_iter()
            .enumerate()
            .for_each(|(i, l)| grid.0.append(&mut Self::from_line_info(l, i, i == len)));
        grid.check_for_parts();
        grid
    }

    fn from_line_info(line: &str, line_num: usize, last_line: bool) -> Vec<GridSpot> {
        let spot_vals: Vec<GridValue> =
            line.split('.')
                .filter(|v| !v.trim().is_empty())
                .fold(vec![], |mut acc, v| {
                    if v.len() > 1 && v.contains(|c: char| c.is_ascii_punctuation()) {
                        if let Some(idx) = v.find(|c: char| c.is_ascii_punctuation()) {
                            let char = v.chars().nth(idx).unwrap();
                            v.split(char)
                                .into_iter()
                                .filter(|k| !k.trim().is_empty())
                                .for_each(|val| {
                                    let spot_value =
                                        GridValue::try_from(val).expect("Spot value get failure");
                                    acc.push(spot_value);
                                });
                            let spot_value = GridValue::try_from(char.to_string().as_str())
                                .expect("Spot value get failure");
                            acc.push(spot_value);
                        }
                    } else {
                        let spot_value = GridValue::try_from(v).expect("Spot value get failure");
                        acc.push(spot_value);
                    }
                    acc
                });
        println!("Got spot values: {:?}", spot_vals);
        let spot_pos_val_tups: Vec<(Vec<(usize, usize)>, GridValue)> =
            spot_vals.iter().fold(vec![], |mut tups, v| {
                let pattern: String = v.clone().into();
                println!(
                    "PATTERN: {}\nLINE: {}\nMATCHES: {:?}\n",
                    pattern,
                    line,
                    line.clone()
                        .match_indices(&pattern)
                        .collect::<Vec<(usize, &str)>>()
                );
                line.match_indices(&pattern).for_each(|(i, p)| {
                    let position = p.chars().enumerate().fold(vec![], |mut acc, (k, _)| {
                        acc.push((line_num, i + k));
                        acc
                    });
                    tups.push((position, v.clone()));
                });
                tups
            });

        println!("Got spot value pos tups: {:?}\n", spot_pos_val_tups);
        spot_pos_val_tups
            .into_iter()
            .fold(vec![], |mut return_spots, (positions, value)| {
                println!("Getting adjacencies for: {:?}", value);
                let adjacencies = positions.clone().into_iter().fold(
                    vec![],
                    |mut adj_acc: Vec<(usize, usize)>, (line_idx, char_idx)| {
                        let include_prev_line = line_idx != 0;
                        let include_next_line = !last_line;
                        let include_prev_char =
                            !positions.iter().any(|p| p.1 < char_idx) && char_idx != 0;
                        let include_next_char =
                            !positions.iter().any(|p| p.1 > char_idx) && char_idx != line.len() - 1;
                        println!("BEFORE PASS {:?}\n", adj_acc);
                        println!(
                            "COORDS: {:?}\nPREV LINE: {}\nNEXT LINE: {}\nPREV CHAR: {}\nNEXT CHAR: {}\n",
                            (line_idx, char_idx),
                            include_prev_line,
                            include_next_line,
                            include_prev_char,
                            include_next_char
                        );

                        // let val_str: String = value.clone().into();
                        let next_char_idx = char_idx + 1;
                        let prev_char_idx = {
                            if char_idx == 0 {
                                None
                            } else {
                                Some(char_idx - 1)
                            }
                        };

                        if include_next_char {
                            adj_acc.push((line_num, next_char_idx))
                        }

                        if include_prev_char {
                            adj_acc.push((line_num, prev_char_idx.unwrap()))
                        }

                        if include_prev_line {
                            adj_acc.push((line_num - 1, char_idx));
                            if include_next_char {
                                adj_acc.push((line_num - 1, next_char_idx))
                            }
                            if include_prev_char {
                                adj_acc.push((line_num - 1, prev_char_idx.unwrap()))
                            }
                        }

                        if include_next_line {
                            adj_acc.push((line_num + 1, char_idx));
                            if include_next_char {
                                adj_acc.push((line_num + 1, next_char_idx))
                            }
                            if include_prev_char {
                                adj_acc.push((line_num + 1, prev_char_idx.unwrap()))
                            }
                        }

                        println!("AFTER PASS {:?}\n", adj_acc);
                        adj_acc
                    },
                );

                let spot = GridSpot {
                    value,
                    positions,
                    adjacencies,
                    is_part: false,
                };
                return_spots.push(spot);
                return_spots
            })
    }

    fn check_for_parts(&mut self) {
        let clone = self.0.clone();
        let char_spot_positions = clone
            .into_iter()
            .filter(|spot| spot.value.is_character())
            .fold(vec![], |mut acc, mut spot| {
                println!("Spot: {:?}", spot);
                acc.append(&mut spot.positions);
                acc
            });

        self.0
            .iter_mut()
            .filter(|spot| spot.value.is_number())
            .for_each(|num_spot| {
                if num_spot
                    .adjacencies
                    .iter()
                    .any(|pos| char_spot_positions.contains(pos))
                {
                    num_spot.is_part = true;
                }
            })
    }

    fn get_sum_of_all_part_numbers(&self) -> u64 {
        self.0
            .iter()
            .filter_map(|s| {
                if s.is_part {
                    if let Some(num) = s.value.inner_num() {
                        Some(num)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Grid, GridSpot, GridValue};

    #[test]
    fn get_spot_from_single_line() {
        let line = "467..114..";
        let grid = Grid::from_input_string(line);
        let spot1 = GridSpot {
            value: GridValue::Number(467),
            positions: vec![(0, 0), (0, 1), (0, 2)],
            adjacencies: vec![(0, 3)],
            is_part: false,
        };
        // println!("{:?}", grid);
        assert_eq!(spot1, grid.0[0]);
    }

    #[test]
    fn get_spot_from_multiple_lines() {
        let lines = "
            467..114..\n
            ...*......\n";

        let grid = Grid::from_input_string(lines);
        let spot1 = GridSpot {
            value: GridValue::Number(467),
            positions: vec![(0, 0), (0, 1), (0, 2)],
            adjacencies: vec![(1, 0), (1, 1), (0, 3), (1, 2), (1, 3)],
            is_part: true,
        };

        // println!("{:?}", grid);
        assert_eq!(spot1, grid.0[0]);
        assert!(!grid.0[1].is_part);
    }

    #[test]
    fn pt_one_example() {
        let lines = "467..114..\n
            ...*......\n
            ..35..633.\n
            ......#...\n
            617*......\n
            .....+.58.\n
            ..592.....\n
            ......755.\n
            ...$.*....\n
            .664.598..\n";

        let grid = Grid::from_input_string(lines);

        // println!("{:?}", grid);
        assert_eq!(4361u64, grid.get_sum_of_all_part_numbers());
    }
}
