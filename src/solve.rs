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

pub fn search_solution(board: Board) -> Solution {
    let mut states = vec![board.clone()];
    let mut reasons = vec![];

    let mut knowledge = Knowledge::new(&board);

    let start = Instant::now();
    'solve: loop {
        for rule in RULES {
            use Volume::*;

            rule(&mut knowledge);
            let reason = knowledge.reason.take();
            match reason {
                Loud(reason) => {
                    states.push(knowledge.board());
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
        let board = knowledge.board();
        let solved = board.solved();
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

        let solved_line = if self.solved {
            "Full solution found".green().bold()
        } else {
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
