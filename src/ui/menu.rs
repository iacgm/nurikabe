use std::io;

use ratatui::DefaultTerminal;

use super::*;

use super::solver_screen::*;

pub enum UI {
    Entry(EntryScreen),
    Solver(SolverScreen),
    Exit,
}

impl UI {
    pub fn entry() -> Self {
        Self::Entry(EntryScreen::new())
    }

    pub fn solver(board: Board) -> Self {
        Self::Solver(SolverScreen::new(board))
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        use UI::*;
        loop {
            self = match self {
                Entry(s) => s.run(terminal)?,
                Solver(s) => s.run(terminal)?,
                Exit => break,
            }
        }

        Ok(())
    }
}
