use std::fs;

fn parse_input(filename: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let data = fs::read_to_string(filename)?;
    Ok(data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect())
}

fn neighbours(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    for di in -1..=1 {
        // ignore out-of-bounds rows
        let cur_i = (i as i32) + di;
        if !(0..10).contains(&cur_i) {
            continue;
        }

        for dj in -1..=1 {
            // ignore self
            if di == 0 && dj == 0 {
                continue;
            }

            // ignore out-of-bounds columns
            let cur_j = (j as i32) + dj;
            if !(0..10).contains(&cur_j) {
                continue;
            }

            neighbours.push((cur_i as usize, cur_j as usize));
        }
    }
    neighbours
}

fn step(grid: &mut Vec<Vec<u8>>) -> u32 {
    let mut flashes = 0;

    // add energy and save flashes
    let mut q: Vec<(usize, usize)> = Vec::new();
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col += 1;
            if *col > 9 {
                q.push((i, j));
            }
        }
    }

    // handle flashes
    while !q.is_empty() {
        let (i, j) = q.pop().unwrap();

        grid[i][j] = 0;
        flashes += 1;

        for (ni, nj) in neighbours(i, j) {
            if grid[ni][nj] == 0 || grid[ni][nj] > 9 {
                continue;
            }
            grid[ni][nj] += 1;
            if grid[ni][nj] > 9 {
                q.push((ni, nj));
            }
        }
    }
    flashes
}

fn star_one(grid: &mut Vec<Vec<u8>>, steps: u8) -> u32 {
    let mut total_flashes = 0;

    for _ in 0..steps {
        total_flashes += step(grid);
    }
    total_flashes
}

fn star_two(grid: &mut Vec<Vec<u8>>) -> u64 {
    let mut total_steps = 0;

    loop {
        total_steps += 1;
        if step(grid) == 100 {
            return total_steps;
        }
    }
}

fn main() {
    let mut grid = parse_input("input.txt").unwrap();
    println!("Total flashes: {}", star_one(&mut grid, 100));

    let mut grid = parse_input("input.txt").unwrap();
    println!("All white at step: {}", star_two(&mut grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        let mut grid: Grid = parse_input("testinput.txt").expect("test input to exist");
        assert_eq!(star_one(&mut grid, 100), 1656);
    }
    fn test_star_two() {
        let mut grid: Grid = parse_input("testinput.txt").expect("test input  to exist");
        assert_eq!(star_two(&mut grid), 195);
    }
}
