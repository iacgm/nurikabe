use std::io;

use ratatui::DefaultTerminal;

use super::*;

use super::solver_screen::*;

pub enum UI {
    Solver(SolverScreen),
    Exit,
}

impl UI {
    pub fn solver(board: Board) -> Self {
        Self::Solver(SolverScreen::new(board))
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        use UI::*;
        loop {
            self = match self {
                Solver(s) => s.run(terminal)?, 
                Exit => break,
            }
        };

        Ok(())
    }
}
