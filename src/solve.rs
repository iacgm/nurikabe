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

pub struct Solution {
    pub states: Vec<Board>,
    pub reasons: Vec<Reason>,
    pub solved: bool,
    pub time: f32,
}

pub fn solve(board: Board) -> Solution {
    let mut knowledge = Knowledge::new(&board);

    solve_knowing(&mut knowledge)
}

pub fn solve_knowing(known: &mut Knowledge) -> Solution {
    let board = known.board();
    let mut states = vec![board.clone()];
    let mut reasons = vec![];

    let start = Instant::now();
    'solve: loop {
        for rule in RULES {
            use ReasonKind::*;

            rule(known);
            let reason = known.take_reason();
            match reason {
                MaxDepthReached if known.depth == 0 => {
                    known.raise_depth_limit();
                    known.reason = Nil;
                    continue 'solve;
                }
                MaxDepthReached => break,
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
            time,
        };
    }
}

impl Widget for &Solution {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Solution Info".blue().bold());

        let solved_line = if self.solved
            && !self
                .reasons
                .iter()
                .any(|s| matches!(s, Reason::ByContradiction(_)))
        {
            "Solution found without guessing".green().bold()
        } else if self.solved {
            "Solved with guesses".yellow().bold()
        } else {
            // This should never happen anymore
            "No solution found".red().bold()
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
