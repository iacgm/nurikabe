use nurikabe::*;

fn main() -> std::io::Result<()> {
    let settings = GenSettings {
        dims: (7, 7),
        max_island_size: 7,
        branch_factor: 2,
        max_island_count: 8,
    };

    let start = std::time::Instant::now();
    let board = loop {
        if let Some(board) = try_generate(settings) {
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
