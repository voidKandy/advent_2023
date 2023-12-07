use core::cmp::Ordering;

use std::{char, collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let hands = get_hands_from_input(&input);
    println!("{}", total_winnings(hands))
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    h_type: HandType,
    ordered_uniques: Vec<Card>,
    bid: u64,
}

type Hands = Vec<Hand>;

trait Heirarchy<'a> {
    fn heirarchy() -> Vec<&'a Self>;
}

impl<'a> Heirarchy<'a> for Card {
    fn heirarchy() -> Vec<&'a Self> {
        vec![&Card::Ace, &Card::King, &Card::Queen, &Card::Jack]
    }
}

impl<'a> Heirarchy<'a> for HandType {
    fn heirarchy() -> Vec<&'a Self> {
        vec![
            &HandType::FiveOfAKind,
            &HandType::FourOfAKind,
            &HandType::FullHouse,
            &HandType::ThreeOfAKind,
            &HandType::TwoPair,
            &HandType::OnePair,
            &HandType::HighCard,
        ]
    }
}

impl From<&Vec<&Card>> for HandType {
    fn from(cards: &Vec<&Card>) -> Self {
        let (vals, _) = Hand::count_unique_cards(cards.to_vec());
        match vals {
            1 => Self::FiveOfAKind,
            2 => match cards.iter().filter(|c| c == &&cards[0]).count() {
                1 | 4 => Self::FourOfAKind,
                2 | 3 => Self::FullHouse,
                _ => panic!("No other variation of cards"),
            },
            3 => match cards.iter().filter(|c| c == &&cards[0]).count() {
                1 => match cards.iter().filter(|c| c == &&cards[1]).count() {
                    3 => Self::ThreeOfAKind,
                    2 => Self::TwoPair,
                    1 => match cards.iter().filter(|c| c == &&cards[2]).count() {
                        3 => Self::ThreeOfAKind,
                        2 => Self::TwoPair,
                        _ => panic!("No other variation of cards"),
                    },
                    _ => panic!("No other variation of cards"),
                },
                3 => Self::ThreeOfAKind,
                2 => Self::TwoPair,
                _ => panic!("No other variation of cards"),
            },
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("No other variation of cards"),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Number(10)),
            num => {
                if let Some(n) = num.to_digit(9).map(|o| o as u8) {
                    if n >= 2 && n <= 9 {
                        Ok(Self::Number(n))
                    } else {
                        Err(format!("Number: {} outside range", n).into())
                    }
                } else {
                    Err(format!("Char: {} not parsable to digit ", num).into())
                }
            }
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let heirarchy = Self::heirarchy();
        if let (Card::Number(s), Card::Number(o)) = (&self, &other) {
            return Some(s.cmp(&o));
        }
        match (
            heirarchy.iter().position(|c| c == &self),
            heirarchy.iter().position(|c| c == &other),
        ) {
            (Some(n), Some(o)) => {
                // println!("{:?} POS: {}\n{:?} POS: {}", self, n, other, o);
                Some(o.cmp(&n))
            }
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (None, None) => None,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let heirarchy = Self::heirarchy();
        match (
            heirarchy.iter().position(|c| c == &self),
            heirarchy.iter().position(|c| c == &other),
        ) {
            (Some(n), Some(o)) => {
                // println!("{:?} POS: {}\n{:?} POS: {}", self, n, other, o);
                Some(o.cmp(&n))
            }
            (_, _) => None,
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let parsed = value.trim().split_once(' ').unwrap();
        let cards = parsed
            .0
            .trim()
            .chars()
            .filter_map(|c| Card::try_from(c).ok())
            .collect::<Vec<Card>>();
        let bid = parsed.1.trim().parse().unwrap();
        let h_type = HandType::from(&cards.iter().collect());
        let (_, ordered_uniques) = Hand::count_unique_cards(cards.iter().collect());
        Hand {
            cards,
            h_type,
            ordered_uniques,
            bid,
        }
    }
}

impl Hand {
    fn count_unique_cards<'a>(cards: Vec<&'a Card>) -> (usize, Vec<Card>) {
        let mut unique_values: HashSet<_> = cards.iter().cloned().collect();
        let amt = unique_values.len();
        let mut uniques: Vec<Card> = unique_values.drain().map(|c| c.to_owned()).collect();
        uniques.sort_by(|a, b| {
            let amt_a = cards.iter().filter(|c| c == &&a).count();
            let amt_b = cards.iter().filter(|c| c == &&b).count();
            let cmp = amt_b.partial_cmp(&amt_a).unwrap();
            if cmp == Ordering::Equal {
                b.partial_cmp(&a).unwrap()
            } else {
                cmp
            }
        });
        (amt, uniques)
    }
}

fn get_hands_from_input(input: &str) -> Hands {
    let hands = input.lines().fold(vec![], |mut hands, l| {
        hands.push(Hand::from(l));
        hands
    });
    hands
}

fn rank_hands(hands: &mut Hands) {
    hands.sort_by(|a, b| match a.h_type.partial_cmp(&b.h_type).unwrap() {
        Ordering::Equal => {
            println!("Comparing {:?} & {:?}", a, b);
            for (i, u) in a.ordered_uniques.iter().enumerate() {
                let bu = &b.ordered_uniques[i];
                println!("COMPARING CARDS: {:?} & {:?}", u, bu);
                let cmp = u.partial_cmp(&bu).unwrap();
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        }
        other => other,
    })
}

fn total_winnings(mut hands: Hands) -> u64 {
    rank_hands(&mut hands);
    hands
        .iter()
        .enumerate()
        .fold(0, |mut winnings, (mut i, h)| {
            i += 1;
            println!("{} * {}", i, h.bid);
            winnings += h.bid * i as u64;
            winnings
        })
}

#[cfg(test)]
mod tests {
    use crate::{get_hands_from_input, rank_hands, total_winnings, Card, HandType};

    #[test]
    fn correct_hand_types_from_pt1_example() {
        let input = "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483";
        let mut hands = get_hands_from_input(input);
        // println!("HANDS: {:?}", hands);
        let types: Vec<&HandType> = hands.iter().map(|h| &h.h_type).collect();
        assert_eq!(
            vec![
                &HandType::OnePair,
                &HandType::ThreeOfAKind,
                &HandType::TwoPair,
                &HandType::TwoPair,
                &HandType::ThreeOfAKind
            ],
            types
        );
        rank_hands(&mut hands);
        println!("{:?}", hands);
        assert_eq!(6440, total_winnings(hands));
    }

    #[test]
    fn test_ordering() {
        assert!(Card::Ace > Card::Number(10));
        assert!(Card::King > Card::Number(7));
        assert!(Card::Number(10) > Card::Number(7));
        assert!(Card::Jack < Card::Queen);
        assert!(Card::Ace > Card::Jack);
        assert!(HandType::ThreeOfAKind > HandType::OnePair);
    }
}
