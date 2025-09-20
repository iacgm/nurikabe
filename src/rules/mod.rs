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
mod reachability;
mod sea_complete;
mod sea_trapped;

use all_paths_border::*;
use all_paths_intersect::*;
use connects_edges::*;
use cornered::*;
use distance::*;
use finished::*;
use one_way::*;
use pool::*;
use reachability::*;
use sea_complete::*;
use sea_trapped::*;

pub type Rule = fn(&mut Knowledge);

pub const RULES: &[Rule] = &[
    sea_complete,
    finished,
    cornered,
    one_way,
    trapped,
    reachability,
    connects_edges,
    pool,
    distance,
    all_paths_intersect,
    all_paths_border,
];

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Reason {
    SeaComplete,
    ConnectsEdges,
    TouchesIslands,
    Trapped,
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

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Reason::*;
        let reason = match self {
            TooFar => "Too far from any island",
            Unreachable => "Not reachable by any island",
            Pool => "L-Corner",
            Finished => "Island Complete",
            OneWayOut => "Only one way for paths to go",
            BruteForce => "Brute Force",
            TouchesIslands => "Borders islands",
            Trapped => "Sea must be contiguous",
            ConnectsEdges => "Connects edges",
            AllPathsIntersect => "All possibilities overlap",
            AllPathsBorder => "All possibilities border this",
            SeaComplete => "Sea is complete",
        };

        write!(f, "{}", reason)
    }
}
