const WIDTH: usize = 7;
const HEIGHT: usize = 6;
// const SIZE: usize = WIDTH * HEIGHT;

const BOTTOM_ROW_MASK: u64 = 0b0000001000000100000010000001000000100000010000001;
const PLAYABLE_AREA_MASK: u64 = 0b0111111011111101111110111111011111101111110111111;


#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Board {
    player_mask: u64,
    tile_mask: u64,
    num_actions: usize,
}


impl Board {
    pub fn new() -> Board {
        Board {
            player_mask: 0,
            tile_mask: 0,
            num_actions: 0,
        }
    }

    pub fn from_position(position: &str) -> Result<Board, ()> {
        let mut board = Board::new();
        for c in position.chars() {
            if !c.is_digit(10) { return Err(()); }

            let col: u32 = c.to_digit(10).unwrap() - 1;
            if col > 6 || !board.is_valid_col(col as usize) { return Err(()); } 

            board.play_col(col as usize);
        }

        Ok(board)
    }

    pub fn play_col(self: &mut Board, col: usize) {
        self.player_mask ^= self.tile_mask;
        self.tile_mask |= self.tile_mask + (1 << (col * 7));
        self.num_actions += 1;
    }

    pub fn make_action(self, action: u64) -> Board {
        Board {
            player_mask: self.player_mask ^ self.tile_mask,
            tile_mask: self.tile_mask | action,
            num_actions: self.num_actions + 1,
        }
    }

    pub fn is_valid_col(self: &Board, col: usize) -> bool {
        self.playable_tile_mask() & (0b0111111 << (col * 7)) != 0
    }

    pub fn is_valid_action(self: &Board, actions_mask: u64, col: usize) -> bool {
        actions_mask & (0b0111111 << (col * 7)) != 0
    }

    pub fn is_winning_col(self: &Board, col: usize) -> bool {
        self.is_winning_action(self.playable_tile_mask() & (0b0111111 << (col * 7)))
    }

    pub fn is_winning_action(self: &Board, action: u64) -> bool {
        let b: u64 = self.player_mask | action;
        
        // vertical check
        let bb = b & (b >> 1);
        if bb & (bb >> 2) != 0 { return true; }

        // horizontal check
        let bb = b & (b >> 7);
        if bb & (bb >> 14) != 0 { return true; }

        // diagonal 1 check
        let bb = b & (b >> 8);
        if bb & (bb >> 16) != 0 { return true; }

        // diagonal 2 check
        let bb = b & (b >> 6);
        if bb & (bb >> 12) != 0 { return true; }

        false
    }

    pub fn get_non_losing_actions(self: Board) -> u64 {
        let mut playable = self.playable_tile_mask();
        let enemy_win = winning_tile_mask(self.player_mask ^ self.tile_mask, self.tile_mask);
        let forced_actions = playable & enemy_win;

        // forced loss, there are no non-losing actions
        if forced_actions != 0 {
            if forced_actions & (forced_actions - 1) != 0 { return 0; }
            else { playable = forced_actions; }
        }
        
        // don't play right below enemy winning spot
        playable & !(enemy_win >> 1)
    }

    pub fn get_action_score(self: Board, action: u64) -> u32 {
        winning_tile_mask(self.player_mask | action, self.tile_mask).count_ones()
    }

    pub fn has_winning_action(self: Board) -> bool {
        self.playable_tile_mask() & winning_tile_mask(self.player_mask, self.tile_mask) != 0
    }

    pub fn playable_tile_mask(self: Board) -> u64 {
        (self.tile_mask + BOTTOM_ROW_MASK) & PLAYABLE_AREA_MASK
    }

    pub fn num_actions(self: &Board) -> usize {
        self.num_actions
    }

    pub fn hash(self: &Board) -> u64 {
        self.player_mask + self.tile_mask
    }

    pub fn is_symmetrical(self: &Board) -> bool {
        if self.num_actions & 1 == 1 { return false; }

        let b1 = self.player_mask;
        if get_col(b1, 0) != get_col(b1, 6) { return false; }
        if get_col(b1, 1) != get_col(b1, 5) { return false; }
        if get_col(b1, 2) != get_col(b1, 4) { return false; }

        let b2 = self.tile_mask;
        if get_col(b2, 0) != get_col(b2, 6) { return false; }
        if get_col(b2, 1) != get_col(b2, 5) { return false; }
        if get_col(b2, 2) != get_col(b2, 4) { return false; }

        true
    }

    pub fn get_action(actions_mask: u64, col: i32) -> u64 {
        actions_mask & (0b0111111 << (col * 7))
    }

    pub fn print(self: &Board) {
        let red_mask = if self.num_actions % 2 == 0 { self.player_mask } else { self.tile_mask ^ self.player_mask };
        let yellow_mask = self.tile_mask ^ red_mask;

        for r in (0..HEIGHT).rev() {
            for c in 0..WIDTH {
                if red_mask & (1 << (c * WIDTH + r)) != 0 {
                    print!("🔴");
                } else if yellow_mask & (1 << (c * WIDTH + r)) != 0 {
                    print!("🟡");
                } else {
                    print!("⚪");
                }
            }
            println!();
        }
    }
}

fn get_col(b: u64, c: usize) -> u64 {
    b >> (c * WIDTH) & 0b1111111
}

fn winning_tile_mask(player_mask: u64, tile_mask: u64) -> u64 {
    let pos = player_mask;

    // vertical
    let mut r: u64 = (pos << 1) & (pos << 2) & (pos << 3);

    // horizontal
    let p = (pos << 7) & (pos << 14);
    r |= p & (pos << 21);
    r |= p & (pos >> 7);

    let p = (pos >> 7) & (pos >> 14);
    r |= p & (pos << 7);
    r |= p & (pos >> 21);

    // diagonal 1
    let p = (pos << 8) & (pos << 16);
    r |= p & (pos << 24);
    r |= p & (pos >> 8);

    let p = (pos >> 8) & (pos >> 16);
    r |= p & (pos << 8);
    r |= p & (pos >> 24);

    // diagonal 2
    let p = (pos << 6) & (pos << 12);
    r |= p & (pos << 18);
    r |= p & (pos >> 6);

    let p = (pos >> 6) & (pos >> 12);
    r |= p & (pos << 6);
    r |= p & (pos >> 18);

    r & (tile_mask ^ PLAYABLE_AREA_MASK)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::new();
        assert_eq!(board.player_mask, 0);
        assert_eq!(board.tile_mask, 0);
        assert_eq!(board.num_actions, 0);
    }

    #[test]
    fn test_from_position() {
        let board = Board::from_position("").unwrap();
        assert_eq!(board.player_mask, 0);
        assert_eq!(board.tile_mask, 0);
        assert_eq!(board.num_actions, 0);

        let board = Board::from_position("4436212").unwrap();
        assert_eq!(board.player_mask, 0b0000100000000000010000000000000000000001);
        assert_eq!(board.tile_mask, 0b0000100000000000011000000100000110000001);
        assert_eq!(board.num_actions, 7);

        let board = Board::from_position("444447533335555").unwrap();
        assert_eq!(board.player_mask, 0b000001000000000010100001010000010100000000000000);
        assert_eq!(board.tile_mask, 0b000001000000000111110011111000111100000000000000);
        assert_eq!(board.num_actions, 15);

        let board = Board::from_position("444444").unwrap();
        assert_eq!(board.player_mask, 0b00000010101000000000000000000000);
        assert_eq!(board.tile_mask, 0b00000111111000000000000000000000);
        assert_eq!(board.num_actions, 6);

        let board = Board::from_position("44444444");
        assert!(board.is_err());

        let board = Board::from_position("4450");
        assert!(board.is_err());

        let board = Board::from_position("4458");
        assert!(board.is_err());

        let board = Board::from_position("error");
        assert!(board.is_err());
    }
}
