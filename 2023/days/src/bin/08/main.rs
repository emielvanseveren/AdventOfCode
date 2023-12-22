use num::integer::lcm;
use std::collections::HashMap;
use std::fs::read_to_string;

type Nodes<'a> = HashMap<&'a [u8], (&'a [u8], &'a [u8])>;

fn parse_input(input: &str) -> (&str, Nodes, Vec<&[u8]>) {
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    let mut nodes = Nodes::new();
    let mut starts = Vec::<&[u8]>::new();

    // Format: AAA = (BBB, CCC)
    for line in lines.skip(1) {
        let line = line.as_bytes();
        let node = &line[0..3];
        let dir_l = &line[7..10];
        let dir_r = &line[12..15];
        nodes.insert(node, (dir_l, dir_r));

        if node.ends_with(&[b'A']) {
            starts.push(node);
        }
    }

    (directions, nodes, starts)
}

fn part_1(directions: &str, nodes: &Nodes) -> u64 {
    solve(directions, nodes, &[b'A', b'A', b'A'], |node| {
        node == [b'Z', b'Z', b'Z']
    })
}

fn part_2(directions: &str, nodes: &Nodes, starts: Vec<&[u8]>) -> u64 {
    let end = |node: &[u8]| node.ends_with(&[b'Z']);
    let values: Vec<u64> = starts
        .iter()
        .map(|start| solve(directions, nodes, start, end))
        .collect::<Vec<u64>>();
    lcm_of_vector(&values)
}

fn solve(directions: &str, nodes: &Nodes, start: &[u8], end: fn(&[u8]) -> bool) -> u64 {
    let mut node = start;
    let mut count = 0;

    for &dir in directions.as_bytes().iter().cycle() {
        count += 1;
        let (l, r) = nodes.get(node).unwrap();
        if dir == b'L' {
            node = l;
        } else {
            node = r;
        }

        if end(node) {
            break;
        }
    }

    count
}

// Lowest Common Multiple
fn lcm_of_vector(values: &[u64]) -> u64 {
    let mut result = values[0];
    for &value in values.iter().skip(1) {
        result = lcm(result, value);
    }

    result
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (directions, nodes, starts) = parse_input(&input);
    println!("Part 1: {}", part_1(directions, &nodes));
    println!("Part 2: {}", part_2(directions, &nodes, starts));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let (directions, nodes, _) = parse_input(INPUT);
        assert_eq!(part_1(directions, &nodes), 2);
    }

    #[test]
    fn part_2_example() {
        const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (directions, nodes, starts) = parse_input(INPUT);
        assert_eq!(part_2(directions, &nodes, starts), 6);
    }
}
