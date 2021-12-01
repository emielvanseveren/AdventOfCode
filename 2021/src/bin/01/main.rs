use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn number_of_increases(measurements: Vec<u16>) -> u16 {
    let mut count = 0;
    for i in 1..measurements.len() {
        if measurements[i - 1] < measurements[i] {
            count += 1;
        }
    }
    count
}

fn number_of_increases_sliding_window(measurements: Vec<u16>) -> u16 {
    let mut count = 0;

    for i in 1..measurements.len() - 2 {
        let current_sum = measurements[i - 1] + measurements[i] + measurements[i + 1];
        let next_sum = measurements[i] + measurements[i + 1] + measurements[i + 2];

        if current_sum < next_sum {
            count += 1;
        }
    }
    count
}

fn main() {
    // read file to vector
    let file = File::open("input").expect("file not found");
    let buf = BufReader::new(file);

    let numbers: Vec<u16> = buf
        .lines()
        .map(|line| line.unwrap().parse::<u16>().unwrap())
        .collect();

    println!(
        "Number of increases: {}",
        number_of_increases(numbers.clone())
    );
    println!(
        "Number of increases sliding window: {}",
        number_of_increases_sliding_window(numbers)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_increases() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(number_of_increases(input), 7);
    }

    #[test]
    fn test_number_of_increases_sliding_window() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(number_of_increases_sliding_window(input), 5);
    }
}
