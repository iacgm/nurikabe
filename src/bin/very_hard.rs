use nurikabe::*;

fn main() -> std::io::Result<()> {
    let islands = vec![
        (0, 2, 7),
        (1, 5, 7),
        (4, 5, 2),
        (4, 7, 5),
        (5, 4, 4),
        (5, 9, 6),
        (6, 6, 8),
        (7, 4, 7),
        (10, 3, 8),
        (11, 0, 2),
        (11, 4, 2),
        (11, 9, 3),
        (12, 1, 3),
        (13, 4, 4),
    ];

    let small_board = Board::from_islands(14, 10, islands.into_iter().map(Island::from));

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(small_board).run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
