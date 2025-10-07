use nurikabe::*;

fn main() -> std::io::Result<()> {
    let islands = vec![(0, 0, 3)];

    let small_board = Board::from_islands(2, 2, islands.into_iter().map(Island::from));

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(small_board).run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
