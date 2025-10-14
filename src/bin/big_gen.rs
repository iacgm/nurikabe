use nurikabe::*;

fn main() -> std::io::Result<()> {
    let islands = vec![
        Island { r: 0, c: 3, n: 3 },
        Island { r: 0, c: 7, n: 2 },
        Island { r: 1, c: 0, n: 1 },
        Island { r: 1, c: 8, n: 1 },
        Island { r: 2, c: 4, n: 3 },
        Island { r: 3, c: 0, n: 2 },
        Island { r: 4, c: 4, n: 2 },
        Island { r: 4, c: 6, n: 2 },
        Island { r: 4, c: 9, n: 3 },
        Island { r: 5, c: 2, n: 4 },
        Island { r: 6, c: 1, n: 2 },
        Island { r: 6, c: 3, n: 2 },
        Island { r: 6, c: 5, n: 1 },
        Island { r: 6, c: 7, n: 3 },
        Island { r: 7, c: 6, n: 1 },
        Island { r: 8, c: 1, n: 3 },
        Island { r: 8, c: 4, n: 2 },
        Island { r: 9, c: 6, n: 2 },
        Island { r: 10, c: 2, n: 2 },
        Island { r: 10, c: 9, n: 5 },
        Island { r: 11, c: 3, n: 2 },
        Island { r: 11, c: 7, n: 1 },
        Island { r: 12, c: 2, n: 1 },
        Island { r: 12, c: 6, n: 5 },
        Island { r: 13, c: 0, n: 3 },
    ];

    let small_board = Board::from_islands(14, 10, islands.into_iter());

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(small_board).run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
