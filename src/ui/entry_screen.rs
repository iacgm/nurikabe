use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use super::*;

pub struct EntryScreen {
    board: Board,
    cursor: Coord,
    cursor_clock: Instant,
}

impl EntryScreen {
    pub fn new() -> Self {
        Self {
            board: Board::empty(8, 6),
            cursor: (2, 2),
            cursor_clock: Instant::now(),
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<UI> {
        let mut next = None;
        while next.is_none() {
            terminal.draw(|frame| self.draw(frame))?;
            next = self.handle_events()?;
        }

        Ok(next.unwrap())
    }

    pub fn handle_events(&mut self) -> io::Result<Option<UI>> {
        if !event::poll(Duration::from_secs_f32(0.05))? {
            return Ok(None);
        }

        let Event::Key(event) = event::read()? else {
            return Ok(None);
        };

        if event.kind != KeyEventKind::Press {
            return Ok(None);
        }

        let (h, w) = self.board.dims();

        use KeyCode::*;
        match event.code {
            Esc | Char('q') => {
                return Ok(Some(UI::Exit));
            }
            Up | Char('k') => {
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1;
                }
            }
            Down | Char('j') => {
                if self.cursor.0 < h - 1 {
                    self.cursor.0 += 1;
                }
            }
            Left | Char('h') => {
                if self.cursor.1 > 0 {
                    self.cursor.1 -= 1;
                }
            }
            Right | Char('l') => {
                if self.cursor.1 < w - 1 {
                    self.cursor.1 += 1;
                }
            }
            Char(c @ ('+' | '_' | '-' | '=')) => {
                let (h, w) = self.board.dims();

                let (h, w) = match c {
                    '+' => (h + 1, w),
                    '_' if h > 1 => (h - 1, w),
                    '=' => (h, w + 1),
                    '-' if w > 1 => (h, w - 1),
                    _ => (h, w),
                };

                let (x, y) = self.cursor;

                self.cursor = (x.clamp(0, h - 1), y.clamp(0, w - 1));

                self.board.resize((h, w));
            }
            Char(' ') => {
                self.cursor_clock = Instant::now();
                self.board.remove_island(self.cursor);
                let tile = &mut self.board[self.cursor];
                *tile = match tile {
                    Empty => Sea,
                    Sea => Land,
                    Land => Empty,
                };
            }
            Char(c @ '0'..'9') => {
                self.cursor_clock = Instant::now();
                let n = c.to_digit(10).unwrap() as usize;

                let (r, c) = self.cursor;
                if let Some(island) = self.board.mut_island((r, c)) {
                    island.n %= 100;
                    island.n *= 10;
                    island.n += n;
                } else {
                    self.board.add_island(Island { r, c, n })
                }
            }
            Backspace => {
                self.cursor_clock = Instant::now();
                if let Some(island) = self.board.mut_island(self.cursor) {
                    island.n /= 10;
                    if island.n == 0 {
                        self.board.remove_island(self.cursor)
                    }
                }
            }
            // Jeez. This menu thing is getting a bit unwieldy...
            // If it ain't broke...
            Enter => {
                return Ok(Some(UI::solver(self.board.clone())));
            }
            _ => (),
        }

        Ok(None)
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }
}

impl Widget for &EntryScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let area = center(area, Percentage(80), Percentage(80));

        let [board_area, instruction_area] = Layout::vertical([Fill(1), Length(4)]).areas(area);

        self.board.render(board_area, buf);
        let (h, w) = self.board.dims();

        // Render cursor
        if Instant::now()
            .duration_since(self.cursor_clock)
            .as_secs_f32()
            % 1.5
            > 0.75
        {
            let inner_board_area = center(
                board_area,
                Constraint::Length(2 * w as u16),
                Constraint::Length(h as u16),
            );
            let cursor_area = Rect {
                height: 1,
                width: 2,
                x: inner_board_area.x + 2 * self.cursor.1 as u16,
                y: inner_board_area.y + self.cursor.0 as u16,
            };

            buf.set_style(cursor_area, Style::new().bg(Color::White));
        }

        let title = Line::from("Controls".bold().blue());
        let instructions = Line::from(vec![
            " Move:".into(),
            " ←↑↓→ / hjkl ".bold().blue(),
            " Cycle Tile:".into(),
            " <Space> ".bold().blue(),
            " Solve:".into(),
            " <Enter> ".bold().blue(),
            " Edit Island:".into(),
            " 0-9/<Backspace> ".bold().blue(),
            " Resize:".into(),
            " -=_+ ".bold().blue(),
        ]);

        let size_info = Line::from(vec![
            " Height: ".into(),
            format!("{} ", h).red().bold(),
            " Width: ".into(),
            format!("{} ", w).red().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);
        let instructions = Paragraph::new(vec![instructions, size_info])
            .centered()
            .block(block);

        instructions.render(instruction_area, buf);
    }
}
