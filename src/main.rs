use rand::Rng;

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
    display_board(&board);
    let mut player_on_move: FieldData = get_first_player();
    
    // while loop - until win or draw
    while is_game_active(&board) {
        if player_on_move == FieldData::X {
            // user is on the move ask for input and update board - keep asking until getting legal move

            player_on_move = FieldData::O;
        } else {
            // bot is on the move (select random legal place)  
            generate_bot_move(&board);
            player_on_move = FieldData::X;
        }
        // update board
    }

    // congratulate winner (if there is one)
    let result = check_for_winners(&board);
    match result {
        FieldData::None => println!("Draw!"),
        FieldData::O => println!("Computer won! You lost!"),
        FieldData::X => println!("Congratulations you won!")
    };
}

fn generate_bot_move(board: &Board) -> i32 {
    // random number 0-8
    // check if legal, if not repeat, else return
    1
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

    return (!is_full && !is_there_winner);
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
            display_field_value(&row[0], &(2+(&index*3))),
            display_field_value(&row[0], &(3+(&index*3)))
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