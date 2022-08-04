mod mancala;
mod search;
mod stacklist;
mod kalah;

use crate::mancala::MancalaGame;

fn main() {
    let mut board = kalah::KalahGame::from_mancala(mancala::Mancala::new());

    loop {
        if board.is_own_turn() {
            let best_move = search::best_move(board.clone(), 15).unwrap();

            println!("Making move {}", best_move);

            board.make_move(best_move);
        } else {
            println!("{}", board);

            print!("Enter pit to move with (id) > ");

            let mut input_text = String::new();
            std::io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            match trimmed.parse::<u32>() {
                Ok(i) => board.make_move(i as usize),
                Err(..) => println!("this was not an integer: {}", trimmed),
            };
        }
    }
}
