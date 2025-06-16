use nurikabe::*;

fn main() -> std::io::Result<()> {
    let islands = vec![
        (1, 0, 8),
        (2, 1, 1),
        (2, 7, 2),
        (3, 3, 4),
        (3, 9, 7),
        (5, 0, 1),
        (5, 4, 4),
        (5, 7, 4),
        (6, 6, 3),
        (8, 2, 4),
        (8, 9, 2),
        (9, 4, 6),
        (11, 0, 8),
        (11, 2, 2),
        (11, 4, 5),
        (11, 7, 2),
        (12, 8, 3),
    ];

    let small_board = Board::from_islands(14, 10, islands.into_iter().map(Island::from));

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(small_board).run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
