use super::*;

use std::collections::HashSet as Set;

pub struct Annotation<'a> {
    pub possible_islands: Grid<Set<Island>>,
    pub board: &'a Board,
}

impl<'a> Annotation<'a> {
    pub fn new(board: &'a Board) -> Self {
        let (h, w) = board.dims();
        let island_set = Set::from_iter(board.islands.iter().copied());
        let mut this = Self {
            // Initially assume any island could reach any tile
            possible_islands: vec![vec![island_set; w]; h],
            board,
        };

        // Fill in islands we already know
        this.fill_islands(board);

        // Empty out islands which are already sea
        this.remove_seas(board);

        // Indicate which islands each tile may be a part of
        // Loop since each pass may inform later passes
        while this.reachability_pass(board) {}

        this
    }

    pub fn island(&self, (r, c): Coord) -> Option<Island> {
        let is = &self.possible_islands[r][c];
        if is.len() == 1 {
            is.iter().copied().next()
        } else {
            None
        }
    }

    fn fill_islands(&mut self, board: &Board) {
        for &island in &board.islands {
            let Island { r, c, .. } = island;
            let mut area = area(board, (r, c));
            area.extend(&surrounding(board, &area));

            for &(r, c) in &area {
                let possibles = &mut self.possible_islands[r][c];
                possibles.clear();
                possibles.insert(island);
            }
        }
    }

    fn remove_seas(&mut self, board: &Board) {
        for ((r, c), t) in board.iter() {
            if t == Sea {
                self.possible_islands[r][c].clear();
            }
        }
    }

    fn reachability_pass(&mut self, board: &Board) -> bool {
        let mut changed = false;

        for ((r, c), t) in board.iter() {
            if t == Land {
                continue;
            }

            for &island in &board.islands {
                if self.possible_islands[r][c].contains(&island)
                    && !island_reaches(board, self, island, (r, c))
                {
                    self.possible_islands[r][c].remove(&island);
                    changed = true;
                }
            }

        }

        changed
    }
}
