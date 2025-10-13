use nurikabe::*;

fn main() -> std::io::Result<()> {
    let settings = BoardGenSettings {
        dims: (12, 12),
        mean_island_size: 3,
        max_island_size: 6,
        branch_factor: 3,
        max_attempts: 100,
        label_attempts: 150,
        max_depth: 1,
        max_amends: 8,
    };

    let start = std::time::Instant::now();
    let board = loop {
        dbg!("Trying");
        if let Some(board) = gen_board(settings) {
            break board;
        }
    };
    let end = std::time::Instant::now();

    let mut terminal = ratatui::init();

    ui::menu::UI::solver(board.clone()).run(&mut terminal)?;

    ratatui::restore();

    println!("TTG: {}s", end.duration_since(start).as_secs_f32());

    Ok(())
}
