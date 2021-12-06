use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn populate(lanternfish: &mut Vec<i8>, days: u32) {
    for _ in 0..days {
        for j in 0..lanternfish.len() {
            if lanternfish[j] == 0 {
                lanternfish.push(8);
                lanternfish[j] = 7;
            }
            lanternfish[j] -= 1;
        }
    }
}

fn main() {
    let f = File::open("input").expect("File not found");
    let input = BufReader::new(f);

    let mut fish = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split(',')
                .map(|s| s.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .flatten()
        .collect::<Vec<i8>>();

    populate(&mut fish, 80);
    println!("Amount of fish: {}", fish.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_populate() {
        let mut fish = vec![3, 4, 3, 1, 2];
        populate(&mut fish, 12);
        assert_eq!(
            fish,
            vec![5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8]
        );

        let mut fish = vec![3, 4, 3, 1, 2];
        populate(&mut fish, 256);
        assert_eq!(fish.len(), 26984457539);
    }
}
