fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let mut lines: Vec<Vec<i64>> = input
        .lines()
        .filter_map(|l| {
            if !l.trim().is_empty() {
                Some(
                    l.split_whitespace()
                        .filter_map(|s| s.to_string().parse().ok())
                        .collect(),
                )
            } else {
                None
            }
        })
        .collect();
    println!("{}", sum_of_missing_values(&mut lines))
}

fn get_number_pyramid(nums: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut pyramid: Vec<Vec<i64>> = vec![nums.to_vec()];
    loop {
        let nums = pyramid.last().expect("Couldn't get last element");
        let next_level = nums.windows(2).fold(vec![], |mut acc, w| {
            acc.push((w[0] - w[1]).abs());
            acc
        });
        let sum: i64 = next_level.iter().sum();
        // println!("{}", sum);
        pyramid.push(next_level);
        if sum == 0 {
            break;
        }
    }
    pyramid
}

fn fill_missing(vecs: &mut Vec<Vec<i64>>) {
    let mut last_vec = vec![];
    vecs.iter_mut()
        .rev()
        .enumerate()
        .map(|(i, v)| {
            if i == 0 {
                v.push(0);
            } else {
                v.push(v.last().unwrap() + last_vec.last().unwrap());
            }
            last_vec = v.clone();
        })
        .collect()
}

fn sum_of_missing_values(vecs: &mut Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for v in vecs {
        let mut py = get_number_pyramid(&v);
        println!("BEFORE MISSING: ");
        py.iter().for_each(|v| {
            println!("{:?}", v);
        });
        fill_missing(&mut py);
        println!("AFTER MISSING: ");
        py.iter().for_each(|v| {
            println!("{:?}", v);
        });
        sum += py[0].last().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::sum_of_missing_values;

    #[test]
    fn example_pt1() {
        let input = "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
            ";
        let mut lines: Vec<Vec<i64>> = input
            .lines()
            .filter_map(|l| {
                if !l.trim().is_empty() {
                    Some(
                        l.split_whitespace()
                            .filter_map(|s| s.to_string().parse().ok())
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .collect();
        let sum = sum_of_missing_values(&mut lines);
        assert_eq!(114, sum);
    }
}
