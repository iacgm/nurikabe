use nurikabe::*;

fn main() -> std::io::Result<()> {
    let islands = vec![
        (3, 0, 5),
        (9, 0, 1),
        (2, 1, 4),
        (4, 2, 3),
        (8, 2, 2),
        (5, 3, 2),
        (7, 3, 4),
        (0, 6, 2),
        (3, 6, 7),
    ];

    let small_board = Board::from_islands(10, 7, islands.into_iter().map(Island::from));

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(small_board).run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
