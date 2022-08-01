mod mancala;
mod search;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let board = mancala::Mancala::new();

    // println!("{}", search::best_move(board.clone(), 16).unwrap());

    for _ in 0..10 {
        search::best_move(board.clone(), 15).unwrap();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    return;

    loop {
        // board.print();

        if board.own_turn {
            let best_move = search::best_move(board.clone(), 15).unwrap();

            println!("Making move {}", best_move);

            board.make_move(best_move);
        } else {
            board.print();

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
