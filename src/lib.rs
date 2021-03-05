use std::io;
use std::io::Write;

pub fn backtrack(board: &mut [[i32; 9]; 9]) -> bool {
    match find_empty(&board) {
        Some((y, x)) => {
            let row = y;
            let col = x;
            for num in 1..10 {
                if is_valid(&board, num, (row, col)) {
                    board[row][col] = num;

                    if backtrack(board) {
                        return true;
                    }

                    board[row][col] = 0;
                }
            }
        }
        None => return true,
    }

    return false;
}

fn is_valid(board: &[[i32; 9]; 9], val: i32, coords: (usize, usize)) -> bool {
    for col in 0..board[0].len() {
        if board[coords.0][col] == val && coords.1 != col {
            return false;
        }
    }
    for row in 0..board.len() {
        if board[row][coords.1] == val && coords.0 != row {
            return false;
        }
    }
    let box_x = coords.1 / 3;
    let box_y = coords.0 / 3;

    for row in box_y * 3..box_y * 3 + 3 {
        for col in box_x * 3..box_x * 3 + 3 {
            if board[row][col] == val && (row, col) != coords {
                return false;
            }
        }
    }

    return true;
}

pub fn print_board(board: &[[i32; 9]; 9]) {
    for row in 0..board.len() {
        if row % 3 == 0 && row != 0 {
            println!("{}", "-".repeat(26));
        }

        for col in 0..board[0].len() {
            if col % 3 == 0 && col != 0 {
                print!(" |  ");
                io::stdout().flush().unwrap();
            }
            if col == 8 {
                println!("{}", board[row][col]);
            } else {
                print!("{} ", board[row][col]);
                io::stdout().flush().unwrap();
            }
        }
    }
}

fn find_empty(board: &[[i32; 9]; 9]) -> Option<(usize, usize)> {
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if board[row][col] == 0 {
                return Some((row, col));
            }
        }
    }

    return None;
}
