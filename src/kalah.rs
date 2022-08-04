use std::fmt::Display;

use crate::mancala::{Mancala, MancalaGame};
use crate::stacklist::StackList;

/// Game interface for Kalah.
/// Wraps Mancala
#[derive(Clone)]
pub struct KalahGame {
    board: Mancala,
    own_turn: bool
}

impl MancalaGame for KalahGame {
    fn make_move(&mut self, mut pit_idx: usize) {
        // Grab all the stones from the pit in question
        let mut stone_count = self.board.get_pit(pit_idx);
        self.board.set_pit(pit_idx, 0);

        // While we still have stones in our "hand"
        while stone_count > 0 {
            if pit_idx < 13 {
                pit_idx += 1;
            } else {
                pit_idx = 0;
            }
            
            // Go around the board and drop them into pits,
            // ignoring the enemy store.
            if !((self.own_turn && pit_idx == 13) || (!self.own_turn && pit_idx == 6)) {
                self.board.increment_pit(pit_idx, 1);
                stone_count -= 1;
            }
        }

        // You get another turn if you place your last stone in your store
        if pit_idx != 13 && pit_idx != 6 {
            // Placing a stone in an empty pit allows you to capture
            if self.board.get_pit(pit_idx) == 1 {
                // Ensure it's actually on our side
                if (self.own_turn && pit_idx < 6) || (!self.own_turn && pit_idx > 6) {
                    let captured_idx = 12 - pit_idx;
                    // If there are stones in the opposite pit
                    if self.board.get_pit(captured_idx) != 0 {
                        let dst = if self.own_turn { 6 } else { 13 };

                        // Move stones from both pits into our store
                        self.board.increment_pit(dst, self.board.get_pit(captured_idx));
                        self.board.increment_pit(dst, self.board.get_pit(pit_idx));

                        self.board.set_pit(captured_idx, 0);
                        self.board.set_pit(pit_idx, 0);
                    }
                }
            }

            self.own_turn = !self.own_turn;
        }
    }

    fn valid_moves(&self) -> StackList<usize, 6> {
        // Maximum of 6 valid moves generated.
        let mut out: StackList<usize, 6> = StackList::new();
    
        let offset = if self.own_turn { 0 } else { 7 };
    
        // If there are stones in the pit, add them to the legal moves table.
        for i in 0..6 {
            if self.board.get_pit(offset + i) != 0 { out.push(offset + i).unwrap(); }
        }

        out
    }

    fn eval(&self) -> i32 {
        // Consider improving. Works well enough for now.
        self.board.get_pit(6) as i32 - self.board.get_pit(13) as i32
    }

    fn is_own_turn(&self) -> bool {
        self.own_turn
    }
}

impl KalahGame {
    /// Consume a Mancala board and return a Kalah implementation
    pub fn from_mancala(board: Mancala) -> Self {
        Self { board, own_turn: false }
    }
}

impl Display for KalahGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.fmt(f)
    }
}