use std::fs::read_to_string;

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find_map(|ch| ch.to_digit(10))
                .unwrap_or_else(|| panic!("No first number found in line: \"{line}\""));

            let last = line
                .chars()
                .rev()
                .find_map(|ch| ch.to_digit(10))
                .unwrap_or_else(|| panic!("No last number found in line: \"{line}\""));
            first * 10 + last
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    use std::collections::HashMap;

    let number_str_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    input
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|i| {
                    number_str_map
                        .keys()
                        .find(|key| line[i..].starts_with(*key))
                })
                .unwrap_or_else(|| panic!("No first number found in line: \"{line}\""));

            let last = (0..line.len())
                .find_map(|i| {
                    number_str_map
                        .keys()
                        .find(|key| line[..(line.len() - i)].ends_with(*key))
                })
                .unwrap_or_else(|| panic!("No last number found in line: \"{line}\""));

            number_str_map[first] * 10 + number_str_map[last]
        })
        .sum()
}

fn main() {
    let input = read_to_string("input.txt").expect("File to exist");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part_1(input), 142);
    }

    #[test]
    fn part_2_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part_2(input), 281);
    }
}
