use std::fs::read_to_string;

enum Overlap {
    None,
    Overlap,
    Contained,
}

type Boundary = (u32, u32);

fn is_overlapping(a: Boundary, b: Boundary) -> Overlap {
    if a.1 < b.0 || a.0 > b.1 {
        return Overlap::None;
    }
    if a.0 >= b.0 && a.1 <= b.1 {
        return Overlap::Contained;
    }
    if b.0 >= a.0 && b.1 <= a.1 {
        return Overlap::Contained;
    }
    Overlap::Overlap
}

fn to_boundary(s: &str) -> Boundary {
    match s
        .split('-')
        .filter_map(|p| p.parse().ok())
        .take(2)
        .collect::<Vec<_>>()[..]
    {
        [a, b] if b >= a => (a, b),
        [a, b] if a > b => (b, a),
        _ => (0, 0),
    }
}

fn part_1(input: &str) -> usize {
    let mut counter = 0;
    for line in input.lines() {
        if let Some(p) = line.split_once(',') {
            let b1 = to_boundary(p.0);
            let b2 = to_boundary(p.1);

            if let Overlap::Contained = is_overlapping(b1, b2) {
                counter += 1;
            }
        }
    }
    counter
}

fn part_2(input: &str) -> usize {
    let mut counter = 0;

    for line in input.lines() {
        if let Some(p) = line.split_once(",") {
            let a = to_boundary(p.0);
            let b = to_boundary(p.1);

            match is_overlapping(a, b) {
                Overlap::Contained => counter += 1,
                Overlap::Overlap => counter += 1,
                _ => (),
            }
        }
    }
    counter
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(INPUT), 2)
    }

    #[test]
    fn part_2_example() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(part_2(INPUT), 4)
    }
}
