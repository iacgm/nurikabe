use super::*;

pub use rustc_hash::FxHashMap as Map;
pub use rustc_hash::FxHashSet as Set;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Possibility {
    Isle(Island),
    Sea,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReasonKind {
    MaxDepthReached,
    Contradiction,
    Loud(Reason),  // Display rule
    Quiet(Reason), // Knowledge updated, but don't display
    Nil,
}

#[derive(Clone)]
pub struct Knowledge {
    pub unique: bool,
    pub depth: usize,
    pub max_depth: usize,
    pub reason: ReasonKind, // Gets disabled when we make a new change
    islands: Vec<Island>,
    possibilities: Grid<Set<Possibility>>, // None = Sea
    island_paths: Map<Island, Vec<Area>>,
}

impl Knowledge {
    pub fn new(board: &Board) -> Self {
        use Possibility::*;
        use ReasonKind::*;

        let (h, w) = board.dims();

        let mut possibility_space = Set::from_iter(board.islands.iter().copied().map(Isle));
        possibility_space.insert(Sea);

        let mut possibilities = vec![vec![possibility_space.clone(); w]; h];

        for &Island { r, c, n } in board.islands.iter() {
            possibilities[r][c] = [Isle((r, c, n).into())].into_iter().collect();
        }

        Self {
            depth: 0,
            max_depth: 1,
            reason: Nil,
            islands: board.islands.clone(),
            // Initially assume any island could reach any tile
            possibilities,
            unique: true,
            island_paths: Default::default(),
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
        use ReasonKind::*;
        let was_known = self.tile_known(c).is_some();

        let possibilities = self.get_mut(c);
        if !possibilities.remove(&Isle(i)) {
            return;
        }
        self.island_paths.remove(&i);

        if self.tile_known(c).is_some() && !was_known {
            self.reason.set(Loud(reason));
        } else {
            self.reason.set(Quiet(reason));
        }
    }

    // Acknowledge a contradiction
    pub fn contradict(&mut self) {
        self.reason = ReasonKind::Contradiction;
    }

    pub fn set_land(&mut self, reason: Reason, c: Coord) {
        use Possibility::*;
        use ReasonKind::*;
        if !self.known_land(c) {
            self.reason = Loud(reason);
            self.get_mut(c).remove(&Sea);
            for p in self.get(c).clone() {
                let Isle(i) = p else {
                    continue;
                };
                self.island_paths.remove(&i);
            }
        }
    }

    pub fn set_sea(&mut self, reason: Reason, c: Coord) {
        use Possibility::*;
        use ReasonKind::*;
        if !self.known_sea(c) {
            self.reason = Loud(reason);
            self.get_mut(c).retain(|p| p == &Sea);
            for p in self.get(c).clone() {
                let Isle(i) = p else {
                    continue;
                };
                self.island_paths.remove(&i);
            }
        }
    }

    pub fn raise_depth_limit(&mut self) {
        self.max_depth += 1;
    }

    pub fn bifurcate(&mut self) -> Option<Self> {
        use ReasonKind::*;
        if self.depth < self.max_depth {
            let mut copy = self.clone();
            copy.depth += 1;
            Some(copy)
        } else {
            self.reason = MaxDepthReached;
            None
        }
    }

    pub fn grid(&self) -> &Grid<Set<Possibility>> {
        &self.possibilities
    }

    pub fn solved(&self) -> bool {
        use ReasonKind::*;
        self.reason != Contradiction
            && self.reason != MaxDepthReached
            && self.possibilities.iter().flatten().all(|s| s.len() == 1)
    }

    pub fn take_reason(&mut self) -> ReasonKind {
        use ReasonKind::*;
        let reason = self.reason;
        match reason {
            Loud(_) | Quiet(_) => {
                self.reason = Nil;
                if self.depth == 0 {
                    self.max_depth = 0;
                }
            }
            _ => (),
        }
        reason
    }

    pub fn island_paths(&mut self, island: Island) -> &Vec<Area> {
        if !self.island_paths.contains_key(&island) {
            let paths = enumerate_island_paths(self, island).collect();
            self.island_paths.insert(island, paths);
        }
        &self.island_paths[&island]
    }
}

impl ReasonKind {
    pub fn set(&mut self, other: Self) {
        use ReasonKind::*;
        if let (Nil | Quiet(_), Loud(r)) = (*self, other) {
            *self = Loud(r)
        };
    }

    pub fn is_set(&self) -> bool {
        use ReasonKind::*;
        matches!(self, Loud(_) | Quiet(_) | Contradiction)
    }
}
