use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Board = [[(u8, bool); 5]; 5];

fn print_board(board: &Board, show_hidden: bool) {
    println!();
    for row in board {
        for (i, (num, set)) in row.iter().enumerate() {
            print!(
                " {} ",
                if *set || show_hidden {
                    (*num).to_string()
                } else {
                    "#".to_string()
                }
            );
            if i == 4 {
                println!();
            }
        }
    }
}

struct BingoSolver {
    boards: Vec<Board>,
    numbers: Vec<u8>,
}

impl BingoSolver {
    fn new(numbers: Vec<u8>) -> Self {
        Self {
            numbers,
            boards: Vec::new(),
        }
    }

    pub fn get_winning_board(&mut self) -> Option<(u8, Board, usize)> {
        for number in self.numbers.iter() {
            for (i, board) in self.boards.iter_mut().enumerate() {
                if BingoSolver::is_winner(board, *number) {
                    return Some((*number, *board, i));
                }
            }
        }
        None
    }
    pub fn get_losing_board(&mut self) -> Option<(u8, Board)> {
        loop {
            let (final_number, board, index) = self.get_winning_board().unwrap();
            self.boards.remove(index);
            if self.boards.is_empty() {
                return Some((final_number, board));
            }
        }
    }

    fn is_winner(board: &mut Board, number: u8) -> bool {
        for row in 0..5 {
            for col in 0..5 {
                if board[row][col].0 == number {
                    board[row][col].1 = true;

                    if board[row][0].1
                        && board[row][1].1
                        && board[row][2].1
                        && board[row][3].1
                        && board[row][4].1
                    {
                        return true;
                    }
                    if board[0][col].1
                        && board[1][col].1
                        && board[2][col].1
                        && board[3][col].1
                        && board[4][col].1
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn load_boards(&mut self, path: &str) {
        let file = File::open(path).expect("File to open");
        let buf_reader = BufReader::new(file);

        let mut board: Board = [[(0, false); 5]; 5];
        let mut i = 0;

        for line in buf_reader.lines() {
            let line = line.expect("Line to read");

            if line.is_empty() {
                self.boards.push(board);
                i = 0;
                continue;
            }

            let trimmed_line = line.trim().replace("  ", " ");
            let numbers = trimmed_line
                .split(' ')
                .map(|str_number| str_number.parse::<u8>().expect("Should be a number"));

            for (j, number) in numbers.enumerate() {
                board[i % 5][j].0 = number;
            }
            i += 1;
        }
    }
}

fn calculate_final_score(last_number: u8, board: &Board) -> u64 {
    let mut sum: u64 = 0;

    for row in board {
        for (num, set) in row.iter() {
            if !(*set) {
                sum += *num as u64;
            }
        }
    }

    sum as u64 * last_number as u64
}

fn main() {
    let numbers = vec![
        27, 14, 70, 7, 85, 66, 65, 57, 68, 23, 33, 78, 4, 84, 25, 18, 43, 71, 76, 61, 34, 82, 93,
        74, 26, 15, 83, 64, 2, 35, 19, 97, 32, 47, 6, 51, 99, 20, 77, 75, 56, 73, 80, 86, 55, 36,
        13, 95, 52, 63, 79, 72, 9, 10, 16, 8, 69, 11, 50, 54, 81, 22, 45, 1, 12, 88, 44, 17, 62, 0,
        96, 94, 31, 90, 39, 92, 37, 40, 5, 98, 24, 38, 46, 21, 30, 49, 41, 87, 91, 60, 48, 29, 59,
        89, 3, 42, 58, 53, 67, 28,
    ];
    let mut bingo = BingoSolver::new(numbers.clone());
    bingo.load_boards("input");
    let (last_number, board, _) = bingo
        .get_winning_board()
        .expect("Should have a winning board");

    println!("Winning: {}", calculate_final_score(last_number, &board));

    let mut bingo = BingoSolver::new(numbers);
    bingo.load_boards("input");
    let (last_number, board) = bingo
        .get_losing_board()
        .expect("Should have a losing board");
    println!("losing: {}", calculate_final_score(last_number, &board));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning_board() {
        let numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let mut bingo = BingoSolver::new(numbers);
        bingo.load_boards("src/bin/04/testinput");
        let (last_number, board, _) = bingo.get_winning_board().unwrap();
        assert_eq!(calculate_final_score(last_number, &board), 4512);
    }
    #[test]
    fn test_losing_board() {
        let numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let mut bingo = BingoSolver::new(numbers);
        bingo.load_boards("src/bin/04/testinput");
        let (last_number, board) = bingo.get_losing_board().unwrap();
        assert_eq!(calculate_final_score(last_number, &board), 1924);
    }
}
