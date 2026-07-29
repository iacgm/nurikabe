use nurikabe::*;

fn main() -> std::io::Result<()> {
    let fs = std::fs::read_to_string("puzzles.b64l")?;

    let mut terminal = ratatui::init();
    for line in fs.lines().map(|s| s.trim()) {
        let board = Board::from_b64(line);
        ui::menu::UI::solver(board).run(&mut terminal)?;
    }
    ratatui::restore();

    Ok(())
}
