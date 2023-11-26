// todo: potentially pass transposition table as a parameter to agent

use crate::board::Board;
use crate::opening_db::OpeningDatabase;
use crate::transposition::TranspositionTable;
use crate::action_ordering::ActionOrdering;

const MIN_SCORE: i8 = -18;
const MAX_SCORE: i8 = 18;

#[derive(Clone)]
pub struct Agent<'a> {
    opening_db: Option<&'a OpeningDatabase>,
    transposition_table: TranspositionTable,
    visited: u64,
}

impl<'a> Agent<'a> {
    pub fn new(opening_db: Option<&'a OpeningDatabase>) -> Agent {
        Agent {
            opening_db: opening_db,
            transposition_table: TranspositionTable::new(),
            visited: 0,
        }
    }

    pub fn best_col(&mut self, board: Board) -> (u8, i8) {
        let mut col: u8 = 0;
        let mut score = -127;

        let action_mask = board.playable_tile_mask();
        for i in [3, 2, 4, 1, 5, 0, 6] {
            let action = Board::get_action(action_mask, i);
            if action != 0 {
                if board.is_winning_action(action) { 
                    return (i as u8, 21 - (board.num_actions() as i8) / 2); 
                }

                let child = board.make_action(action);
                let child_score = -self.best_score(child).0;

                if child_score > score {
                    score = child_score;
                    col = i as u8;
                }
            }
        }

        (col, score)
    }

    pub fn best_score(&mut self, board: Board) -> (i8, u64) {
        if board.has_winning_action() { return (21 - (board.num_actions() as i8) / 2, 0); }

        self.visited = 0;

        let mut min = -(42 - board.num_actions() as i8) / 2;
        let mut max = (43 - board.num_actions() as i8) / 2;

        while min < max {
            let mut med = min + (max - min) / 2;
            if med <= 0 && min / 2 < med { med = min / 2; }
            else if med >= 0 && max / 2 > med { med = max / 2; }

            let res = self.negamax(board, med, med + 1);
            if res <= med { max = res; }
            else { min = res; }
        }

        (min, self.visited)
    }

    fn negamax(&mut self, board: Board, mut alpha: i8, mut beta: i8) -> i8 {
        self.visited += 1;

        let actions_mask = board.get_non_losing_actions();
        if actions_mask == 0 { return -(42 - board.num_actions() as i8) / 2; }

        if board.num_actions() >= 40 { return 0; }

        let mut min: i8 = -(40 - board.num_actions() as i8) / 2;
        if alpha < min {
            alpha = min;
            if alpha >= beta { return alpha; }
        }

        let mut max: i8 = (41 - board.num_actions() as i8) / 2;
        if beta > max {
            beta = max;
            if alpha >= beta { return beta; }
        }

        let hash = board.hash();
        if let Some(score) = self.transposition_table.get(hash) {
            if score > MAX_SCORE - MIN_SCORE + 1 {
                min = score + 2*MIN_SCORE - MAX_SCORE - 2;
                if alpha < min {
                    alpha = min;
                    if alpha >= beta { return alpha; }
                }
            } else {
                max = score + MIN_SCORE - 1;
                if beta > max {
                    beta = max;
                    if alpha >= beta { return beta; }
                }
            }
        }

        if let Some(ref db) = self.opening_db {
            if let Some(score) = db.get(board.hash(), board.num_actions()) {
                return score;
            }
        }

        let sym = board.is_symmetrical();
        let mut actions_ordered = ActionOrdering::new();
        for i in if sym {[0, 1, 2, 3].iter()} else {[6, 0, 5, 1, 4, 2, 3].iter()} {
            let action = Board::get_action(actions_mask, *i);
            if action != 0 {
                actions_ordered.push(action, board.get_action_score(action));
            }
        }

        for action in actions_ordered {
            let child = board.make_action(action);
            let score = -self.negamax(child, -beta, -alpha);

            if score >= beta {
                self.transposition_table.set(hash, score + MAX_SCORE - 2*MIN_SCORE + 2);
                return score; 
            }
            if score > alpha { alpha = score; }
        }

        self.transposition_table.set(hash, alpha - MIN_SCORE + 1);
        alpha
    }
}
