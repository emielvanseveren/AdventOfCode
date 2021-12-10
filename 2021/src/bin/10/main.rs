#![feature(bool_to_option)]

fn star_one(input: &str) -> i32 {
    input
        .lines()
        .flat_map(|line| {
            let mut st = Vec::new();
            line.chars().find_map(|c| match c {
                ')' => st.pop().and_then(|c| (c != '(').then_some(3)),
                ']' => st.pop().and_then(|c| (c != '[').then_some(57)),
                '}' => st.pop().and_then(|c| (c != '{').then_some(1197)),
                '>' => st.pop().and_then(|c| (c != '<').then_some(25137)),
                _ => {
                    st.push(c);
                    None
                }
            })
        })
        .sum()
}

fn star_two(input: &str) -> usize {
    let mut scores: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::new();
            line.chars()
                .all(|c| match c {
                    ')' => stack.pop() == Some('('),
                    ']' => stack.pop() == Some('['),
                    '}' => stack.pop() == Some('{'),
                    '>' => stack.pop() == Some('<'),
                    _ => {
                        stack.push(c);
                        true
                    }
                })
                .then(|| {
                    stack.into_iter().rev().fold(0, |acc, c| {
                        acc * 5
                            + match c {
                                '(' => 1,
                                '[' => 2,
                                '{' => 3,
                                '<' => 4,
                                _ => 0,
                            }
                    })
                })
                .filter(|&x| x > 0)
        })
        .collect();

    let middle = scores.len() / 2;
    *scores.select_nth_unstable(middle).1
}

fn main() {
    let input = include_str!("input.txt").trim();
    let points = star_one(input);
    println!("star one: {}", points);

    let points = star_two(input);
    println!("star two: {}", points);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        let input = include_str!("testinput.txt");
        assert_eq!(star_one(input), 26397);
    }

    #[test]
    fn test_star_two() {
        let input = include_str("testinput.txt");
        assert_eq!(star_two(input), 288957);
    }
}
