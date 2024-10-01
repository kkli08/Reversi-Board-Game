use std::io;
use std::io::Write;

fn print_board(board: &Vec<Vec<char>>) {
    let row_0 = String::from("  abcdefgh");
    println!("{}", row_0);
    for (i, row) in board.iter().enumerate() {
        let current_row = ('a' as u8 + i as u8) as char;
        print!("{} ", current_row);

        for cell in row {
            print!("{}", cell);
        }
        print!("\n");
    }
}

fn init_board(board: &mut Vec<Vec<char>>, rows: usize, cols: usize) {
    *board = vec![vec!['.'; cols]; rows];
    board[3][3] = 'W';
    board[4][4] = 'W';
    board[3][4] = 'B';
    board[4][3] = 'B';
}

fn check_input(input: &str, board: &Vec<Vec<char>>) -> bool {
    if input.len() != 2 {
        return false;
    }

    let chars: Vec<char> = input.chars().collect();

    if !(chars[0] >= 'a' && chars[0] <= 'h') || !(chars[1] >= 'a' && chars[1] <= 'h') {
        return false;
    }

    let row_idx = chars[0] as usize - 'a' as usize;
    let col_idx = chars[1] as usize - 'a' as usize;

    // println!("Current cell is : {}", board[row_idx][col_idx]);
    if board[row_idx][col_idx] != '.' {
        false
    } else {
        true
    }

}
fn main() {
    // board instance
    let mut board: Vec<Vec<char>> = Vec::new();
    init_board(&mut board, 8, 8);

    print_board(&board);


    // game loop
    loop {
        let mut user_input = String::new();
        let input_row_idx;
        let input_col_idx;
        // loop for check user input
        loop {
            print!("Enter move for color _ (RowCol): ");
            io::stdout().flush().unwrap();

            // read input
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {
                    let input = user_input.trim();

                    if check_input(input, &board) {
                        let chars: Vec<char> = input.chars().collect();
                        input_row_idx = chars[0] as usize - 'a' as usize;
                        input_col_idx = chars[1] as usize - 'a' as usize;
                        println!("Current cell is : {}", board[input_row_idx][input_col_idx]);
                        break;
                    } else {
                        println!("Invalid move. Try again.");
                        print_board(&board);
                        user_input.clear();
                    }
                }
                Err(_) => {
                    println!("Invalid move. Try again.");
                    print_board(&board);
                    user_input.clear();
                }
            }
        }

        // check flip
        // let mut flip: Vec<Vec<u8>> = vec![vec![0; 8]; 8];

    }




}
