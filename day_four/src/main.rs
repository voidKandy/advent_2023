use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let cards = CardPile::from(input.as_str());
    println!("{}", cards.sum());
}

#[derive(Debug)]
pub struct CardPile(Vec<ScratchCard>);

#[derive(Debug)]
pub struct ScratchCard {
    winning_nums: Vec<u32>,
    given_nums: Vec<(u32, bool)>,
}

impl From<&str> for CardPile {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .into_iter()
                .filter_map(|l| {
                    if !l.trim().is_empty() {
                        Some(ScratchCard::from(l))
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}

impl From<&str> for ScratchCard {
    fn from(value: &str) -> Self {
        let (winning_str, given_str) = value
            .split_once(':')
            .expect(&format!("Couldn't split {} on ':'", value))
            .1
            .split_once('|')
            .expect(&format!("Couldn't split {} on '|'", value));
        let winning_nums: Vec<u32> = winning_str
            .split(' ')
            .into_iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        let given_nums = given_str
            .split(' ')
            .into_iter()
            .filter_map(|s| {
                if let Some(num) = s.parse().ok() {
                    if winning_nums.iter().any(|n| n == &num) {
                        Some((num, true))
                    } else {
                        Some((num, false))
                    }
                } else {
                    None
                }
            })
            .collect();
        Self {
            winning_nums,
            given_nums,
        }
    }
}

impl CardPile {
    fn sum(&self) -> u64 {
        self.0.iter().fold(0, |mut sum, c| {
            sum += c.score();
            sum
        })
    }
}

impl ScratchCard {
    fn score(&self) -> u64 {
        self.given_nums.iter().fold(0, |mut score, (n, b)| {
            match *b {
                true => {
                    println!("{} IS A WINNER", n);
                    println!("SCORE BEFORE: {}", score);
                    if score == 0 {
                        score += 1;
                    } else {
                        score = score * 2;
                    }
                    println!("SCORE AFTER: {}", score);
                }
                false => score = score,
            }
            score
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::CardPile;

    #[test]
    fn solve_part_one_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards: CardPile = CardPile::from(input);
        println!("{:?}", cards);
        assert_eq!(8, cards.0[0].score());
        assert_eq!(2, cards.0[1].score());
        assert_eq!(2, cards.0[2].score());
        assert_eq!(1, cards.0[3].score());
        assert_eq!(0, cards.0[4].score());
        assert_eq!(0, cards.0[5].score());
        assert_eq!(13, cards.sum());
    }
}
