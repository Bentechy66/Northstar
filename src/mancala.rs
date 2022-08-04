use std::fmt::Display;
use crate::stacklist::StackList;

/// Describes a Mancala board.
/// Lightweight wrapper around an array of pits.
/// 
/// Mancala Board pit IDs:
/// Own Pits     Enemy Pits
///
///   [Enemy Store: 13]
///
///    0     ID     12
///    1     ID     11
///    2     ID     10
///    3     ID     9
///    4     ID     8
///    5     ID     7
///
///   [ Own Store: 6 ]
#[derive(Clone)]
pub struct Mancala {
    pits:         [u32; 14],
}

impl Mancala {
    /// Return a new Mancala board with 4 stones in each pit, and 0 in each store.
    pub fn new() -> Self {
        Mancala {
            pits: [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0]
        }
    }

    /// Set the amount of stones in a pit
    /// Refer to the [struct-level documentation](Mancala) for pit IDs
    pub fn set_pit(&mut self, pit_id: usize, value: u32) {
        self.pits[pit_id] = value;
    }

    /// Get the amount of stones in a pit
    /// Refer to the [struct-level documentation](Mancala) for pit IDs
    pub fn get_pit(&self, pit_id: usize) -> u32 {
        self.pits[pit_id]
    }

    /// Increment the amount of stones in a pit
    /// Refer to the [struct-level documentation](Mancala) for pit IDs
    pub fn increment_pit(&mut self, pit_id: usize, by: u32) {
        self.pits[pit_id] += by as u32;
    }
}

impl Display for Mancala {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " [{}]\n{}   {}\n{}   {}\n{}   {}\n{}   {}\n{}   {}\n{}   {}\n [{}]", 
            self.pits[13], 
            self.pits[0], self.pits[12],
            self.pits[1], self.pits[11],
            self.pits[2], self.pits[10],
            self.pits[3], self.pits[9],
            self.pits[4], self.pits[8],
            self.pits[5], self.pits[7],
            self.pits[6]
        )
    }
}

pub trait MancalaGame {
    fn make_move(&mut self, pit_idx: usize);
    fn valid_moves(&self) -> StackList<usize, 6>;
    fn eval(&self) -> i32;
    fn is_own_turn(&self) -> bool;
}