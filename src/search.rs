use crate::mancala;

pub fn best_move(board: mancala::Mancala, depth: u8) -> Option<usize> {
    let mut best_move = None;
    let mut best_score = i32::MIN;

    for mve in board.valid_moves() {
        let mut board_copy = board.clone();

        board_copy.make_move(mve);

        let score = minimax(board_copy, depth - 1);
        if score > best_score {
            best_score = score;
            best_move = Some(mve);
        }
    }

    best_move
}

pub fn minimax(board: mancala::Mancala, depth: u8) -> i32 {
    if depth == 0 {
        return board.eval()
    }

    let mut value;

    if board.own_turn {
        value = i32::MIN;

        for mve in board.valid_moves() {
            let mut board_copy = board.clone();

            board_copy.make_move(mve);

            value = std::cmp::max(value, minimax(board_copy, depth - 1));
        }
    } else {
        value = i32::MAX;

        for mve in board.valid_moves() {
            let mut board_copy = board.clone();

            board_copy.make_move(mve);

            value = std::cmp::min(value, minimax(board_copy, depth - 1));
        }
    }

    value
}