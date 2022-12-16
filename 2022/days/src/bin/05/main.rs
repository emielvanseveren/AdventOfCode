use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::read_to_string;

type Crate = char;
type Stack = VecDeque<Crate>;
type Stacks = [Stack; STACKS];
const STACKS: usize = 9;

fn part_1(input: &str) -> String {
    // only option is Default::default for this??
    let mut stacks: [Stack; STACKS] = Default::default();

    let (build, moves) = input.split_once("\n\n").unwrap();

    build
        .lines()
        .rev()
        .skip(1) // removes line with stack number
        .for_each(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .for_each(|(i, c)| stacks[i].push_back(c))
        });

    // CrateMover9000
    moves.lines().for_each(|mv| {
        let (amount, from, to): (usize, usize, usize) = mv
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        // todo: we might be able to use drain here, however we have to watch out for changing the
        // order of the crates
        for _ in 0..amount {
            // move crate
            let _crate = stacks[from - 1].pop_back().unwrap();
            stacks[to - 1].push_back(_crate);
        }
    });

    stacks.iter().fold(String::new(), |mut acc, stack| {
        // only add new letter in case the stack has an element
        if let Some(c) = stack.back() {
            acc.push(*c);
        }
        acc
    })
}

fn part_2(input: &str) -> String {
    // only option is Default::default for this??
    let mut stacks: Stacks = Default::default();

    let (build, moves) = input.split_once("\n\n").unwrap();

    build
        .lines()
        .rev()
        .skip(1) // removes line with stack number
        .for_each(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .for_each(|(i, c)| stacks[i].push_back(c))
        });

    // CrateMover9001
    moves.lines().for_each(|mv| {
        let (amount, from, to): (usize, usize, usize) = mv
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let len = stacks[from - 1].len();
        let crates = stacks[from - 1].drain(len - amount..len).collect_vec();
        for c in crates.iter() {
            stacks[to - 1].push_back(*c);
        }
    });

    // get last item from each stack
    stacks.iter().fold(String::new(), |mut acc, stack| {
        // only add new letter in case the stack has an element
        if let Some(c) = stack.back() {
            acc.push(*c);
        }
        acc
    })
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_1_example() {
        assert_eq!(String::from("CMZ"), part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(String::from("MCD"), part_2(INPUT));
    }
}
