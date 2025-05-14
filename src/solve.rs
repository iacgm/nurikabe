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
    pub logic: Vec<Update>,
    pub solved: bool,
    pub time: f32,
}

pub fn search_solution(board: Board) -> Solution {
    let mut state = board.clone();
    let mut states = vec![board];
    let mut logic = vec![];

    let start = Instant::now();
    'solve: loop {
        for rule in RULES {
            if let Some(update) = rule(&state) {
                states.push(state.clone());
                update.apply_to(&mut state);
                logic.push(update);

                continue 'solve;
            }
        }

        // Push final state, so we have completed board at the end
        let solved = state.solved();
        states.push(state);

        let time = Instant::now().duration_since(start).as_secs_f32();
        break Solution {
            states,
            logic,
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
            format!("{}", self.logic.len()).bold().blue(),
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
