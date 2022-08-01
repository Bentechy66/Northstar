// Mancala Board pit IDs:
// Own Pits     Enemy Pits
//
//   [Enemy Store: 13]
//
//    0     ID     12
//    1     ID     11
//    2     ID     10
//    3     ID     9
//    4     ID     8
//    5     ID     7
//
//   [ Own Store: 6 ]

#[derive(Clone)]
pub struct Mancala {
    pub own_turn: bool,
    pits:         [u32; 14],
}

impl Mancala {
    pub fn new() -> Self {
        Mancala {
            own_turn: true,
            // pits: [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0]
            pits: [3, 2, 2, 4, 2, 2, 11, 4, 6, 2, 3, 1, 0, 6]
        }
    }

    pub fn print(&self) {
        println!(
            " [{}]\n{}   {}\n{}   {}\n{}   {}\n{}   {}\n{}   {}\n{}   {}\n [{}]\n\n{} to move", 
            self.pits[13], 
            self.pits[0], self.pits[12],
            self.pits[1], self.pits[11],
            self.pits[2], self.pits[10],
            self.pits[3], self.pits[9],
            self.pits[4], self.pits[8],
            self.pits[5], self.pits[7],
            self.pits[6],
            if self.own_turn { "self" } else { "other" }
        );
    }

    pub fn make_move(&mut self, mut pit_idx: usize) {
        let mut stone_count = self.pits[pit_idx];
        self.pits[pit_idx] = 0;

        while stone_count > 0 {
            if pit_idx < 13 {
                pit_idx += 1;
            } else {
                pit_idx = 0;
            }
            
            if !((self.own_turn && pit_idx == 13) || (!self.own_turn && pit_idx == 6)) {
                self.pits[pit_idx] += 1;
                stone_count -= 1;
            }
        }

        // You get another turn if you place your last stone in your store
        if pit_idx != 13 && pit_idx != 6 {
            // Placing a stone in an empty pit allows you to capture
            if self.pits[pit_idx] == 1 {
                // Ensure it's actually on our side
                if self.own_turn && pit_idx < 6 {
                    let captured_idx = 12 - pit_idx;
                    if self.pits[captured_idx] != 0 {
                        self.pits[6] += self.pits[captured_idx];
                        self.pits[6] += self.pits[pit_idx];

                        self.pits[captured_idx] = 0;
                        self.pits[pit_idx] = 0;
                    }
                } else if !self.own_turn && pit_idx > 6 {
                    let captured_idx = 12 - pit_idx;
                    if self.pits[captured_idx] != 0 {
                        self.pits[13] += self.pits[captured_idx];
                        self.pits[13] += self.pits[pit_idx];

                        self.pits[captured_idx] = 0;
                        self.pits[pit_idx] = 0;
                    }
                }

                
            }

            self.own_turn = !self.own_turn;
        }
    }

    pub fn valid_moves(&self) -> Vec<usize> {
        let mut out = Vec::with_capacity(6);

        if self.own_turn {
            if self.pits[0] != 0 { out.push(0); }
            if self.pits[1] != 0 { out.push(1); }
            if self.pits[2] != 0 { out.push(2); }
            if self.pits[3] != 0 { out.push(3); }
            if self.pits[4] != 0 { out.push(4); }
            if self.pits[5] != 0 { out.push(5); }
        } else {
            if self.pits[7] != 0  { out.push(7); }
            if self.pits[8] != 0  { out.push(8); }
            if self.pits[9] != 0  { out.push(9); }
            if self.pits[10] != 0 { out.push(10); }
            if self.pits[11] != 0 { out.push(11); }
            if self.pits[12] != 0 { out.push(12); }
        }

        out
    }

    pub fn eval(&self) -> i32 {
        self.pits[6] as i32 - self.pits[13] as i32
    }
}

// enum MancalaMoveResult {
//     TurnFinished,
//     CanMakeAnotherMove
// }