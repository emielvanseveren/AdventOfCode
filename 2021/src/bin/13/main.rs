#![feature(hash_drain_filter)]

use std::collections::HashSet;
use std::fs;

type Point = (i32, i32);
type Fold = (i32, bool);

fn parse_input(filename: &str) -> (HashSet<Point>, Vec<Fold>) {
    let input = fs::read_to_string(filename).unwrap();
    let parts = input.split_once("\n\n").unwrap();

    let points = parts
        .0
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|e| (e.0.parse().unwrap(), e.1.parse().unwrap()))
        .collect();

    let folds = parts
        .1
        .lines()
        .map(|line| line.split_once('=').unwrap())
        .map(|e| (e.1.parse().unwrap(), e.0.contains('x')))
        .collect();
    (points, folds)
}

fn execute_fold(points: &mut HashSet<Point>, fold: Fold) {
    if fold.1 {
        let folded = points
            .drain_filter(|e| e.0 > fold.0)
            .map(|e| (e.0 - (e.0 - fold.0) * 2, e.1))
            .collect::<Vec<_>>();
        points.extend(folded.iter());
    } else {
        let folded = points
            .drain_filter(|e| e.1 > fold.0)
            .map(|e| (e.0, e.1 - (e.1 - fold.0) * 2))
            .collect::<Vec<_>>();
        points.extend(folded.iter());
    }
}

fn star_two(points: &mut HashSet<Point>, folds: &[Fold]) -> usize {
    folds.iter().for_each(|f| {
        execute_fold(points, *f);
    });
    points.len()
}

fn main() {
    let (mut points, folds) = parse_input("input.txt");
    execute_fold(&mut points, *folds.get(0).unwrap());
    println!("points after initial fold: {}", points.len());

    let (mut points, folds) = parse_input("input.txt");
    star_two(&mut points, &folds);
    print_grid(&points);
}

fn print_grid(points: &HashSet<Point>) {
    for y in 0..6 {
        for x in 0..40 {
            print!("{}", if points.contains(&(x, y)) { "#" } else { "." });
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_fold() {
        let (mut points, folds) = parse_input("testinput.txt");
        execute_fold(&mut points, *folds.get(0).unwrap());
        assert_eq!(points.len(), 17);
    }
}
