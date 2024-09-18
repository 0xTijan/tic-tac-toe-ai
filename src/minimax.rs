use crate::FieldData;
use crate::Board;
use crate::check_for_winners;

// Struct to store minimax results
pub struct MinimaxRes {
  pub score: i32,
  pub index: Option<i32>,
}

pub fn minimax(board: &mut Board, curr_player: &FieldData) -> MinimaxRes {
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
        // ai - maximizing player
        let mut best_score: i32 = -10000;
        for i in all_test_play_infos {
            if i.score > best_score {
                // found better test play
                best_score = i.score;
                best_test_play = i;
            }
        }
    } else {
        // human - minimizing player
        let mut best_score: i32 = 10000;
        for i in all_test_play_infos {
            if i.score < best_score {
                // found better test play
                best_score = i.score;
                best_test_play = i;
            }
        }
    }

    // println!("W: {:?}", curr_winner);
    return best_test_play;
}

pub fn find_empty_fields(board: &Board) -> Vec<usize> {
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
