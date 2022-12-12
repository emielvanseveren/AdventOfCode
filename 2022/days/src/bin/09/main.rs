use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Motion {
    Left,
    Right,
    Up,
    Down,
}

type Location = (i32, i32);

fn follow(head: &Location, tail: &Location) -> Option<Location> {
    let dx = tail.0 - head.0;
    let dy = tail.1 - head.1;

    if (dx == 2 || dx == -2) && (dy == 2 || dy == -2) {
        return Some((head.0 + dx.clamp(-1, 1), head.1 + dy.clamp(-1, 1)));
    } else if dx == 2 || dx == -2 {
        return Some((head.0 + dx.clamp(-1, 1), head.1));
    } else if dy == 2 || dy == -2 {
        return Some((head.0, head.1 + dy.clamp(-1, 1)));
    }

    None
}

fn parse_input(input: &str) -> Vec<Motion> {
    input
        .lines()
        .flat_map(|l| {
            let (dir, steps) = l.split_once(' ').unwrap();
            let amount = str::parse::<usize>(steps).unwrap();

            match dir {
                "L" => std::iter::repeat(Motion::Left).take(amount),
                "R" => std::iter::repeat(Motion::Right).take(amount),
                "U" => std::iter::repeat(Motion::Up).take(amount),
                "D" => std::iter::repeat(Motion::Down).take(amount),
                _ => panic!("unknown motion direction"),
            }
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let motions = parse_input(input);
    let mut visited: HashSet<Location> = HashSet::new();

    // Add start position as visited location
    visited.insert((0, 0));

    motions
        .iter()
        .fold(((0, 0), (0, 0)), |mut head_tail, motion| {
            match motion {
                Motion::Up => {
                    head_tail.0 .1 += 1;
                }
                Motion::Right => {
                    head_tail.0 .0 += 1;
                }
                Motion::Down => {
                    head_tail.0 .1 -= 1;
                }
                Motion::Left => {
                    head_tail.0 .0 -= 1;
                }
            };

            if let Some(new_tail) = follow(&head_tail.0, &head_tail.1) {
                visited.insert(new_tail);
                head_tail.1 = new_tail;
            }
            head_tail
        });
    visited.len()
}

fn part_2(input: &str) -> usize {
    let motions = parse_input(input);
    let mut visited: HashSet<Location> = HashSet::new();

    let initial = vec![(0, 0); 10];

    motions.iter().fold(initial, |mut rope, motion| {
        // move head
        match motion {
            Motion::Up => {
                rope[0].1 += 1;
            }
            Motion::Right => {
                rope[0].0 += 1;
            }
            Motion::Down => {
                rope[0].1 -= 1;
            }
            Motion::Left => {
                rope[0].0 -= 1;
            }
        };

        for i in 1..rope.len() {
            if let Some(pos) = follow(&rope[i - 1], &rope[i]) {
                rope[i] = pos
            } else {
                break;
            }
        }
        visited.insert(*rope.last().unwrap());
        rope
    });

    visited.len()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("result p1: {}", part_1(&input));
    println!("result p2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(13, part_1(input));
    }
    #[test]
    fn part_2_example() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(1, part_2(input));
    }

    #[test]
    fn part_2_example_b() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, part_2(input));
    }
}
