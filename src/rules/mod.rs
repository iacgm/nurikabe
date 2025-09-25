use std::fmt::Display;

use super::*;

mod all_paths_border;
mod all_paths_intersect;
mod borders_multiple;
mod connects_edges;
mod corner;
mod distance;
mod finished;
mod guess;
mod impossible;
mod island_contra;
mod no_space;
mod noncontiguous;
mod one_way;
mod pools;
mod reachability;
mod sea_complete;
mod sea_trapped;
mod wall_trick;

use all_paths_border::*;
use all_paths_intersect::*;
use borders_multiple::*;
use connects_edges::*;
use corner::*;
use distance::*;
use finished::*;
use guess::*;
use impossible::*;
use island_contra::*;
use no_space::*;
use noncontiguous::*;
use one_way::*;
use pools::*;
use reachability::*;
use sea_complete::*;
use sea_trapped::*;
use wall_trick::*;

pub type Rule = fn(&mut Knowledge);

pub const RULES: &[Rule] = &[
    // Contradiction rules
    pools,
    noncontiguous,
    impossible,
    no_space,
    // Deduction rules
    sea_complete,
    finished,
    cornered,
    borders_multiple,
    one_way,
    trapped,
    connects_edges,
    distance,
    reachability,
    all_paths_intersect,
    all_paths_border,
    wall_trick,
    // Resort to trial & error
    island_contra,
    guess,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    WallTrick,
    AllPathsIntersect,
    AllPathsBorder,
    ByContradiction(usize),
    Bifurcation,
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Reason::*;
        let reason = match self {
            TooFar => "No island near enough",
            Unreachable => "Unreachable square",
            Pool => "L-Corner",
            Finished => "Island completed",
            OneWayOut => "Only one way to go",
            WallTrick => "Wall pattern",
            TouchesIslands => "Borders separate islands",
            Trapped => "Sea must be contiguous",
            ConnectsEdges => "Connects edges",
            AllPathsIntersect => "Island must pass square",
            AllPathsBorder => "Island must border square",
            SeaComplete => "Sea complete",
            ByContradiction(l) => return write!(f, "Contradiction in {} steps", l),
            Bifurcation => "Guess",
        };

        write!(f, "{}", reason)
    }
}
