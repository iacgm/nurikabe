use nurikabe::*;

fn main() -> std::io::Result<()> {
    let settings = BoardGenSettings {
        dims: (12, 12),
        power: 0.65,
        max_island_size: 6,
        branch_factor: 2,
        max_attempts: 1000,
        label_attempts: 20,
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
