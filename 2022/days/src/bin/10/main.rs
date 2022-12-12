use std::fs::read_to_string;
use std::num;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    NoOp,
    AddX(i32),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut splits = l.split_whitespace();

            match splits.next().unwrap() {
                "noop" => Instruction::NoOp,
                "addx" => Instruction::AddX(str::parse::<i32>(splits.next().unwrap()).unwrap()),
                _ => panic!("unknown instruction"),
            }
        })
        .collect()
}

fn calc_signal_strength(cycle: i32, reg: i32, signal: &mut i32) {
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        *signal += reg * cycle
    }
}

fn part_1(input: &str) -> i32 {
    let instructions = parse_input(input);

    let mut clock: i32 = 0;
    let mut x: i32 = 1;

    let mut signal = 0;

    for ins in instructions.iter() {
        match ins {
            Instruction::NoOp => {
                clock += 1;
                calc_signal_strength(clock, x, &mut signal);
            }
            Instruction::AddX(v) => {
                clock += 1;

                calc_signal_strength(clock, x, &mut signal);

                clock += 1;

                calc_signal_strength(clock, x, &mut signal);

                x += v;
            }
        }
    }
    signal
}

fn update_register(register: &mut Vec<i32>, delta: i32) {
    let x = register[register.len() - 1];
    register.push(x + delta);
}

fn part_2(input: &str) -> String {
    let mut register = vec![1];
    let instructions = parse_input(input);

    for instruction in instructions {
        match instruction {
            Instruction::NoOp => update_register(&mut register, 0),
            Instruction::AddX(v) => {
                update_register(&mut register, 0);
                update_register(&mut register, v);
            }
        }
    }

    register
        .chunks(40)
        .into_iter()
        .flat_map(|row| {
            row.iter()
                .enumerate()
                .map(|(i, x)| if x.abs_diff(i as i32) <= 1 { '#' } else { ' ' })
                .chain(std::iter::once('\n'))
        })
        .collect::<String>()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    println!("part 1: {}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
        let result = part_1(input);
        assert_eq!(13140, result);
    }
}
