fn main() {
    // let mut board: [[u8; 9]; 9] = [
    //     [5, 3, 0, 0, 7, 0, 0, 0, 0],
    //     [6, 0, 0, 1, 9, 5, 0, 0, 0],
    //     [0, 9, 8, 0, 0, 0, 0, 6, 0],
    //     [8, 0, 0, 0, 6, 0, 0, 0, 3],
    //     [4, 0, 0, 8, 0, 3, 0, 0, 1],
    //     [7, 0, 0, 0, 2, 0, 0, 0, 6],
    //     [0, 6, 0, 0, 0, 0, 2, 8, 0],
    //     [0, 0, 0, 4, 1, 9, 0, 0, 5],
    //     [0, 0, 0, 0, 8, 0, 0, 7, 9],
    // ];
    let mut board: [[u8; 9]; 9] = [
        [0, 0, 0, 0, 1, 4, 0, 0, 0],
        [3, 0, 0, 0, 0, 9, 0, 0, 0],
        [5, 0, 0, 6, 7, 0, 9, 2, 0],
        [0, 0, 7, 0, 2, 6, 0, 0, 4],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 0, 9, 8, 0, 3, 0, 0],
        [0, 8, 4, 0, 9, 1, 0, 0, 6],
        [0, 0, 0, 7, 0, 0, 0, 0, 1],
        [0, 0, 0, 5, 4, 0, 0, 0, 0],
    ];

    println!("Sudoku iniziale:");
    print_board(&board);

    if solve_sudoku_backtracking(&mut board) {
        println!("Sudoku risolto:");
        print_board(&board);
    } else {
        println!("Impossibile risolvere il Sudoku.");
    }
}

fn solve_sudoku_backtracking(board: &mut [[u8; 9]; 9]) -> bool {
    let (row, col) = match find_empty(board) {
        Some(pos) => pos,
        None => return true,
    };

    for num in 1..=9 {
        if is_valid(board, num, row, col) {
            board[row][col] = num; // se trova un numero valido, lo inserisce nella cella
            println!(
                "\n inserito il numero {} nella cella ({}, {})\n",
                num, row, col
            );
            print_board(board);
            if solve_sudoku_backtracking(board) {
                println!("risolto 😊");
                return true;
            }
            println!("soluzione non valida, riprovo con un altro numero");
            board[row][col] = 0;
        }
    }

    false
}

fn find_empty(board: &[[u8; 9]; 9]) -> Option<(usize, usize)> {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                return Some((row, col));
            }
        }
    }
    None
}

fn is_valid(board: &[[u8; 9]; 9], num: u8, row: usize, col: usize) -> bool {
    for i in 0..9 {
        // Check row and column simultaneously
        let val_row = board[row][i];
        let val_col = board[i][col];
        // if board[row][i] == num || board[i][col] == num {
        if val_row == num || val_col == num {
            return false;
        }
    }
    // se il numero non è presente nella riga e nella colonna, controlla il box
    let box_row = row / 3 * 3; // riga della cella in alto a sinistra del box
    let box_col = col / 3 * 3; // colonna della cella in alto a sinistra del box
    for i in 0..3 {
        for j in 0..3 {
            let cell = board[box_row + i][box_col + j];
            // if board[box_row + i][box_col + j] == num {
            if cell == num {
                return false;
            }
        }
    }

    true
}

fn print_board(board: &[[u8; 9]; 9]) {
    for (i, row) in board.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!("------+-------+------");
        }
        for (j, &num) in row.iter().enumerate() {
            if j % 3 == 0 && j != 0 {
                print!("| ");
            }
            if num == 0 {
                print!("_ ");
            } else {
                print!("{} ", num);
            }
        }
        println!();
    }
}

// TODO usa algoritmo di riduzione
