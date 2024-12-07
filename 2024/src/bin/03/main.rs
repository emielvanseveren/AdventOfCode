use regex::Regex;

fn part_one(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|capture| {
            let (_, [n1, n2]) = capture.extract();

            n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)(o)(?:n't)?\(\)").unwrap();
    let mut enabled = true;
    re.captures_iter(input).fold(0, |acc, capture| {
        let (s, [n1, n2]) = capture.extract();
        match s {
            "do()" => {
                enabled = true;
                acc
            }

            "don't()" => {
                enabled = false;
                acc
            }
            _ => acc + ((n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap()) * enabled as u32),
        }
    })
}

fn main() {
    let input = include_str!("./input.txt");

    println!("part one: {}", part_one(input));
    println!("part two: {}", part_two(input));
}

#[test]
fn example_1() {
    let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    assert_eq!(part_one(input), 161);
}

#[test]
fn example_2() {
    let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    assert_eq!(part_two(input), 48);
}
