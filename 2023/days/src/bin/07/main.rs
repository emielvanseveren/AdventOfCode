use bstr::ByteSlice;
use itertools::Itertools;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Hand {
    kind: HandType,
    cards: [u8; 5],
}

fn part_1(input: &[u8]) -> u64 {
    input
        .lines()
        .map(|line| {
            let (hand_str, bid) = line.split_once_str(" ").unwrap();
            let bid = unsafe { bid.to_str_unchecked() }.parse::<u64>().unwrap();

            let cards_in_hand = TryInto::<[u8; 5]>::try_into(hand_str)
                .unwrap()
                .map(|c| match c {
                    b'2'..=b'9' => c - b'0',
                    b'T' => 10,
                    b'J' => 11,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    _ => unreachable!(),
                });

            let frequency: Vec<usize> = cards_in_hand
                .iter()
                .sorted_unstable()
                .group_by(|&&c| c)
                .into_iter()
                .map(|(_c, g)| g.count())
                .sorted_unstable()
                .collect();

            // by sorting we know the order e.g. [1,4] and not [4,1]
            let hand_type = match frequency.as_slice() {
                [5] => HandType::FiveOfAKind,
                [1, 4] => HandType::FourOfAKind,
                [2, 3] => HandType::FullHouse,
                [1, 1, 3] => HandType::ThreeOfAKind,
                [1, 2, 2] => HandType::TwoPair,
                [1, 1, 1, 2] => HandType::OnePair,
                _ => HandType::HighCard,
            };

            let hand = Hand {
                cards: cards_in_hand,
                kind: hand_type,
            };

            (hand, bid)
        })
        .sorted_unstable()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum()
}

fn part_2(input: &[u8]) -> u64 {
    input
        .lines()
        .map(|line| {
            let (hand_str, bid) = line.split_once_str(" ").unwrap();
            let bid = unsafe { bid.to_str_unchecked() }.parse::<u64>().unwrap();

            let cards_in_hand = TryInto::<[u8; 5]>::try_into(hand_str)
                .unwrap()
                .map(|c| match c {
                    b'J' => 0,
                    b'2'..=b'9' => c - b'0',
                    b'T' => 10,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    _ => unreachable!(),
                });

            let mut joker_count = 0;
            let mut frequency = cards_in_hand
                .iter()
                .sorted_unstable()
                .group_by(|&&c| c)
                .into_iter()
                .filter_map(|(c, g)| {
                    if c == 0 {
                        joker_count = g.count();
                        None
                    } else {
                        Some(g.count())
                    }
                })
                .sorted_unstable()
                .collect_vec();

            match frequency.last_mut() {
                Some(f) => *f += joker_count,
                None => frequency.push(joker_count),
            }

            let hand_type = match frequency.as_slice() {
                [5] => HandType::FiveOfAKind,
                [1, 4] => HandType::FourOfAKind,
                [2, 3] => HandType::FullHouse,
                [1, 1, 3] => HandType::ThreeOfAKind,
                [1, 2, 2] => HandType::TwoPair,
                [1, 1, 1, 2] => HandType::OnePair,
                _ => HandType::HighCard,
            };

            let hand = Hand {
                cards: cards_in_hand,
                kind: hand_type,
            };

            (hand, bid)
        })
        .sorted_unstable()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum()
}
fn main() {
    let input = include_bytes!("input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = b"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(INPUT), 6440);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(INPUT), 5905);
    }
}
