fn star_one(positions: &mut Vec<i32>) -> i32 {
    positions.sort_unstable();
    let mid = positions[positions.len() / 2];
    let mut fuel: i32 = 0;

    for position in positions.iter() {
        fuel += (mid - position).abs();
    }
    fuel
}

fn star_two(positions: &mut Vec<i32>) -> i32 {
    use std::ops::RangeInclusive;
    let mean = positions.iter().sum::<i32>() / positions.len() as i32;

    RangeInclusive::new(-1, 1)
        .map(|d| {
            positions
                .iter()
                .map(|p| {
                    let n = (p - mean + d).abs();
                    // n=4  (4+3+2+1) is equivalent to the binomial of (n+1) over 2
                    n * (n + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn main() {
    let mut positions: Vec<_> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|i| i.parse::<i32>().expect("to be number"))
        .collect();

    let required_fuel = star_one(&mut positions);
    println!("Required fuel: {}", required_fuel);

    let required_fuel = star_two(&mut positions);
    println!("Required fuel: {}", required_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        let mut positions: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(star_one(&mut positions), 37);
    }
    #[test]
    fn test_star_two() {
        let mut positions: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(star_two(&mut positions), 168);
    }
}
