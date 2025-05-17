use std::fmt::Display;

use super::*;

mod all_paths_border;
mod all_paths_intersect;
mod connects_edges;
mod cornered;
mod distance;
mod finished;
mod one_way;
mod pool;
mod reachable;
mod sea_trapped;
mod sea_complete;

use all_paths_border::*;
use all_paths_intersect::*;
use connects_edges::*;
use cornered::*;
use distance::*;
use finished::*;
use one_way::*;
use pool::*;
use reachable::*;
use sea_trapped::*;
use sea_complete::*;

pub type Rule = fn(&Annotation) -> Option<Update>;

pub const RULES: &[Rule] = &[
    sea_complete,
    finished,
    reachability,
    cornered,
    one_way,
    sea_trapped,
    connects_edges,
    pool,
    distance,
    all_paths_intersect,
    all_paths_border,
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
    SeaComplete,
    ConnectsEdges,
    Cornered,
    SeaTrapped,
    TooFar,
    Unreachable,
    Pool,
    Finished,
    OneWayOut,
    AllPathsIntersect,
    AllPathsBorder,
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
            AllPathsIntersect => "All possibilities overlap",
            AllPathsBorder => "All possibilities border this",
            SeaComplete => "Sea is complete",
        };

        write!(f, "{}", reason)
    }
}
