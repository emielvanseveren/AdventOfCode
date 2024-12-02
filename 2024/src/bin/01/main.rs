use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

fn parse_data<R: BufRead>(reader: R) -> Result<(Vec<i32>, Vec<i32>), anyhow::Error> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        left.push(row[0]);
        right.push(row[1]);
    }
    left.sort_unstable();
    right.sort_unstable();
    Ok((left, right))
}
fn part_one(left: &[i32], right: &[i32]) -> Result<usize, anyhow::Error> {
    let total = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| (r - l).abs())
        .sum::<i32>();
    Ok(total as usize)
}
fn part_two(left: &[i32], right: &[i32]) -> Result<usize, anyhow::Error> {
    let total = left
        .iter()
        .map(|&l| right.iter().filter(|&r| *r == l).count() * (l as usize))
        .sum();
    Ok(total)
}

fn main() -> Result<(), anyhow::Error> {
    let input = BufReader::new(File::open("./input.txt")?);
    let (left, right) = parse_data(input)?;
    println!("part one: {}", part_one(&left, &right)?);
    println!("part two: {}", part_two(&left, &right)?);
    Ok(())
}

#[test]
fn test_part_one() {
    let (left, right) = parse_data(BufReader::new(TEST_INPUT.as_bytes())).unwrap();
    assert_eq!(part_one(&left, &right).unwrap(), 11);
}
#[test]
fn test_part_two() {
    let (left, right) = parse_data(BufReader::new(TEST_INPUT.as_bytes())).unwrap();
    assert_eq!(part_two(&left, &right).unwrap(), 31);
}
