use nurikabe::*;

fn main() -> std::io::Result<()> {
    let islands = vec![
        (0, 2, 2),
        (0, 5, 5),
        (1, 4, 2),
        (2, 1, 3),
        (3, 3, 3),
        (5, 2, 5),
        (5, 7, 3),
        (6, 0, 4),
        (9, 2, 2),
        (9, 4, 3),
        (10, 5, 5),
    ];

    let small_board = Board::from_islands(11, 8, islands.into_iter().map(Island::from));

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(small_board).run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
