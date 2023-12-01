use std::fs;

enum WordDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl<'a> TryFrom<&'a str> for WordDigit {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "zero" => Ok(Self::Zero),
            "one" => Ok(Self::One),
            "two" => Ok(Self::Two),
            "three" => Ok(Self::Three),
            "four" => Ok(Self::Four),
            "five" => Ok(Self::Five),
            "six" => Ok(Self::Six),
            "seven" => Ok(Self::Seven),
            "eight" => Ok(Self::Eight),
            "nine" => Ok(Self::Nine),
            _ => Err("No matching value".into()),
        }
    }
}

impl Into<u32> for WordDigit {
    fn into(self) -> u32 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let ret = run_input(&input).unwrap();
    let sum: u32 = ret.iter().sum();
    println!("{:?}", sum);
}

fn run_input<'a>(input: &'a str) -> Result<Vec<u32>, Box<(dyn std::error::Error)>> {
    let input_strs: Vec<&str> = input.split('\n').collect();
    let mut return_vec = vec![];

    input_strs.into_iter().for_each(|str| {
        let str = convert_word_digits_in_str(str);
        let tups = get_digits_from_str(&str);
        if let Some(int) = first_and_last_digits_to_int(tups) {
            return_vec.push(int)
        }
    });
    Ok(return_vec)
}

fn convert_word_digits_in_str<'a>(str: &'a str) -> String {
    println!("String before convert: {}", str);
    let str = str.to_lowercase();
    let digit_words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut return_string = str.clone().to_string();

    let mut matches: Vec<(usize, &str)> = vec![];
    for pattern in digit_words.into_iter() {
        str.rmatch_indices(pattern)
            .into_iter()
            .for_each(|m| matches.push(m));
    }
    matches.sort_by(|(ai, _), (bi, _)| bi.cmp(ai));

    matches.into_iter().for_each(|(_, pat)| {
        if return_string.contains(pat) {
            if let Ok(replacement) = WordDigit::try_from(pat) {
                let replacement: u32 = replacement.into();
                return_string = return_string
                    .to_owned()
                    .clone()
                    .replace(pat, &replacement.to_string());
            }
        }
    });
    println!("String after convert: {}", return_string);
    return_string
}

fn get_digits_from_str<'a>(str: &'a str) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];

    str.chars().for_each(|c| {
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap())
        }
    });
    digits
}

fn first_and_last_digits_to_int(vec: Vec<u32>) -> Option<u32> {
    let mut return_str = String::new();
    if let Some(int) = vec.first() {
        return_str.push_str(&int.to_string())
    }
    if vec.len() > 1 {
        return_str.push_str(&vec.last().unwrap().to_string());
    } else {
        return_str = return_str.to_owned() + &return_str;
    }
    println!("Returning: {}", return_str);
    return_str.parse::<u32>().ok()
}

#[cfg(test)]
mod test {
    use crate::run_input;

    #[test]
    fn part_one_test_case() {
        let test_input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let returns = run_input(test_input).unwrap();
        assert_eq!(vec![12, 38, 15, 77], returns);
        assert_eq!(142u32, returns.iter().sum());
    }
    #[test]
    fn part_two_test_case() {
        let test_input = "two1nine\n
            eightwothree\n
            abcone2threexyz\n
            xtwone3four\n
            4nineeightseven2\n
            zoneight234\n
            7pqrstsixteen";

        let returns = run_input(test_input).unwrap();
        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], returns);
        assert_eq!(281u32, returns.iter().sum());
    }
}
