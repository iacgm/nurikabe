use ratatui::{
    Frame,
    buffer::Buffer,
    prelude::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use super::*;

pub struct Solution {
    pub states: Vec<Board>,
    pub logic: Vec<Update>,
    pub solved: bool,
}

pub fn search_solution(board: Board) -> Solution {
    let mut state = board.clone();
    let mut states = vec![board];
    let mut logic = vec![];

    'solve: loop {
        for rule in RULES {
            if let Some(update) = rule(&state) {
                states.push(state.clone());
                update.apply_to(&mut state);
                logic.push(update);

                continue 'solve;
            }
        }

        break Solution {
            states,
            logic,
            solved: state.is_finished(),
        };
    }
}

impl Widget for &Solution {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Solution Info".bold().bold());

        let solved_line = if self.solved {
            "Full solution found:".green().bold()
        } else {
            "No solution found:".red().bold()
        };

        let length_line = Line::from(vec![
            "# of steps: ".into(),
            format!("{}", self.logic.len()).bold().blue(),
        ]);

        let info = Line::from(solved_line);

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);

        Paragraph::new(vec![info, length_line])
            .block(block)
            .render(area, buf)
    }
}
