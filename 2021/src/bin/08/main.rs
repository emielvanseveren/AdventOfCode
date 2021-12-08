use std::collections::HashSet;

fn decode(patterns: &[HashSet<char>], digit: &HashSet<char>) -> i32 {
    if digit.len() == patterns[0].len() {
        return 1;
    }
    if digit.len() == patterns[1].len() {
        return 7;
    }
    if digit.len() == patterns[2].len() {
        return 4;
    }
    if digit.len() == patterns[9].len() {
        return 8;
    }

    if digit.len() == 5 {
        if patterns[0].difference(digit).count() == 0 {
            return 3;
        } else if patterns[2].difference(digit).count() == 1 {
            return 5;
        } else {
            return 2;
        }
    } else {
        if patterns[0].difference(digit).count() > 0 {
            return 6;
        } else if patterns[2].difference(digit).count() > 0 {
            return 0;
        } else {
            return 9;
        }
    }
}

fn star_one(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let (_, raw_digits) = line.split_once(" | ").unwrap();
            let digits: Vec<_> = raw_digits.trim().split_whitespace().collect();
            digits
                .iter()
                .filter(|digit| {
                    digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7
                })
                .count()
        })
        .sum()
}

fn star_two(input: String) -> i32 {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let (raw_pattern, raw_digits) = line.split_once(" | ").unwrap();

            let mut patterns: Vec<_> = raw_pattern
                .trim()
                .split_whitespace()
                .map(|d| d.chars().collect::<HashSet<_>>())
                .collect();
            patterns.sort_unstable_by_key(|p| p.len());

            let digits: Vec<_> = raw_digits
                .trim()
                .split_whitespace()
                .map(|d| d.chars().collect::<HashSet<_>>())
                .collect();

            digits
                .iter()
                .map(|digit| decode(&patterns, digit))
                .fold(0, |cum, d| cum * 10 + d)
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", star_one(input.to_string()));

    let input = include_str!("input.txt");
    println!("{}", star_two(input.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        let test_input = include_str!("testinput.txt");
        assert_eq!(star_one(test_input.to_string()), 26);
    }

    #[test]
    fn test_star_two() {
        let test_input = include_str!("testinput.txt");
        assert_eq!(star_two(test_input.to_string()), 61229);
    }
}
