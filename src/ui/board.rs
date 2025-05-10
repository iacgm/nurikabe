use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use super::*;

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Board".bold().blue());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);

        let mut lines = vec![];

        for row in &self.tiles {
            let mut line = vec![];
            for tile in row {
                let text = if let Tile::Number(n) = tile {
                    format!("{:2}", n)
                } else {
                    format!("  ")
                };
                line.push(text.bg(tile.color()));
            }
            lines.push(Line::from(line));
        }

        let inside = Text::from(lines);

        block.render(area, buf);

        let (h, w) = self.dims();
        let (h, w) = (h + 2, 2 * w + 2);
        let area = center(area, Constraint::Length(w as u16), Constraint::Length(h as u16));

        let board_block = Block::bordered().border_set(border::ROUNDED);

        Paragraph::new(inside)
            .centered()
            .block(board_block)
            .render(area, buf);
    }
}
