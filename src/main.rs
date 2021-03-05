use sudoku_solver::{backtrack, print_board};
mod tests;

fn main() {
    let mut board = [
        [6, 1, 3, 8, 0, 0, 0, 9, 0],
        [0, 0, 7, 6, 4, 0, 8, 0, 0],
        [9, 0, 0, 5, 3, 1, 0, 0, 7],
        [8, 3, 0, 0, 9, 6, 0, 0, 0],
        [0, 9, 6, 7, 0, 0, 2, 4, 0],
        [7, 4, 0, 0, 1, 0, 0, 6, 9],
        [0, 0, 8, 0, 6, 0, 4, 0, 1],
        [4, 2, 0, 0, 8, 0, 0, 3, 0],
        [0, 6, 1, 0, 0, 0, 0, 0, 0],
    ];

    print_board(&board);
    println!("\n\n\n");
    backtrack(&mut board);
    print_board(&board);

    // testing
    // tests::tests::sudoku_solving();
}
