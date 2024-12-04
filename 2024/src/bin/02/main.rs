use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Report(Vec<u8>);

impl Report {
    fn is_adjacent_safe(l1: u8, l2: u8) -> bool {
        (1..=3).contains(&l1.abs_diff(l2))
    }
    pub fn is_safe(&self) -> bool {
        let mut iter = self.0.windows(2);

        let mut direction = None;

        iter.all(|window| {
            let (l1, l2) = (window[0], window[1]);
            if !Report::is_adjacent_safe(l1, l2) {
                return false;
            }

            if let Some(is_increasing) = direction {
                if is_increasing != (l1 < l2) {
                    return false;
                }
            } else {
                direction = Some(l1 < l2)
            }
            true
        })
    }

    pub fn is_safe_with_one_removal(&self) -> bool {
        self.is_safe()
            || (0..self.0.len()).any(|i| {
                let mut tmp = self.0.clone();
                tmp.remove(i);
                Report(tmp).is_safe()
            })
    }
}

fn parse_data<R: BufRead>(reader: R) -> Result<Vec<Report>, anyhow::Error> {
    reader
        .lines()
        .map(|line| {
            let line = line?;
            let levels = line
                .split_whitespace()
                .map(str::parse::<u8>)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Report(levels))
        })
        .collect()
}

fn part_one(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.is_safe()).count()
}

fn part_two(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|r| r.is_safe_with_one_removal())
        .count()
}

fn main() -> Result<(), anyhow::Error> {
    let input = BufReader::new(File::open("./input.txt")?);
    let data = parse_data(input)?;
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));

    Ok(())
}

#[test]
fn example_1() {
    let report = Report(vec![7, 6, 4, 2, 1]);
    assert!(report.is_safe());
    assert!(report.is_safe_with_one_removal());
}
#[test]
fn example_2() {
    let report = Report(vec![1, 2, 7, 8, 9]);
    assert!(!report.is_safe());
    assert!(!report.is_safe_with_one_removal());
}
#[test]
fn example_3() {
    let report = Report(vec![9, 7, 6, 2, 1]);
    assert!(!report.is_safe());
    assert!(!report.is_safe_with_one_removal());
}
#[test]
fn example_4() {
    let report = Report(vec![1, 3, 2, 4, 5]);
    assert!(!report.is_safe());
    assert!(report.is_safe_with_one_removal());
}
#[test]
fn example_5() {
    let report = Report(vec![8, 6, 4, 4, 1]);
    assert!(!report.is_safe());
    assert!(report.is_safe_with_one_removal());
}
#[test]
fn example_6() {
    let report = Report(vec![1, 3, 6, 7, 9]);
    assert!(report.is_safe());
    assert!(report.is_safe_with_one_removal());
}
