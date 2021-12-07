// Naive method
// exponential in space and time
fn populate(lantern_fish: &mut Vec<usize>, days: u32) {
    for _ in 0..days {
        for j in 0..lantern_fish.len() {
            if lantern_fish[j] == 0 {
                lantern_fish.push(8);
                lantern_fish[j] = 7;
            }
            lantern_fish[j] -= 1;
        }
    }
}

// Naive method recursive implementation
fn populate_recursive(lantern_fish: &mut Vec<usize>, days: u32) {
    if days == 0 {
        return;
    }
    for j in 0..lantern_fish.len() {
        if lantern_fish[j] == 0 {
            lantern_fish.push(8);
            lantern_fish[j] = 7;
        }
        lantern_fish[j] -= 1;
    }
    populate_recursive(lantern_fish, days - 1);
}

// constant in space, O(days) in time
fn populate_size(lantern_fish: Vec<usize>, days: u16) -> usize {
    let max_day: usize = 9;
    let mut counter = vec![0; max_day];
    for fish in lantern_fish {
        counter[fish] += 1;
    }
    for d in 0..days {
        counter[(7 + d as usize) % max_day] += counter[d as usize % max_day];
    }
    counter.iter().sum::<usize>()
}

fn main() {
    let mut fish: Vec<usize> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    let fish_clone = fish.clone();

    populate(&mut fish, 80);
    println!("Part 1: {}", fish.len());

    let size = populate_size(fish_clone, 256);
    println!("Part 2: {}", size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_population() {
        let mut fish = vec![3, 4, 3, 1, 2];
        populate(&mut fish, 12);
        assert_eq!(
            fish,
            vec![5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8]
        );
    }
    #[test]
    fn test_small_population_recursive() {
        let mut fish = vec![3, 4, 3, 1, 2];
        populate_recursive(&mut fish, 12);
        assert_eq!(
            fish,
            vec![5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8]
        );
    }
    #[test]
    fn test_large_population() {
        let fish = vec![3, 4, 3, 1, 2];
        assert_eq!(populate_size(fish, 256), 26984457539);
    }
}
