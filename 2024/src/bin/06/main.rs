use std::collections::HashSet;

#[derive(Clone)]
struct Lab(Vec<Vec<LabLocationType>>);

impl Lab {
    fn get(&self, x: isize, y: isize) -> Option<LabLocationType> {
        if x < 0 || y < 0 || y as usize >= self.0.len() || x as usize >= self.0[y as usize].len() {
            return None;
        }
        Some(self.0[y as usize][x as usize])
    }

    fn set(&mut self, x: isize, y: isize, location_type: LabLocationType) {
        self.0[y as usize][x as usize] = location_type;
    }
}
#[repr(usize)]
#[derive(Clone, Debug, Copy, PartialEq)]
enum LabLocationType {
    Obstructed = 0,
    Empty = 1,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
struct Guard {
    pub position: (isize, isize),
    pub direction: Direction,
}

impl Guard {
    fn new(start_position: (isize, isize), direction: Direction) -> Self {
        Self {
            position: start_position,
            direction,
        }
    }

    fn next_position(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Right => (self.position.0 + 1, self.position.1),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
        }
    }

    fn step(&mut self, lab: &Lab) -> bool {
        let next_position = self.next_position();

        match lab.get(next_position.0, next_position.1) {
            Some(LabLocationType::Empty) => {
                self.position = next_position;
                true
            }
            Some(LabLocationType::Obstructed) => {
                self.turn_right();
                self.step(lab)
            }
            None => false,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

fn part_two(start_pos: (isize, isize), lab: &Lab) -> usize {
    let mut obstructions = HashSet::new();
    let mut guard = Guard::new(start_pos, Direction::Up);

    loop {
        let next_position = guard.next_position();
        let next_in_bounds = lab.get(next_position.0, next_position.1).is_some();

        if next_in_bounds
            && next_position != start_pos
            && lab.get(next_position.0, next_position.1) == Some(LabLocationType::Empty)
            && !obstructions.contains(&next_position)
        {
            let mut new_lab = lab.clone();
            new_lab.set(
                next_position.0,
                next_position.1,
                LabLocationType::Obstructed,
            );

            if find_loop(guard.position, guard.direction, new_lab) {
                obstructions.insert(next_position);
            }
        }
        if !guard.step(lab) {
            break;
        }
    }

    obstructions.len()
}

fn find_loop(start_pos: (isize, isize), direction: Direction, lab: Lab) -> bool {
    let mut guard = Guard::new(start_pos, direction);
    let mut visited = HashSet::new();

    loop {
        let state = (guard.position, guard.direction);

        if !visited.insert(state) {
            return true;
        }

        if !guard.step(&lab) {
            return false;
        }
    }
}

fn part_one(guard_start_position: (isize, isize), lab: &Lab) -> usize {
    let mut guard = Guard::new(guard_start_position, Direction::Up);
    let mut visited = HashSet::new();
    visited.insert(guard_start_position);

    loop {
        if guard.step(lab) {
            visited.insert(guard.position);
        } else {
            break;
        }
    }
    visited.len()
}

fn parse_data(input: &str) -> ((isize, isize), Lab) {
    let mut start_position = (0, 0);
    let lab = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => LabLocationType::Empty,
                    '#' => LabLocationType::Obstructed,
                    '^' => {
                        start_position = (x as isize, y as isize);
                        LabLocationType::Empty
                    }
                    _ => unreachable!("unknown lab location type"),
                })
                .collect::<Vec<LabLocationType>>()
        })
        .collect();

    (start_position, Lab(lab))
}

fn main() -> Result<(), anyhow::Error> {
    let input = include_str!("./input.txt");
    let (guard_start_position, lab) = parse_data(input);
    println!("part one: {}", part_one(guard_start_position, &lab));
    println!("part two: {}", part_two(guard_start_position, &lab));
    Ok(())
}

const TEST_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

#[test]
fn example_one() {
    let (guard_start_position, lab) = parse_data(TEST_INPUT);
    assert_eq!(part_one(guard_start_position, &lab), 41);
}

#[test]
fn example_two() {
    let (guard_start_position, lab) = parse_data(TEST_INPUT);
    assert_eq!(part_two(guard_start_position, &lab), 6);
}
