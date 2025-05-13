use std::fmt::Display;

use super::*;

mod utils;
mod distance;
mod finished;
mod pool;
mod reachable;
mod one_way;

pub use utils::*;

use distance::*;
use finished::*;
use pool::*;
use reachable::*;
use one_way::*;

pub type Rule = fn(&Board) -> Option<Update>;
pub const RULES: &[Rule] = &[finished, distance, one_way, pool, reachability];

#[derive(Default)]
pub struct Update {
    pub justification: Justification,
    pub land: Vec<Coord>,
    pub sea: Vec<Coord>,
}

impl Update {
    pub fn new(justification: Justification) -> Self {
        Self {
            justification,
            land: vec![],
            sea: vec![],
        }
    }

    pub fn apply_to(&self, board: &mut Board) {
        for &(r, c) in &self.sea {
            board.tiles[r][c] = Sea;
        }
        for &(r, c) in &self.land {
            board.tiles[r][c] = Land;
        }
    }

    pub fn set_sea(&mut self, (r, c) : Coord) {
        if !self.sea.contains(&(r, c)) {
            self.sea.push((r, c));
        }
    }

    pub fn set_land(&mut self, (r, c) : Coord) {
        if !self.land.contains(&(r, c)) {
            self.land.push((r, c));
        }
    }

    pub fn check(mut self, board: &Board) -> Option<Self> {
        self.land.retain(|&(r, c)| board.tiles[r][c] != Land);
        self.sea.retain(|&(r, c)| board.tiles[r][c] != Sea);


        if self.land.is_empty() && self.sea.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

#[derive(Default, Debug)]
pub enum Justification {
    TooFar,
    Unreachable,
    Pool,
    Finished,
    OneWayOut,
    #[default]
    BruteForce
}

impl Display for Justification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Justification::*;
        match self {
            TooFar => write!(f, "Too far from any island."),
            Unreachable => write!(f, "Not reachable by any island."),
            Pool => write!(f, "L-Corner"),
            Finished => write!(f, "Island Complete"),
            OneWayOut => write!(f, "Only one way out"),
            BruteForce => write!(f, "Brute Force"),
        }
    }
}
