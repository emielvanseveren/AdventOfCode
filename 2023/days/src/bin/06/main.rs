use std::fs::read_to_string;
use std::{iter::zip, ops::BitXor};

fn get_possible_ways_to_win(time: usize, distance: usize) -> usize {
    // kwadratische vergelijking afstand = 1/2 X versnelling X tijd ^2
    // Versnelling is lineair dus 1 millimeter per milliseconde kwadraat
    // 4* distance zorgt dat we rekening houden met het bestaande record.
    let d = time * time - 4 * distance;
    let sqrt_d = (d as f64).sqrt() as usize;

    // perfect square
    if sqrt_d * sqrt_d == d {
        // er is een afstand die specific overeenkomt met het record
        sqrt_d - 1
    } else {
        // rekening houden met even oneven van time en sqrt_d
        sqrt_d + 1 - (time & 1).bitxor(sqrt_d & 1)
    }
}

fn part_1(input: &str) -> usize {
    let values: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    zip(&values[0], &values[1])
        .map(|(time, distance)| get_possible_ways_to_win(*time, *distance))
        .collect::<Vec<usize>>()
        .into_iter()
        .product()
}

fn part_2(input: &str) -> usize {
    let values: Vec<usize> = input
        .lines()
        .map(|l| {
            let mut s = l.split_once(':').unwrap().1.to_owned();
            s.retain(|s| !s.is_whitespace());
            s.parse::<usize>().unwrap()
        })
        .collect();

    let (time, distance) = (values[0], values[1]);
    get_possible_ways_to_win(time, distance)
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(part_1(INPUT), 288);
    }

    #[test]
    fn part_2_example() {
        const INPUT: &str = "Time:      71530
Distance:  940200";
        assert_eq!(part_1(INPUT), 71503);
    }
}
