use nurikabe::*;

fn main() -> std::io::Result<()> {
    let islands = vec![
        Island { r: 2, c: 4, n: 1 },
        Island { r: 4, c: 4, n: 1 },
        Island { r: 2, c: 2, n: 3 },
        Island { r: 3, c: 5, n: 3 },
        Island { r: 4, c: 0, n: 3 },
        Island { r: 5, c: 3, n: 3 },
        Island { r: 6, c: 7, n: 4 },
        Island { r: 0, c: 1, n: 5 },
        Island { r: 0, c: 4, n: 6 },
    ];

    let small_board = Board::from_islands(8, 8, islands.into_iter());

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(small_board).run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
