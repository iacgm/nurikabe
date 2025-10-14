use std::time::Instant;

use ratatui::{
    buffer::Buffer,
    prelude::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use super::*;

#[derive(Clone)]
pub struct Solution {
    pub unique: bool,
    pub contradiction: bool,
    pub states: Vec<Board>,
    pub reasons: Vec<Reason>,
    pub solved: bool,
    pub time: f32,
}

impl Solution {
    pub fn steps(&self) -> usize {
        use Reason::*;
        self.reasons
            .iter()
            .map(|r| if let ByContradiction(n) = r { n + 1 } else { 1 })
            .sum()
    }

    pub fn forced_board(&self) -> &Board {
        use Reason::*;
        self.states
            .iter()
            .enumerate()
            // Reasons and boards are offset by 1
            .take_while(|(i, _)| {
                *i == 0 || *i == self.states.len() - 1 || self.reasons[*i - 1] != Bifurcation
            })
            .last()
            .unwrap()
            .1
    }
}

pub fn solve(board: &Board) -> Solution {
    let mut knowledge = Knowledge::new(board);

    solve_knowing(&mut knowledge)
}

pub fn solve_with_limits(board: &Board, max_depth: usize) -> Solution {
    let mut knowledge = Knowledge::new(board);

    knowledge.depth_limit = 0;
    knowledge.raise_max = Some(max_depth);

    solve_knowing(&mut knowledge)
}

pub fn solve_knowing(known: &mut Knowledge) -> Solution {
    use ReasonKind::*;
    
    let board = known.board();
    let mut states = vec![board.clone()];
    let mut reasons = vec![];

    let start = Instant::now();
    'solve: loop {
        let board = known.board();

        for rule in RULES {
            rule(known, &board);
            let reason = known.take_reason();

            match reason {
                MaxDepthReached => {
                    if let Some(max) = known.raise_max && max <= known.depth_limit {
                        break
                    } else {
                        known.raise_depth_limit();
                        known.reason = Nil;
                        continue 'solve;
                    }
                }
                Contradiction => break,
                Loud(reason) => {
                    states.push(known.board());
                    reasons.push(reason);
                    continue 'solve;
                }
                Quiet(_) => {
                    continue 'solve;
                }
                Nil => (),
            }
        }

        // Push final state, so we have completed board at the end
        let board = known.board();
        let solved = known.solved();
        states.push(board);

        let time = Instant::now().duration_since(start).as_secs_f32();
        break Solution {
            states,
            reasons,
            solved,
            contradiction: known.reason == Contradiction,
            time,
            unique: known.unique,
        };
    }
}

impl Widget for &Solution {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Solution Info".blue().bold());

        let solved_line = if self.solved && !self.unique {
            "Solution found, but it is not unique".yellow().bold()
        } else if self.solved
            && !self
                .reasons
                .iter()
                .any(|s| matches!(s, Reason::ByContradiction(_)))
        {
            "Solution found without guesses".green().bold()
        } else if self.solved {
            "Solved with guesses".light_yellow().bold()
        } else if self.contradiction {
            "Board is unsolvable".red().bold()
        } else {
            // This should happen only if the puzzle has zero solutions
            "Max search depth reached".red().bold()
        };

        let length_line = Line::from(vec![
            "Number of steps: ".into(),
            format!("{}", self.states.len() - 1).bold().blue(),
        ]);

        let time_line = Line::from(vec![
            "Time to solve (s): ".into(),
            format!("{}", self.time).bold().blue(),
        ]);

        let info = Line::from(solved_line);

        let block = Block::bordered()
            .title(title.centered())
            .padding(Padding::horizontal(1))
            .border_set(border::ROUNDED);

        Paragraph::new(vec![info, length_line, time_line])
            .block(block)
            .render(area, buf)
    }
}
