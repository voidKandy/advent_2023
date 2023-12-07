use std::{env, fs};

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: cargo run -- <pt1 or pt2>");
        return;
    }
    let input = fs::read_to_string("src/input.txt").unwrap();

    match args.nth(1).as_deref() {
        Some("pt1") => {
            let races = test_time_pairs_from_str(&input);
            let product = races
                .into_iter()
                .fold(1, |acc, r| acc * get_wins(r).len() as u64);
            println!("{}", product);
        }
        Some("pt2") => {
            let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
            let tup = {
                (
                    lines[0]
                        .split_once(':')
                        .unwrap()
                        .1
                        .split_whitespace()
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<&str>>()
                        .join("")
                        .parse()
                        .unwrap(),
                    lines[1]
                        .split_once(':')
                        .unwrap()
                        .1
                        .split_whitespace()
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<&str>>()
                        .join("")
                        .parse()
                        .unwrap(),
                )
            };
            let wins = get_wins(tup);
            println!("{}", wins.len());
        }
        _ => println!("Invalid argument. Use 'pt1' or 'pt2'."),
    }
}

type TimeDistanceTup = (u64, u64);

fn test_time_pairs_from_str(str: &str) -> Vec<TimeDistanceTup> {
    let (times, distances): (Vec<u64>, Vec<u64>) = str
        .lines()
        .filter(|l| !l.trim().is_empty())
        .fold((vec![], vec![]), |(mut times, mut distances), l| {
            let mut parts: Vec<u64> = l
                .split_once(':')
                .expect("Failed to split on :")
                .1
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if l.to_lowercase().contains("time") {
                times.append(&mut parts);
            } else if l.to_lowercase().contains("distance") {
                distances.append(&mut parts);
            }

            (times, distances)
        });
    (0..times.len()).into_iter().fold(vec![], |mut acc, i| {
        acc.push((times[i], distances[i]));
        acc
    })
}

/// Gets all time to distance possiblities
fn get_race_possibilities(tup: TimeDistanceTup) -> Vec<TimeDistanceTup> {
    (0..=tup.0).into_iter().fold(vec![], |mut tup_acc, i| {
        let difference: u64 = tup.0 - i;
        let ml_per_ms: u64 = i;
        let distance_travelled = ml_per_ms * difference;
        println!("ON {}th iteration\nTime left after holding: {}\nMillimeters per ms: {}\nTotal distance: {}\n", i, difference, ml_per_ms, distance_travelled);
        tup_acc.push((i, distance_travelled));
        tup_acc
    })
}

fn get_wins(tup: TimeDistanceTup) -> Vec<u64> {
    get_race_possibilities(tup)
        .into_iter()
        .filter_map(|t| if t.1 > tup.1 { Some(t.0) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{get_race_possibilities, get_wins, test_time_pairs_from_str};

    #[test]
    fn solve_example_one() {
        let input = "Time:      7  15   30
            Distance:  9  40  200";
        let races = test_time_pairs_from_str(input);
        assert_eq!(vec![(7, 9), (15, 40), (30, 200)], races);
        let possibilities = get_race_possibilities(races[0]);
        assert_eq!(
            vec![
                (0, 0),
                (1, 6),
                (2, 10),
                (3, 12),
                (4, 12),
                (5, 10),
                (6, 6),
                (7, 0)
            ],
            possibilities
        );
        assert_eq!(vec![2, 3, 4, 5], get_wins(races[0]));
        assert_eq!(
            vec![4, 8, 9],
            races
                .iter()
                .map(|r| get_wins(*r).len())
                .collect::<Vec<usize>>()
        );
        assert_eq!(
            288,
            races
                .into_iter()
                .fold(1, |acc, r| { acc * get_wins(r).len() })
        )
    }
}
