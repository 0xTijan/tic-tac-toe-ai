use rand::Rng;
use std::io;

#[derive(Copy, Clone, Debug, PartialEq)]

enum FieldData {
    X,
    O,
    None
}

enum GameType {
    Random,
    Minimax,
    MCTS
}

impl GameType {
    // Function to map a number to the GameType
    fn from_number(number: i32) -> Option<GameType> {
        match number {
            1 => Some(GameType::Random),
            2 => Some(GameType::Minimax),
            3 => Some(GameType::MCTS),
            _ => None, // invalid input
        }
    }
}

type Board = [[FieldData; 3]; 3];

struct MinimaxRes {
    score: i32,
    index: Option<i32>,
}

fn main() {
    let game_type = read_game_type();
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
            let field_num = generate_bot_move(&mut board, &game_type);
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

fn read_game_type() -> GameType {
    loop {
        println!("Select game type (how you want the bot to play):");
        println!("1 Random (easy)");
        println!("2 Minimax (hard)");
        println!("3 Monte Carlo Tree Search (hard)");

        let mut input = String::new();

        // Read the input from the user
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Attempt to parse the input as a number
        let number: Result<i32, _> = input.trim().parse();

        match number {
            Ok(n) => {
                // Check if number is in range
                match GameType::from_number(n) {
                    Some(t) => {
                        println!("Selected game type: {:?}", n);
                        return t;
                    },
                    None => println!("Number must be between 1 and 3."),
                };
            }
            Err(_) => {
                println!("Invalid number, please try again.");
            }
        }
    }
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

fn generate_bot_move(board: &mut Board, game_type: &GameType) -> usize {
    match game_type {
        GameType::Random => random_bot_move(board),
        GameType::Minimax => {
            let res = minimax(board, &FieldData::O);
            match res.index {
                None => panic!("Something went wrong!"),
                Some(i) => i as usize
            }
        },
        GameType::MCTS => 1
    }
}

fn minimax(board: &mut Board, curr_player: &FieldData) -> MinimaxRes {
    // let mut board_array = flatten_2d_board(&board);
    let empty_fields = find_empty_fields(&board);

    // check if final state (win, loss, draw) - these return statements will only happen from recursive calls in for loop (not from main call of this function)
    let curr_winner = check_for_winners(&board);
    if curr_winner == FieldData::X {
        // human won, punish
        return MinimaxRes{
            score: -1,
            index: None,
        };
    } else if curr_winner == FieldData::O {
        // ai won, award
        return MinimaxRes{
            score: 1,
            index: None,
        };
    } else if curr_winner == FieldData::None && empty_fields.len() == 0 {
        // draw, zero reward
        return MinimaxRes{
            score: 0,
            index: None,
        };
    }

    let mut all_test_play_infos: Vec<MinimaxRes> = Vec::new();

    // simulate all possible moves
    for i in empty_fields {
        let mut curr_play_info: MinimaxRes = MinimaxRes{
            index: Some(i as i32),
            score: 0
        };

        // simulate move - placing current player mark on the board
        board[i / 3][i % 3] = *curr_player;

        // recursively run minimax on updated boards (runs until final board state (win, loss, draw))
        if *curr_player == FieldData::O {
            // ai
            let res = minimax(board, &FieldData::X);    // pass human (O) - opponent
            curr_play_info.score = res.score;
        } else {
            // human
            let res = minimax(board, &FieldData::O);    // pass ai (X) - opponent
            curr_play_info.score = res.score;
        }

        // after simulation was done, reset the board
        board[i / 3][i % 3] = FieldData::None;
        // save the result of the current test play (score and the index of the field of the current test play)
        all_test_play_infos.push(curr_play_info);
    }

    let mut best_test_play: MinimaxRes = MinimaxRes{
        score: 0,
        index: Some(0),
    };

    // find current players best test play and return it
    if *curr_player == FieldData::O {
        // ai
        let mut best_score: i32 = -10000;
        for i in all_test_play_infos {
            if i.score > best_score {
                // found better test play
                best_score = i.score;
                best_test_play = i;
            }
        }
    } else {
        // human
        let mut best_score: i32 = 10000;
        for i in all_test_play_infos {
            if i.score < best_score {
                // found better test play
                best_score = i.score;
                best_test_play = i;
            }
        }
    }

    return best_test_play;
}

fn flatten_2d_board(board: &Board) -> [FieldData; 9] {
    let mut flattened: [FieldData; 9] = [FieldData::None; 9];
    for (i, row) in board.iter().enumerate() {
        for (j, field) in row.iter().enumerate() {
            flattened[i * 3 + j] = *field;
        }
    }
    flattened
}

fn find_empty_fields(board: &Board) -> Vec<usize> {
    let mut available_fields = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, field) in row.iter().enumerate() {
            if *field == FieldData::None {
                available_fields.push(j+(&i*3));
            }
        }
    }
    available_fields
}

fn random_bot_move(board: &Board) -> usize {
    // random number 0-8
    let mut rng = rand::thread_rng();
    let field_num = rng.gen_range(0..9) as usize;

    // check if legal, if not repeat, else return
    match is_legal_move(&board, field_num) {
        Err(_) => random_bot_move(&board), // If move is illegal, try again recursively
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