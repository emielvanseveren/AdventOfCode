use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|i| i.trim().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect()
}

fn solve(nums: &[isize]) -> isize {
    let mut ans = *nums.last().unwrap();
    let mut diffs: Vec<_> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    ans += diffs.last().unwrap();

    while diffs.windows(2).any(|w| w[0] != w[1]) {
        for i in 0..diffs.len() - 1 {
            diffs[i] = diffs[i + 1] - diffs[i];
        }
        diffs.pop();
        ans += diffs.last().unwrap();
    }
    ans
}

fn part_1(nums: Vec<Vec<isize>>) -> isize {
    nums.iter().map(|nums| solve(nums)).sum()
}

fn part_2(nums: Vec<Vec<isize>>) -> isize {
    nums.iter()
        .map(|nums| solve(&nums.iter().rev().copied().collect::<Vec<_>>()))
        .sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part_1(parse_input(&input)));
    println!("Part 2: {}", part_2(parse_input(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(parse_input(INPUT)), 114);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(parse_input(INPUT)), 2);
    }
}
