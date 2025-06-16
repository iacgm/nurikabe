use nurikabe::*;

fn main() -> std::io::Result<()> {
    let islands = vec![
        (0, 3, 4),
        (1, 7, 9),
        (2, 3, 1),
        (2, 6, 2),
        (3, 2, 5),
        (4, 4, 1),
        (7, 7, 2),
        (7, 9, 5),
        (8, 3, 3),
        (8, 5, 2),
        (9, 2, 6),
        (9, 6, 2),
        (10, 0, 2),
        (10, 4, 2),
        (11, 2, 2),
        (12, 4, 6),
        (13, 0, 4),
    ];

    let small_board = Board::from_islands(14, 10, islands.into_iter().map(Island::from));

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(small_board).run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}

