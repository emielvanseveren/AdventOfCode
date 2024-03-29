use anyhow::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

type Point = (i32, i32);
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    /*
    fn is_straight(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }
    */
    fn points(&self) -> impl Iterator<Item = Point> {
        let delta = (
            (self.end.0 - self.start.0).signum(),
            (self.end.1 - self.start.1).signum(),
        );
        let (mut point, end) = (self.start, self.end);
        std::iter::repeat_with(move || {
            let r = point;
            point = (point.0 + delta.0, point.1 + delta.1);
            r
        })
        .take_while(move |p| p != &end)
        .chain(std::iter::once(self.end))
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        }
        let m = LINE_RE
            .captures(s)
            .ok_or_else(|| panic!("Unable to parse"))
            .unwrap();
        Ok(Line {
            start: (m[1].parse()?, m[2].parse()?),
            end: (m[3].parse()?, m[4].parse()?),
        })
    }
}

fn parse(input: &str) -> impl Iterator<Item = Line> + '_ {
    input.lines().map(|l| l.parse().unwrap())
}

fn main() {
    /*
    let input = std::fs::read_to_string("input").unwrap();
    let mut points = HashMap::new();
    parse(&input)
        .filter(Line::is_straight)
        .flat_map(|l| l.points())
        .for_each(|p| *points.entry(p).or_insert(0) += 1);
    println!("{}", points.values().filter(|v| **v > 1).count());
    */

    let input = std::fs::read_to_string("input").unwrap();
    let mut points = HashMap::new();
    parse(&input)
        .flat_map(|l| l.points())
        .for_each(|p| *points.entry(p).or_insert(0) += 1);
    println!("{}", points.values().filter(|v| **v > 1).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
