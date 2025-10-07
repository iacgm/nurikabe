use nurikabe::*;

fn main() -> std::io::Result<()> {
    let islands = vec![
        (0, 0, 2),
        (0, 2, 2),
        (0, 4, 4),
        (3, 0, 2),
        (3, 3, 4),
        (3, 6, 1),
        (3, 6, 1),
        (4, 5, 1),
        (5, 4, 1),
        (6, 0, 1),
        (6, 6, 1),
        (7, 2, 1),
        (8, 3, 4),
        (8, 0, 1),
        (9, 2, 1),
        (10, 0, 1),
        (10, 5, 3),
        (11, 1, 1),
        (12, 2, 4),
        (12, 5, 1),
        (13, 0, 1),
        (13, 6, 3),
        (14, 1, 1),
        (16, 0, 1),
        (16, 2, 2),
        (16, 6, 2),
    ];

    let small_board = Board::from_islands(17, 7, islands.into_iter().map(Island::from));

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(small_board).run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
