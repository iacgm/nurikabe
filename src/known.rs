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
    pub depth_limit: usize, // Current depth limit. This can be raised (using Iterative deepening) up to raise_max
    pub raise_max: Option<usize>,
    pub reason: ReasonKind, // Gets disabled when we make a new change
    islands: Vec<Island>,
    dims: (usize, usize),
    possibilities: Vec<Set<Possibility>>, // None = Sea
    island_paths: Map<Island, Vec<Area>>,
}

impl Knowledge {
    pub fn new(board: &Board) -> Self {
        use Possibility::*;
        use ReasonKind::*;

        let (h, w) = board.dims();

        let mut possibility_space = Set::from_iter(board.islands.iter().copied().map(Isle));
        possibility_space.insert(Sea);

        let mut possibilities = vec![possibility_space.clone(); w * h];

        for &Island { r, c, n } in board.islands.iter() {
            let i = r * w + c;
            possibilities[i] = [Isle((r, c, n).into())].into_iter().collect();
        }

        for ((r, c), t) in board.iter() {
            let i = r * w + c;
            if t == Water {
                possibilities[i] = [Sea].into_iter().collect();
            } else if t == Land {
                possibilities[i].remove(&Sea);
            }
        }
        Self {
            depth: 0,
            depth_limit: 1,
            raise_max: Some(1),
            reason: Nil,
            islands: board.islands.clone(),
            // Initially assume any island could reach any tile
            dims: board.dims,
            possibilities,
            unique: true,
            island_paths: Default::default(),
        }
    }

    pub fn board(&self) -> Board {
        let (h, w) = self.dims;

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
        let (_, w) = self.dims;
        let i = r * w + c;
        &mut self.possibilities[i]
    }

    pub fn get(&self, (r, c): Coord) -> &Set<Possibility> {
        let (_, w) = self.dims;
        let i = r * w + c;
        &self.possibilities[i]
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
        let (_, w) = self.dims;
        let i = r * w + c;
        let is = &self.possibilities[i];
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
        if let Some(paths) = self.island_paths.get_mut(&i) {
            paths.retain(|p| !p.contains(&c));
        }

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

            for p in self.get(c).clone() {
                let Isle(i) = p else {
                    continue;
                };

                if let Some(paths) = self.island_paths.get_mut(&i) {
                    paths.retain(|p| !p.contains(&c));
                }
            }

            self.get_mut(c).retain(|p| p == &Sea);
        }
    }

    pub fn raise_depth_limit(&mut self) {
        self.depth_limit += 1;
    }

    pub fn bifurcate(&mut self) -> Option<Self> {
        use ReasonKind::*;
        if self.depth < self.depth_limit {
            let mut copy = self.clone();
            copy.raise_max = Some(0);
            copy.depth += 1;
            Some(copy)
        } else {
            self.reason = MaxDepthReached;
            None
        }
    }

    pub fn possibilities(&self) -> &Vec<Set<Possibility>> {
        &self.possibilities
    }

    pub fn solved(&self) -> bool {
        use ReasonKind::*;

        self.reason != Contradiction
            && self.reason != MaxDepthReached
            && self.possibilities.iter().all(|s| s.len() == 1)
    }

    pub fn take_reason(&mut self) -> ReasonKind {
        use ReasonKind::*;
        let reason = self.reason;
        match reason {
            Loud(_) | Quiet(_) => {
                self.reason = Nil;
                if self.depth == 0 {
                    self.depth_limit = 0;
                }
            }
            _ => (),
        }
        reason
    }

    pub fn island_paths(&mut self, island: Island) -> &Vec<Area> {
        if !self.island_paths.contains_key(&island) {
            let paths = enumerate_island_paths(self, island).collect();

            for path in &paths {
                for &t in path {
                    use Tile::*;
                    debug_assert!(self.tile_known(t) != Some(Water));
                }
            }

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
