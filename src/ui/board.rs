use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use super::*;

pub struct Diff<'a>(pub &'a Board, pub &'a Board); // (Current, previous)

impl<'a> Widget for Diff<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let board = self.0;
        let prev = self.1;

        let title = Line::from("Board".bold().blue());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);

        let mut lines = vec![];

        for (r, row) in board.tiles.iter().enumerate() {
            let mut line = vec![];
            for (c, tile) in row.iter().enumerate() {
                let coord = (r, c);
                let color = if board[coord] == Water && prev[coord] == Empty {
                    Color::Cyan
                } else if board[coord] == Land && prev[coord] == Empty {
                    Color::LightGreen
                } else {
                    tile.color()
                };

                let text = match board.lookup_island((r, c)) {
                    Some(Island { n, .. }) => {
                        format!("{:2}", n)
                    }
                    _ => "  ".to_string(),
                };
                line.push(text.bg(color));
            }
            lines.push(Line::from(line));
        }

        let inside = Text::from(lines);

        block.render(area, buf);

        let (h, w) = board.dims();
        let (h, w) = (h + 2, 2 * w + 2);
        let area = center(
            area,
            Constraint::Length(w as u16),
            Constraint::Length(h as u16),
        );

        let board_block = Block::bordered().border_set(border::ROUNDED);

        Paragraph::new(inside)
            .centered()
            .block(board_block)
            .render(area, buf);
    }
}

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Diff(self, self).render(area, buf)
    }
}
