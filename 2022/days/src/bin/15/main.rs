use regex::Regex;
use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::convert::identity;
use std::fs::read_to_string;
use std::ops::Range;

type Point = (isize, isize);

struct Intervals(Vec<Range<isize>>);
impl Intervals {
    fn new() -> Self {
        Intervals(Vec::new())
    }
    fn add(&mut self, range: Range<isize>) {
        // remove overlapping intervals
        let i = self
            .0
            .binary_search_by_key(&range.start, |range| range.end)
            .unwrap_or_else(identity);
        let j = self
            .0
            .binary_search_by_key(&range.end, |range| range.start)
            .unwrap_or_else(identity);
        let range = if i < j {
            min(range.start, self.0[i].start)..max(range.end, self.0[j - 1].end)
        } else {
            range
        };
        // update range
        self.0.splice(i..j, [range]);
    }
}

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    let re = Regex::new(
        r"Sensor at x=([-|\d]+), y=([-|\d]+): closest beacon is at x=([-|\d]+), y=([-|\d]+)",
    )
    .unwrap();

    re.captures_iter(input)
        .map(|caps| {
            (
                (
                    caps.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                ),
                (
                    caps.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                    caps.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                ),
            )
        })
        .collect()
}

fn part_1(input: &str, y: isize) -> usize {
    // calculate diamond area of each sensor.
    // line X intersect with x amount of diamond areas => positions beacon cannot be present.
    let (intervals, filtered_beacons) = parse_input(input).iter().fold(
        (Intervals::new(), BTreeSet::new()),
        |(mut acc, mut set), (sensor, beacon)| {
            // manhattan distance - distance to intersection line
            let dx = (beacon.0.abs_diff(sensor.0) + beacon.1.abs_diff(sensor.1)) as isize
                - y.abs_diff(sensor.1) as isize;

            if dx >= 0 {
                acc.add(sensor.0 - dx..sensor.0 + dx + 1);
            }

            // count beacons on intersection line
            if beacon.1 == y {
                set.insert(beacon.0);
            }

            (acc, set)
        },
    );

    intervals
        .0
        .into_iter()
        .map(|range| (range.end - range.start) as usize)
        .sum::<usize>()
        - filtered_beacons.len()
}

fn part_2(input: &str, size: isize) -> Option<u64> {
    let data = parse_input(input);
    (0..=size).find_map(|y| {
        data.iter()
            .fold(Intervals::new(), |mut acc, (sensor, beacon)| {
                let dx = (beacon.0.abs_diff(sensor.0) + beacon.1.abs_diff(sensor.1)) as isize
                    - y.abs_diff(sensor.1) as isize;

                let lo = max(0, sensor.0 - dx);
                let hi = min(size, sensor.0 + dx);
                if lo <= hi {
                    acc.add(lo..hi + 1);
                }
                acc
            })
            .0
            .into_iter()
            .chain([size + 1..size + 1])
            .scan(0, |acc, range| {
                let x = Some(*acc).filter(|x| x < &range.start);
                *acc = range.end;
                Some(x)
            })
            .find_map(|x| x.map(|x| 4000000 * x as u64 + y as u64))
    })
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_1(&input, 2000000));
    println!("part 2: {}", part_2(&input, 4000000).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part_1_example() {
        assert_eq!(26, part_1(INPUT, 10))
    }
    #[test]
    fn part_2_example() {
        assert_eq!(Some(56000011), part_2(INPUT, 20))
    }
}
