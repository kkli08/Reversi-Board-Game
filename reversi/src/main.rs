use std::io;
use std::io::Write;

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

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

fn opposite_color_to_char(color: Color) -> char {
    match color {
        Color::Black => 'W',
        Color::White => 'B',
    }
}
fn check_flip(board: &mut Vec<Vec<char>>, row_idx: usize, col_idx: usize, current_color: Color) {
    let current_char = board[row_idx][col_idx];
    let opponent_char = opposite_color_to_char(current_color);

    let directions: [(i32, i32); 8] = [
        (-1, 0), // up
        (1, 0), // down
        (0, -1), // left
        (0, 1), // right
        (-1, -1), // upper-left
        (-1, 1), // upper-right
        (1, -1), // lower-left
        (1, 1), // lower-right
    ];

    for &(dx, dy) in &directions {
        let mut x = row_idx as i32 + dx;
        let mut y = col_idx as i32 + dy;
        let mut has_opponent = false;

        let mut position_to_flip = Vec::new();

        while x >= 0 && x < board[0].len() as i32 && y >= 0 && y < board.len() as i32 {
            let (ux, uy) = (x as usize, y as usize);

            if board[ux][uy] == '.' {
                break;
            } else if board[ux][uy] == opponent_char {
                has_opponent = true;
                position_to_flip.push((x as usize, y as usize));
            } else {
                // same color
                if has_opponent {
                    for &(fx, fy) in &position_to_flip {
                        board[fx][fy] = current_char;
                    }
                    break;
                }
            }

            x += dx;
            y += dy;
        }
    }

}

fn check_flip_rule(board: &Vec<Vec<char>>, row_idx: usize, col_idx: usize, current_color: Color) -> bool{
    let opponent_char = opposite_color_to_char(current_color);

    let directions: [(i32, i32); 8] = [
        (-1, -1), // upper-left
        (-1, 1), // upper-right
        (1, -1), // lower-left
        (1, 1), // lower-right
        (-1, 0), // up
        (1, 0), // down
        (0, -1), // left
        (0, 1), // right
    ];

    for &(dx, dy) in &directions {
        let mut x = row_idx as i32 + dx;
        let mut y = col_idx as i32 + dy;
        let mut has_opponent = false;

        while x >= 0 && x < board[0].len() as i32 && y >= 0 && y < board.len() as i32 {
            let (ux, uy) = (x as usize, y as usize);

            if board[ux][uy] == '.' {
                break;
            } else if board[ux][uy] == opponent_char {
                has_opponent = true;
            } else {
                // same color
                if has_opponent {
                    return true;
                }
            }
            x += dx;
            y += dy;
        }
    }
    false
}

fn read_input(board: &mut Vec<Vec<char>>, current_color: Color) -> (usize, usize) {
    let mut user_input = String::new();
    let mut input_row_idx;
    let mut input_col_idx;
    // loop for check user input
    loop {
        print!("Enter move for color {} (RowCol): ", match current_color {
            Color::Black => "B",
            Color::White => "W",
        });
        // io::stdout().flush().unwrap();
        io::stdout().flush().expect("Failed to flush stdout.");

        // read input
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                let input = user_input.trim();

                if check_input(input, &board) {
                    let chars: Vec<char> = input.chars().collect();
                    input_row_idx = chars[0] as usize - 'a' as usize;
                    input_col_idx = chars[1] as usize - 'a' as usize;

                    // println!("Current cell is : {}", board[input_row_idx][input_col_idx]);

                    // check flip rule
                    if check_flip_rule(&board, input_row_idx, input_col_idx, current_color) {
                        return (input_row_idx, input_col_idx);
                    } else {
                        // flip-rule not meet
                        println!("Invalid move. Try again.");
                        print_board(&board);
                        user_input.clear();
                    }

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
}

fn check_availability(board: &Vec<Vec<char>>, current_color: Color) -> bool {
    for row_idx in 0..board.len() {
        for col_idx in 0..board[row_idx].len() {
            if board[row_idx][col_idx] == '.' && check_flip_rule(&board, row_idx, col_idx, current_color) {
                return true;
            }
        }
    }

    false
}

fn game_over(board: &Vec<Vec<char>>) {
    let mut black_cnt = 0;
    let mut white_cnt = 0;

    for row_idx in 0..board.len() {
        for col_idx in 0..board[row_idx].len() {
            if board[row_idx][col_idx] == '.' {continue;}
            else if board[row_idx][col_idx] == 'W' { white_cnt += 1; }
            else { black_cnt += 1; }
        }
    }

    match (black_cnt, white_cnt) {
        (b, w) if b > w => println!("Black wins by {} points!", b-w),
        (b, w) if b < w => println!("White wins by {} points!", w-b),
        _ => println!("Draw!"),
    }
}

fn main() {
    // board instance
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut current_color = Color::Black;
    let mut no_more_moves_black = false;
    let mut no_more_moves_white = false;

    // init board
    init_board(&mut board, 8, 8);
    print_board(&board);


    // game loop
    loop {
        // check game state
        if no_more_moves_black && no_more_moves_white {
            // game over
            game_over(&board);
            break;
        }
        // check availability
        if check_availability(&board, current_color) {
            match current_color {
                Color::Black => {no_more_moves_black = false;},
                Color::White => {no_more_moves_white = false;},
            }
            // read input
            let (input_row_idx, input_col_idx) = read_input(&mut board, current_color);

            // new move
            match current_color {
                Color::Black => {board[input_row_idx][input_col_idx] = 'B';}
                Color::White => {board[input_row_idx][input_col_idx] = 'W';}
            }

            // flip the opponent disks
            check_flip(&mut board, input_row_idx, input_col_idx, current_color);
            print_board(&board);
        } else {
            match current_color {
                Color::Black => {no_more_moves_black = true;},
                Color::White => {no_more_moves_white = true;},
            }
            println!("{} player has no valid move.", match current_color {
                Color::Black => "B",
                Color::White => "W",
            });
        }

        // change turn
        current_color = match current_color {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }

    }



}
