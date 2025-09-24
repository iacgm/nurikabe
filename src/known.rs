use super::*;

pub use rustc_hash::FxHashSet as Set;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Possibility {
    Isle(Island),
    Sea,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Volume {
    Contradiction,
    Loud(Reason),  // Display rule
    Quiet(Reason), // Knowledge updated, but don't display
    Nil,
}

#[derive(Clone)]
pub struct Knowledge {
    pub depth: usize,
    pub reason: Volume, // Gets disabled when we make a new change
    islands: Vec<Island>,
    possibilities: Grid<Set<Possibility>>, // None = Sea
}

impl Knowledge {
    pub fn new(board: &Board) -> Self {
        use Possibility::*;
        use Volume::*;

        let (h, w) = board.dims();

        let mut possibility_space = Set::from_iter(board.islands.iter().copied().map(Isle));
        possibility_space.insert(Sea);

        let mut possibilities = vec![vec![possibility_space.clone(); w]; h];

        for &Island { r, c, n } in board.islands.iter() {
            possibilities[r][c] = [Isle((r, c, n).into())].into_iter().collect();
        }

        Self {
            depth: 0,
            reason: Nil,
            islands: board.islands.clone(),
            // Initially assume any island could reach any tile
            possibilities,
        }
    }

    pub fn board(&self) -> Board {
        let h = self.possibilities.len();
        let w = self.possibilities[0].len();

        let mut board = Board::from_islands(h, w, self.islands.iter().copied());

        for r in 0..h {
            for c in 0..w {
                if let Some(tile) = self.tile_known((r, c)) {
                    board[(r, c)] = tile;
                }
            }
        }

        board
    }

    pub fn get_mut(&mut self, (r, c): Coord) -> &mut Set<Possibility> {
        &mut self.possibilities[r][c]
    }

    pub fn get(&self, (r, c): Coord) -> &Set<Possibility> {
        &self.possibilities[r][c]
    }

    pub fn known_sea(&self, c: Coord) -> bool {
        use Possibility::*;
        self.if_known(c) == Some(Sea)
    }

    pub fn known_land(&self, c: Coord) -> bool {
        use Possibility::*;
        !self.get(c).contains(&Sea)
    }

    pub fn tile_known(&self, c: Coord) -> Option<Tile> {
        use Tile::*;
        if self.known_land(c) {
            Some(Land)
        } else if self.known_sea(c) {
            Some(Water)
        } else {
            None
        }
    }

    pub fn if_known(&self, (r, c): Coord) -> Option<Possibility> {
        let is = &self.possibilities[r][c];
        if is.len() == 1 {
            is.iter().copied().next()
        } else {
            None
        }
    }

    pub fn island_set(&self) -> &Vec<Island> {
        &self.islands
    }

    pub fn elim_island(&mut self, reason: Reason, c: Coord, i: Island) {
        use Possibility::*;
        use Volume::*;
        let was_known = self.tile_known(c).is_some();

        let possibilities = self.get_mut(c);
        if !possibilities.remove(&Isle(i)) {
            return;
        }

        if self.tile_known(c).is_some() && !was_known {
            self.reason.set(Loud(reason));
        } else {
            self.reason.set(Quiet(reason));
        }
    }

    // Acknowledge a contradiction
    pub fn contradict(&mut self) {
        self.reason = Volume::Contradiction;
    }

    pub fn set_land(&mut self, reason: Reason, c: Coord) {
        use Possibility::*;
        use Volume::*;
        if !self.known_land(c) {
            self.reason = Loud(reason);
            self.get_mut(c).remove(&Sea);
        }
    }

    pub fn set_sea(&mut self, reason: Reason, c: Coord) {
        use Possibility::*;
        use Volume::*;
        if !self.known_sea(c) {
            self.reason = Loud(reason);
            self.get_mut(c).retain(|p| p == &Sea);
        }
    }

    pub fn bifurcate(&self) -> Self {
        let mut copy = self.clone();
        copy.depth += 1;
        copy
    }

    pub fn grid(&self) -> &Grid<Set<Possibility>> {
        &self.possibilities
    }

    pub fn solved(&self) -> bool {
        use Volume::*;
        self.reason != Contradiction && self.possibilities.iter().flatten().all(|s| s.len() == 1)
    }
}

impl Volume {
    pub fn set(&mut self, other: Self) {
        use Volume::*;
        if let (Nil | Quiet(_), Loud(r)) = (*self, other) {
            *self = Loud(r)
        };
    }

    pub fn is_set(&self) -> bool {
        !matches!(self, Self::Nil)
    }

    pub fn take(&mut self) -> Volume {
        use Volume::*;
        let reason = *self;
        *self = Nil;
        reason
    }
}
