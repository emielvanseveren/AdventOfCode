use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn power_consumption(codes: Vec<String>) -> i32 {
    let code_length = codes.first().unwrap().len();
    let mut counts = vec![0i32; code_length];

    for code in codes.iter() {
        for (i, c) in code.chars().enumerate() {
            let p = match c {
                '1' => 1,
                _ => 0,
            };
            counts[i] += p;
        }
    }

    let gamma_str = counts
        .iter()
        .map(|i| *i > (codes.len() / 2) as i32)
        .map(|b| if b { 1 } else { 0 })
        .map(|i| i.to_string())
        .collect::<String>();

    let gamma = i32::from_str_radix(&gamma_str, 2).unwrap();
    println!("Gamma str={}, int={}", gamma_str, gamma);

    let epsilon_str = counts
        .iter()
        .map(|i| *i < (codes.len() / 2) as i32)
        .map(|b| if b { 1 } else { 0 })
        .map(|i| i.to_string())
        .collect::<String>();

    let epsilon = i32::from_str_radix(&epsilon_str, 2).unwrap();
    println!("Epsilon str={}, int={}", epsilon_str, epsilon);

    gamma * epsilon
}

fn life_support_rate(codes: Vec<String>) -> i32 {
    let co2_rating = rating(codes.clone(), '0').unwrap();
    let oxygen_rating = rating(codes, '1').unwrap();

    oxygen_rating * co2_rating
}

fn rating(mut codes: Vec<String>, bit: char) -> Option<i32> {
    use std::cmp::Ordering;
    let code_length = codes.first().unwrap().len();

    for i in 0..code_length {
        let count = codes.len() as i32;
        let bit_1_count = codes
            .iter()
            .filter_map(|line| line.chars().nth(i))
            .filter(|c| *c == bit)
            .count() as i32;

        let bit_0_count = count - bit_1_count;
        let bit_mask = match bit_1_count.cmp(&bit_0_count) {
            Ordering::Less => '0',
            Ordering::Equal => bit,
            Ordering::Greater => '1',
        };

        codes = codes
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == bit_mask)
            .collect::<Vec<String>>();

        if codes.len() == 1 {
            return Some(i32::from_str_radix(&codes[0].clone(), 2).unwrap());
        }
    }
    None
}

fn main() {
    let f = File::open("input").expect("File not found");
    let buf_reader = BufReader::new(f);

    let codes = buf_reader
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .collect::<Vec<String>>();

    println!("Power consumption: {}", power_consumption(codes.clone()));
    println!("Live support rate: {}", life_support_rate(codes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_consumption() {
        let codes: Vec<String> = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        assert_eq!(power_consumption(codes), 198);
    }

    #[test]
    fn test_life_support_rate() {
        let codes: Vec<String> = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        assert_eq!(life_support_rate(codes), 230);
    }
}
