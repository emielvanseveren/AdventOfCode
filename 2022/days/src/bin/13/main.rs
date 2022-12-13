use std::{cmp::Ordering, fs::read_to_string, num::ParseIntError, str::FromStr};

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Value(i32),
    Packet(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // if comparing 2 values, we can just compare
            (Packet::Value(s), Packet::Value(o)) => s.cmp(o),

            // if comparing a value and a packet,
            (Packet::Value(n), Packet::Packet(_)) => {
                Packet::Packet(vec![Packet::Value(*n)]).cmp(other)
            }

            // if comparing a packet and a value (mind order)
            (Packet::Packet(_), Packet::Value(n)) => {
                self.cmp(&Packet::Packet(vec![Packet::Value(*n)]))
            }

            // if comparing 2 packets (compare elements in packets)
            (Packet::Packet(left), Packet::Packet(right)) => {
                for i in 0..left.len().min(right.len()) {
                    match left[i].cmp(&right[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => {}
                    }
                }
                left.len().cmp(&right.len())
            }
        }
    }
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn process_tokens(tokens: &mut Vec<&str>) -> Result<Vec<Packet>, ParseIntError> {
            let mut result = Vec::new();
            while !tokens.is_empty() {
                match tokens.pop() {
                    Some("]") => {
                        return Ok(result);
                    }
                    Some("[") => {
                        result.push(Packet::Packet(process_tokens(tokens)?));
                    }
                    Some("") => { /* important that empty is not matched with Some(n) */ }

                    Some(n) => {
                        result.push(Packet::Value(n.parse::<i32>()?));
                    }
                    None => unreachable!(),
                };
            }
            Ok(result)
        }

        Ok(Packet::Packet(process_tokens(
            &mut s
                .replace('[', "[,")
                .replace(']', ",]")
                .split(',')
                .rev()
                .collect::<Vec<_>>(),
        )?))
    }
}

fn part_1(input: &str) -> usize {
    input.split("\n\n").enumerate().fold(0, |acc, (idx, pair)| {
        let mut lines = pair.lines();
        let left = lines.next().unwrap().parse::<Packet>().unwrap();
        let right = lines.next().unwrap().parse::<Packet>().unwrap();

        // sum of index of all correctly ordered packets
        if left < right {
            return acc + idx + 1;
        }
        acc
    })
}

fn part_2(input: &str) -> usize {
    let mut all_packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Packet>().unwrap())
        .collect::<Vec<_>>();

    // add divider packets
    let div_2 = "[[2]]".parse::<Packet>().unwrap();
    let div_6 = "[[6]]".parse::<Packet>().unwrap();
    all_packets.push(div_2.clone());
    all_packets.push(div_6.clone());

    // since we have implemented the order trait we can now just sort all packets!!
    all_packets.sort();

    // decoder key = index (not zero based) from 2 divider packets
    let pos_2 = all_packets.iter().position(|p| p == &div_2).unwrap() + 1;
    let pos_6 = all_packets.iter().position(|p| p == &div_6).unwrap() + 1;
    pos_2 * pos_6
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(13, part_1(input));
    }

    #[test]
    fn part_2_example() {
        let input = "[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[[2]]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[[6]]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]";

        assert_eq!(150, part_2(input));
    }
}
