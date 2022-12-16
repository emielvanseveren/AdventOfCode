use std::{fs::read_to_string, num::ParseIntError, str::FromStr};

struct TreeGrid {
    rows: usize,
    cols: usize,
    trees: Vec<Vec<(u8, bool)>>,
}

impl FromStr for TreeGrid {
    type Err = ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // we increase all trees so the range is from 1 to 10
        let trees = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| (ch.to_digit(10).unwrap() + 1) as u8)
                    .map(|d| (d, false))
                    .collect::<Vec<(u8, bool)>>()
            })
            .collect::<Vec<_>>();

        let (cols, rows) = (trees[0].len(), trees.len());
        Ok(Self { trees, cols, rows })
    }
}

impl TreeGrid {
    // from north
    fn set_visible_n(&mut self) {
        for c in 0..self.cols {
            let mut heighest = 0;
            for r in 0..self.rows {
                if self.trees[r][c].0 > heighest {
                    heighest = self.trees[r][c].0;
                    self.trees[r][c].1 = true;
                }
            }
        }
    }

    // from south
    fn set_visible_s(&mut self) {
        for c in 0..self.cols {
            let mut heighest = 0;
            for r in (0..self.rows).rev() {
                if self.trees[r][c].0 > heighest {
                    heighest = self.trees[r][c].0;
                    self.trees[r][c].1 = true;
                }
            }
        }
    }

    // from east
    fn set_visible_e(&mut self) {
        for r in 0..self.rows {
            let mut heighest = 0;
            for c in (0..self.cols).rev() {
                if self.trees[r][c].0 > heighest {
                    heighest = self.trees[r][c].0;
                    self.trees[r][c].1 = true;
                }
            }
        }
    }

    // from west
    fn set_visible_w(&mut self) {
        for r in 0..self.rows {
            let mut heighest = 0;
            for c in 0..self.cols {
                if self.trees[r][c].0 > heighest {
                    heighest = self.trees[r][c].0;
                    self.trees[r][c].1 = true;
                }
            }
        }
    }

    fn count_visible(&self) -> u32 {
        self.trees.iter().fold(0, |acc, row| {
            row.iter()
                .fold(acc, |a, (_, visible)| if *visible { a + 1 } else { a })
        })
    }

    fn get_score(
        &self,
        row: usize,
        col: usize,
        row_step: Box<dyn Fn(usize) -> usize>,
        col_step: Box<dyn Fn(usize) -> usize>,
    ) -> u32 {
        let mut trees: Vec<u32> = Vec::new();
        let starting_tree = self.trees[row][col];
        let mut current_tree: u32;
        let mut row_index = row;
        let mut col_index = col;

        loop {
            if (col_step(1) == 1 && (row_index == 0 || row_index >= self.trees.len() - 1))
                || (row_step(1) == 1 && (col_index == 0 || col_index >= self.trees[0].len() - 1))
            {
                break;
            }

            row_index = row_step(row_index);
            col_index = col_step(col_index);
            current_tree = self.trees[row_index][col_index].0 as u32;
            trees.push(current_tree);

            if current_tree >= starting_tree.0 as u32 {
                break;
            }
        }

        trees.len().try_into().unwrap()
    }

    fn get_max_scenic(&self) -> u32 {
        self.trees
            .iter()
            .enumerate()
            .fold(0, |row_acc, (row, row_list)| {
                let row_score = row_list.iter().enumerate().fold(0, |col_acc, (col, _)| {
                    let col_score = self.get_score(row, col, Box::new(|r| r), Box::new(|c| c - 1))
                        * self.get_score(row, col, Box::new(|r| r), Box::new(|c| c + 1))
                        * self.get_score(row, col, Box::new(|r| r - 1), Box::new(|c| c))
                        * self.get_score(row, col, Box::new(|r| r + 1), Box::new(|c| c));

                    // inner fold
                    if col_score > col_acc {
                        col_score
                    } else {
                        col_acc
                    }
                });

                // outer fold
                if row_score > row_acc {
                    row_score
                } else {
                    row_acc
                }
            })
    }
}

fn part_1(input: &str) -> u32 {
    let mut grid: TreeGrid = TreeGrid::from_str(input).unwrap();
    grid.set_visible_n();
    grid.set_visible_e();
    grid.set_visible_s();
    grid.set_visible_w();
    grid.count_visible()
}

fn part_2(input: &str) -> u32 {
    let grid: TreeGrid = TreeGrid::from_str(input).unwrap();
    grid.get_max_scenic()
}

fn main() {
    let input = read_to_string("input.txt").expect("file to exist");
    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part_1_example() {
        assert_eq!(21, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(8, part_2(INPUT));
    }
}
