mod mancala;
mod search;

fn main() {
    let mut board = mancala::Mancala::new();

    loop {
        // board.print();

        if board.own_turn {
            let best_move = search::best_move(board.clone(), 9).unwrap();

            println!("Making move {}", best_move);

            board.make_move(best_move);
        } else {
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
