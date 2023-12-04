fn adjacent(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    (x.saturating_sub(1)..max_x.min(x + 2))
        .flat_map(|i| (y.saturating_sub(1)..max_y.min(y + 2)).map(move |j| (i, j)))
        .filter(|&(i, j)| i != x || j != y)
        .collect()
}

fn adjacent_to_symbol(map: &[Vec<char>], x: usize, y: usize) -> bool {
    adjacent(x, y, map.len(), map[0].len())
        .iter()
        .any(|&(i, j)| {
            !map[i][j].is_ascii_digit() && map[i][j] != '.'
                || (y + 1 == j
                    && x == i
                    && map[i][j].is_ascii_digit()
                    && adjacent_to_symbol(map, i, j))
        })
}

fn whole_number(map: &[Vec<char>], x: usize, y: usize) -> (usize, usize) {
    let num_str: String = map[x][y..]
        .iter()
        .take_while(|&&c| c.is_ascii_digit())
        .collect();
    (num_str.parse().unwrap(), num_str.len())
}

fn number(map: &[Vec<char>], x: usize, y: usize) -> Option<usize> {
    if map[x][y].is_ascii_digit() {
        let mut left = y;
        while left != 0 && map[x][left - 1].is_ascii_digit() {
            left -= 1;
        }
        return Some(whole_number(map, x, left).0);
    }
    None
}

fn adjacent_numbers(map: &[Vec<char>], x: usize, y: usize) -> Vec<usize> {
    let mut result = Vec::new();

    // Check going left.
    if let Some(n) = number(map, x, y - 1) {
        result.push(n);
    }

    // Check going right.
    if let Some(n) = number(map, x, y + 1) {
        result.push(n);
    }

    // Check if the top middle is a digit, if so, the the entire top
    // must be contained in it.
    if x > 0 && map[x - 1][y].is_ascii_digit() {
        if let Some(n) = number(map, x - 1, y) {
            result.push(n);
        }
    } else if x > 0 {
        // Either the left or right could have a number, check both.
        if let Some(n) = number(map, x - 1, y - 1) {
            result.push(n);
        }

        if let Some(n) = number(map, x - 1, y + 1) {
            result.push(n);
        }
    }

    // Check if the bottom middle is a digit, if so, the the entire top
    // must be contained in it.
    if x < map.len() && map[x + 1][y].is_ascii_digit() {
        if let Some(n) = number(map, x + 1, y) {
            result.push(n);
        }
    } else if x < map.len() {
        // Either the left or right could have a number, check both.
        if let Some(n) = number(map, x + 1, y - 1) {
            result.push(n);
        }

        if let Some(n) = number(map, x + 1, y + 1) {
            result.push(n);
        }
    }

    result
}

fn part_1(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    map.iter().enumerate().fold(0, |acc, (x, row)| {
        let mut y = 0;
        let mut inner_acc = acc;
        while y < row.len() {
            if row[y].is_ascii_digit() && adjacent_to_symbol(&map, x, y) {
                let (n, len) = whole_number(&map, x, y);
                inner_acc += n;
                y += len; // Skip the next len - 1 elements
            } else {
                y += 1;
            }
        }
        inner_acc
    })
}

fn part_2(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    map.iter().enumerate().fold(0, |acc, (x, row)| {
        row.iter().enumerate().fold(acc, |inner_acc, (y, &c)| {
            if c == '*' {
                let nn = adjacent_numbers(&map, x, y);
                if nn.len() == 2 {
                    inner_acc + nn[0] * nn[1]
                } else {
                    inner_acc
                }
            } else {
                inner_acc
            }
        })
    })
}

fn main() {
    let input: &str = include_str!("input.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(INPUT), 4361);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(INPUT), 467835);
    }
}
