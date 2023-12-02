use std::fs::read_to_string;

fn part_1(input: &str) -> u32 {
    let max_cubes = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect::<std::collections::HashMap<_, _>>();

    input
        .lines()
        .filter_map(|game| {
            let (id_part, sets_part) = game.split_once(": ").unwrap_or_default();
            let game_id: u32 = id_part
                .split_whitespace()
                .nth(1)
                .and_then(|id| id.parse().ok())
                .unwrap_or_default();
            let is_valid_game = sets_part.split("; ").all(|set| {
                set.split(", ").all(|cubes| {
                    let parts = cubes.split_whitespace().collect::<Vec<_>>();
                    let count = parts
                        .first()
                        .and_then(|&c| c.parse::<u32>().ok())
                        .unwrap_or_default();
                    let color = parts.get(1).copied().unwrap_or_default();
                    count <= *max_cubes.get(color).unwrap_or(&0)
                })
            });

            if is_valid_game {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let mut min_cubes = [("red", 0), ("green", 0), ("blue", 0)]
                .iter()
                .copied()
                .collect::<std::collections::HashMap<_, _>>();

            game.split(": ")
                .nth(1)
                .unwrap_or_default()
                .split("; ")
                .for_each(|set| {
                    set.split(", ").for_each(|cubes| {
                        let parts = cubes.split_whitespace().collect::<Vec<_>>();
                        if let (Some(&count_str), Some(&color)) = (parts.first(), parts.get(1)) {
                            if let Ok(count) = count_str.parse::<u32>() {
                                *min_cubes.entry(color).or_insert(0) = count.max(min_cubes[color]);
                            }
                        }
                    })
                });
            min_cubes.values().product::<u32>()
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

    static input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input), 8);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input), 2286);
    }
}
