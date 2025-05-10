use super::*;

use Rule::*;
const RULES: &[Rule] = &[Distance, Pool, Finished];

pub struct Solution {
    pub states: Vec<Board>,
    pub logic: Vec<Justification>,
}

pub fn search_solution(board: Board) -> Solution {
    let mut state = board.clone();
    let mut states = vec![board];
    let mut logic = vec![];

    'solve: loop {
        for rule in RULES {
            if let Some(justification) = rule.apply(&mut state) {
                states.push(state.clone());
                logic.push(justification);

                continue 'solve;
            }
        }

        break Solution { states, logic };
    }
}
