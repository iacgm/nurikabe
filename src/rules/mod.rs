use std::fmt::Display;

use super::*;

mod cornered;
mod distance;
mod finished;
mod one_way;
mod pool;
mod reachable;
mod sea_trapped;

use cornered::*;
use distance::*;
use finished::*;
use one_way::*;
use pool::*;
use reachable::*;
use sea_trapped::*;

pub type Rule = fn(&Board) -> Option<Update>;
pub const RULES: &[Rule] = &[
    cornered,
    finished,
    sea_trapped,
    one_way,
    pool,
    distance,
    reachability,
];

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
        for &coord in &self.sea {
            board[coord] = Sea;
        }

        for &coord in &self.land {
            board[coord] = Land;
        }

        for &coord in &self.land {
            let area = area(board, coord);
            let island = area.iter().find_map(|&c| board.island_map[c.0][c.1]);
            for (r, c) in area {
                board.island_map[r][c] = island;
            }
        }
    }

    pub fn set_sea(&mut self, (r, c): Coord) {
        if !self.sea.contains(&(r, c)) {
            self.sea.push((r, c));
        }
    }

    pub fn set_land(&mut self, (r, c): Coord) {
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
    Cornered,
    SeaTrapped,
    TooFar,
    Unreachable,
    Pool,
    Finished,
    OneWayOut,
    #[default]
    BruteForce,
}

impl Display for Justification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Justification::*;
        match self {
            TooFar => write!(f, "Too far from any island"),
            Unreachable => write!(f, "Not reachable by any island"),
            Pool => write!(f, "L-Corner"),
            Finished => write!(f, "Island Complete"),
            OneWayOut => write!(f, "Only one way for island to go"),
            BruteForce => write!(f, "Brute Force"),
            Cornered => write!(f, "Touches 2 islands"),
            SeaTrapped => write!(f, "Sea must be contiguous"),
        }
    }
}
