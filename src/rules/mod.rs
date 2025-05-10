use std::fmt::Display;

use super::*;

mod distance;
mod pool;
mod finished;

use distance::*;
use pool::*;
use finished::*;

pub enum Rule {
    Distance,
    Pool,
    Finished,
}

impl Rule {
    pub fn apply(&self, board: &mut Board) -> Option<Justification> {
        use Rule::*;
        match self {
            Distance => distance(board),
            Pool => pool(board),
            Finished => finished(board),
        }
    }
}

#[derive(Debug)]
pub enum Justification {
    Unreachable,
    Pool,
    Finished,
}

impl Display for Justification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Justification::*;
        match self {
            Unreachable => write!(f, "Unreachable"),
            Pool => write!(f, "L-Corner"),
            Finished => write!(f, "Island Complete"),
        }
    }
}
