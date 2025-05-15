use std::fmt::Display;

use super::*;

mod connects_edges;
mod cornered;
mod distance;
mod finished;
mod one_way;
mod pool;
mod reachable;
mod sea_trapped;

use connects_edges::*;
use cornered::*;
use distance::*;
use finished::*;
use one_way::*;
use pool::*;
use reachable::*;
use sea_trapped::*;

pub type Rule = fn(&Annotation) -> Option<Update>;

pub const RULES: &[Rule] = &[
    finished,
    reachability,
    cornered,
    one_way,
    sea_trapped,
    connects_edges,
    pool,
    distance,
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
    }

    pub fn set(&mut self, coord: Coord, tile: Tile) {
        let v = match tile {
            Sea => &mut self.sea,
            Land => &mut self.land,
            _ => unreachable!(),
        };

        if !v.contains(&coord) {
            v.push(coord)
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
    ConnectsEdges,
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
        let reason = match self {
            TooFar => "Too far from any island",
            Unreachable => "Not reachable by any island",
            Pool => "L-Corner",
            Finished => "Island Complete",
            OneWayOut => "Only one way for paths to go",
            BruteForce => "Brute Force",
            Cornered => "Touches 2 islands",
            SeaTrapped => "Sea must be contiguous",
            ConnectsEdges => "Connects edges",
        };

        write!(f, "{}", reason)
    }
}
