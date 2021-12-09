use std::collections::BinaryHeap;
use std::fs;

fn parse_input(filename: &str) -> Vec<Vec<u8>> {
    let input: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .map(|mut v| {
            v.iter_mut().for_each(|x| *x -= b'0');
            v
        })
        .collect()
}

fn star_one(input: &[Vec<u8>]) -> u32 {
    let mut sum = 0;

    for row_index in 0..input.len() {
        for col_index in 0..input[row_index].len() {
            let cell = input[row_index][col_index];

            // if cell is not on the left edge.
            // Check if the cell to the left is larger.
            if col_index > 0 && cell >= input[row_index][col_index - 1] {
                continue;
            }

            // If cell is not on the right edge.
            // Check if the cell to the right is larger.
            if col_index < input[row_index].len() - 1 && cell >= input[row_index][col_index + 1] {
                continue;
            }

            // If the cell is not on the top edge.
            // check the cell above is larger.
            if row_index > 0 && cell >= input[row_index - 1][col_index] {
                continue;
            }

            // If the cell is not on the bottom edge.
            // Check if the cell below is larger
            if row_index < input.len() - 1 && cell >= input[row_index + 1][col_index] {
                continue;
            }

            // If none of the neighbours are larger, add the cell to the sum.
            sum += (cell + 1) as u32;
        }
    }
    sum
}

fn star_two(input: &[Vec<u8>]) -> u32 {
    let mut input = input.to_vec();
    let mut heap = BinaryHeap::new();

    for row_index in 0..input.len() {
        for col_index in 0..input[row_index].len() {
            let cell = input[row_index][col_index];

            // same idea as star_one()

            if col_index > 0 && cell >= input[row_index][col_index - 1] {
                continue;
            }

            if col_index < input[row_index].len() - 1 && cell >= input[row_index][col_index + 1] {
                continue;
            }

            if row_index > 0 && cell >= input[row_index - 1][col_index] {
                continue;
            }

            if row_index < input.len() - 1 && cell >= input[row_index + 1][col_index] {
                continue;
            }

            let size = get_basin_size(&mut input, row_index, col_index);
            heap.push(size);
        }
    }
    heap.pop().unwrap() * heap.pop().unwrap() * heap.pop().unwrap()
}

fn get_basin_size(grid: &mut [Vec<u8>], row_index: usize, col_index: usize) -> u32 {
    let mut size = 1;
    grid[row_index][col_index] = u8::MAX;

    //top
    if row_index > 0 && grid[row_index - 1][col_index] < 9 {
        size += get_basin_size(grid, row_index - 1, col_index);
    }

    //left
    if col_index > 0 && grid[row_index][col_index - 1] < 9 {
        size += get_basin_size(grid, row_index, col_index - 1);
    }

    //right
    if col_index < grid[row_index].len() - 1 && grid[row_index][col_index + 1] < 9 {
        size += get_basin_size(grid, row_index, col_index + 1);
    }

    //bottom
    if row_index < grid.len() - 1 && grid[row_index + 1][col_index] < 9 {
        size += get_basin_size(grid, row_index + 1, col_index);
    }
    size
}

fn main() {
    let input = parse_input("input.txt");
    println!("Star one: {}", star_one(&input));

    let input = parse_input("input.txt");
    println!("Star two: {}", star_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        let input = parse_input("testinput.txt");
        assert_eq!(star_one(&input), 15);
    }

    #[test]
    fn test_star_two() {
        let input = parse_input("testinput.txt");
        assert_eq!(star_two(&input), 1134);
    }
}
