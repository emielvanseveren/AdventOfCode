use std::fs::read_to_string;

enum Match {
    Lose,
    Tie,
    Win,
}

impl std::convert::From<u8> for Match {
    fn from(value: u8) -> Self {
        match value {
            b'X' => Self::Lose,
            b'Y' => Self::Tie,
            b'Z' => Self::Win,
            _ => panic!("unknown options {}", value as char),
        }
    }
}

impl std::convert::Into<u8> for Match {
    fn into(self) -> u8 {
        match self {
            Match::Lose => 0,
            Match::Tie => 3,
            Match::Win => 6,
        }
    }
}

enum RPC {
    Rock,
    Paper,
    Scissors,
}

impl std::convert::From<u8> for RPC {
    fn from(value: u8) -> Self {
        match value {
            b'A' | b'X' => RPC::Rock,
            b'B' | b'Y' => RPC::Paper,
            b'C' | b'Z' => RPC::Scissors,
            _ => panic!("unknown options {}", value as char),
        }
    }
}

impl std::convert::Into<u8> for RPC {
    fn into(self) -> u8 {
        match self {
            RPC::Rock => 1,
            RPC::Paper => 2,
            RPC::Scissors => 3,
        }
    }
}

fn part_1(input: String) -> u32 {
    input
        .lines()
        .map(|l| {
            let opponent = l.as_bytes()[0].into();
            let me = l.as_bytes()[2].into();

            let match_result: u8 = match (&me, opponent) {
                (RPC::Rock, RPC::Rock)
                | (RPC::Paper, RPC::Paper)
                | (RPC::Scissors, RPC::Scissors) => Match::Tie,
                (RPC::Rock, RPC::Paper) => Match::Win,
                (RPC::Rock, RPC::Scissors) => Match::Lose,
                (RPC::Paper, RPC::Scissors) => Match::Win,
                (RPC::Paper, RPC::Rock) => Match::Lose,
                (RPC::Scissors, RPC::Rock) => Match::Win,
                (RPC::Scissors, RPC::Paper) => Match::Lose,
            }
            .into();

            let me_score: u8 = me.into();
            (match_result + me_score) as u32
        })
        .sum()
}

fn part_2(input: String) -> u32 {
    input
        .lines()
        .map(|l| {
            let opponent = l.as_bytes()[0].into();
            let result = l.as_bytes()[2].into();

            let me: u8 = match (&opponent, &result) {
                (RPC::Rock, Match::Tie)
                | (RPC::Paper, Match::Tie)
                | (RPC::Scissors, Match::Tie) => opponent,
                (RPC::Rock, Match::Win) => RPC::Paper,
                (RPC::Rock, Match::Lose) => RPC::Scissors,
                (RPC::Paper, Match::Win) => RPC::Scissors,
                (RPC::Paper, Match::Lose) => RPC::Rock,
                (RPC::Scissors, Match::Win) => RPC::Rock,
                (RPC::Scissors, Match::Lose) => RPC::Paper,
            }
            .into();

            let result_num: u8 = result.into();
            (result_num + me) as u32
        })
        .sum()
}

fn main() {
    let input = read_to_string("input.txt").expect("File to exist");
    println!("{}", part_1(input.clone()));
    println!("{}", part_2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let str = r#"A Y
B X
C Z"#;
        assert_eq!(part_1(str.to_string()), 15);
    }

    #[test]
    fn part_2_example() {
        let str = r#"A Y
B X
C Z"#;

        assert_eq!(part_2(str.to_string()), 12);
    }
}
