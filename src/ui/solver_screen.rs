use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListState},
    DefaultTerminal, Frame,
};

use crate::ui::board::Diff;

use super::*;

pub struct SolverScreen {
    solution: Solution,
    list_state: ListState,
    alive: bool,
}

impl SolverScreen {
    pub fn new(board: Board) -> Self {
        Self {
            solution: solve_with_limits(&board, 1),
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

        use KeyCode::*;
        match event.code {
            Esc => {
                self.alive = false;
            }
            Up | Char('k') => {
                self.list_state.select_previous();
            }
            Down | Char('j') => {
                self.list_state.select_next();
            }
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        use Constraint::*;

        let area = center(frame.area(), Percentage(80), Percentage(80));

        let [solution_area, _, board_area] =
            Layout::horizontal([Fill(3), Length(1), Fill(5)]).areas(area);

        let title = Line::from("Justification".bold().blue());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);

        let mut list_entries = vec!["Initial Board".into()];
        list_entries.extend(
            self.solution
                .reasons
                .iter()
                .enumerate()
                .map(|(i, r)| format!("{:2}: {}", i + 1, r)),
        );
        list_entries.push("Final Board".into());

        let [proof_area, solution_info_area] =
            Layout::vertical([Fill(6), Length(6)]).areas(solution_area);

        frame.render_widget(&self.solution, solution_info_area);

        let proof_step = List::new(list_entries)
            .block(block)
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(proof_step, proof_area, &mut self.list_state);

        let selected = self.list_state.selected().unwrap();
        let board = &self.solution.states[selected];

        // Display initial state if no Update can be displayed
        if selected == 0 || selected == self.solution.states.len() - 1 {
            frame.render_widget(board, board_area);
        } else {
            let prev = &self.solution.states[selected - 1];
            frame.render_widget(Diff(board, prev), board_area);
        }
    }
}
