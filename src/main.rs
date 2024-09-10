use rand::Rng;
use std::io;

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq)]
enum FieldData {
    X,
    O,
    None
}

type Board = [[FieldData; 3]; 3];

fn main() {
    let mut board: Board = [[FieldData::None; 3]; 3];
    let mut player_on_move: FieldData = get_first_player();
    display_board(&board);

    // while loop - until win or draw
    while is_game_active(&board) {
        if player_on_move == FieldData::X {
            // user is on the move ask for input and update board - keep asking until getting legal move
            let field_num = get_user_move(&board);
            println!("You chose field: {}.", field_num + 1);
            board = update_board(&board, field_num, &player_on_move);
            player_on_move = FieldData::O;
        } else {
            // bot is on the move (select random legal place)  
            let field_num = generate_bot_move(&board);
            println!("Bot chose field: {}.", field_num + 1);
            board = update_board(&board, field_num, &player_on_move);
            player_on_move = FieldData::X;
        }

        // display board
        display_board(&board);
    }

    // congratulate winner (if there is one)
    let result = check_for_winners(&board);
    match result {
        FieldData::None => println!("Draw!"),
        FieldData::O => println!("Computer won! You lost!"),
        FieldData::X => println!("Congratulations you won!")
    };
}

fn update_board(board: &Board, field_num: usize, player: &FieldData) -> Board {
    let mut new_board = *board;
    new_board[field_num / 3][field_num % 3] = *player;
    return new_board;
}

fn get_user_move(board: &Board) -> usize {
    loop {
        println!("Your turn. Enter a number:");
        let mut input = String::new();

        // Read the input from the user
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Attempt to parse the input as a number
        let number: Result<usize, _> = input.trim().parse();

        match number {
            Ok(n) => {
                match is_legal_move(&board, n-1) {
                    Err(_) => println!("The field must be empty."), // If move is illegal, print error and continue loop
                    Ok(_) => return n-1, // If move is legal, return the number and break the loop
                }
            }
            Err(_) => {
                println!("Invalid number, please try again.");
            }
        }
    }
}

fn generate_bot_move(board: &Board) -> usize {
    // random number 0-8
    let mut rng = rand::thread_rng();
    let field_num = rng.gen_range(0..9) as usize;

    // check if legal, if not repeat, else return
    match is_legal_move(&board, field_num) {
        Err(_) => generate_bot_move(&board), // If move is illegal, try again recursively
        Ok(_) => field_num // If move is legal, return the number
    }
}

fn is_legal_move(board: &Board, field_num: usize) -> Result<bool, &str> {
    // field num must be between 0 and 8
    if field_num > 8 {
        return Err("Move out of range");
    }

    // field must be free
    if board[field_num / 3][field_num % 3] != FieldData::None {
        return Err("Field already taken");
    }

    Ok(true)
}

fn is_game_active(board: &Board) -> bool {
    // check if game is active AKA next move can be played
    // check if game is won
    let game_state = check_for_winners(&board);
    let is_there_winner = match game_state {
        FieldData::O => true,
        FieldData::X => true,
        FieldData::None => false
    };

    // check if all fields are taken
    let is_full = are_fields_full(&board);

    return !is_full && !is_there_winner;
}

fn are_fields_full(board: &Board) -> bool {
    for row in board {
        for field in row {
            if *field == FieldData::None {
                return false;
            }
        }
    }

    true
}

fn check_for_winners(board: &Board) -> FieldData {
    let mut winner = FieldData::None;
    // check for horizontal wins
    board.iter().for_each(|&row| {
        if &row[0] == &row[1] &&  &row[2] == &row[1] {
            winner = row[0];
        };
    });

    // check for vertical
    for (index, &field) in board[0].iter().enumerate() {
        if 
            &board[0][index] == &board[1][index] &&
            &board[2][index] == &board[1][index]
        {
            winner = field;
        }
    };
    
    // check both diagonals
    if 
        (&board[0][0] == &board[1][1] && &board[2][2] == &board[1][1]) ||
        (&board[2][0] == &board[1][1] && &board[0][2] == &board[1][1])
    {
        winner = board[1][1];
    }

    // return winner
    winner
}

fn display_board(board: &Board) {
    println!("-------------");
    for (index, &row) in board.iter().enumerate() {
        println!(
            "| {} | {} | {} |",
            display_field_value(&row[0], &(1+(&index*3))),
            display_field_value(&row[1], &(2+(&index*3))),
            display_field_value(&row[2], &(3+(&index*3)))
        );
        if index == 2 {
            println!("-------------");
        } else {
            println!("|---|---|---|");
        }
    }
}

fn display_field_value(value: &FieldData, index: &usize) -> String {
    match value {
        FieldData::X => "X".to_string(),
        FieldData::O => "O".to_string(),
        FieldData::None => index.to_string(),
    }
}

fn get_first_player() -> FieldData{
    let mut rng = rand::thread_rng();
    let num: u8 = rng.gen();
    if num % 2 == 0 {
        println!("You start. You play X, bot plays O.");
        FieldData::X
    } else {
        println!("Bot starts. You play X, bot plays O.");
        FieldData::O
    }
}