use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListState},
};

use super::*;

pub struct SolverScreen {
    solution: Solution,
    list_state: ListState,
    alive: bool,
}

impl SolverScreen {
    pub fn new(board: Board) -> Self {
        Self {
            solution: search_solution(board),
            list_state: ListState::default().with_selected(Some(0)),
            alive: true,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<UI> {
        while self.alive {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(UI::Exit)
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        let Event::Key(event) = event::read()? else {
            return Ok(());
        };

        if event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match event.code {
            KeyCode::Esc => {
                self.alive = false;
            }
            KeyCode::Up => {
                self.list_state.select_previous();
            }
            KeyCode::Down => {
                self.list_state.select_next();
            }
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        use Constraint::*;

        let area = center(frame.area(), Percentage(80), Percentage(80));

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Fill(1), Length(1), Fill(1)])
            .split(area);

        let title = Line::from("Justifications".bold().blue());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);

        let mut list_entries = vec!["Initial State".into()];
        list_entries.extend(self.solution.logic.iter().map(|j| format!("{}", j)));

        let list = List::new(list_entries)
            .block(block)
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, layout[0], &mut self.list_state);

        let board = &self.solution.states[self.list_state.selected().unwrap()];

        frame.render_widget(board, layout[2]);
    }
}
