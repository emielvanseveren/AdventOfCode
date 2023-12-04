use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn part_1(input: &str) -> u32 {
    let mut total = 0_u32;

    for l in input.lines() {
        let card = l
            .split(':')
            .nth(1)
            .unwrap()
            .split('|')
            .map(|s| s.split_whitespace().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let cnt = card
            .get(0)
            .unwrap()
            .intersection(card.get(1).unwrap())
            .count();
        if cnt > 0 {
            total += 2_u32.pow(cnt as u32 - 1);
        }
    }
    total
}

fn part_2(input: &str) -> u32 {
    let mut ticket_count: HashMap<usize, usize> = HashMap::new();

    for (card, l) in input.lines().enumerate() {
        *ticket_count.entry(card + 1).or_insert(0) += 1;

        let ticket = l
            .split(':')
            .nth(1)
            .unwrap()
            .split('|')
            .map(|s| s.split_whitespace().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let cnt = ticket
            .get(0)
            .unwrap()
            .intersection(ticket.get(1).unwrap())
            .count();

        for n in 0..cnt {
            let num = *ticket_count.get(&(card + 1)).unwrap();
            *ticket_count.entry(card + 2 + n).or_insert(0) += num;
        }
    }
    ticket_count.values().sum::<usize>() as u32
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(INPUT), 13);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(INPUT), 30);
    }
}
