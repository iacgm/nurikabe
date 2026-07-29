use nurikabe::*;

use std::io::*;

fn main() -> Result<()> {
    let settings = BoardGenSettings {
        dims: (12, 9),
        mean_island_size: 4,
        max_island_size: 6,
        branch_factor: 3,
        max_attempts: 100,
        label_attempts: 6,
        max_depth: 1,
        max_amends: 4,
    };

    let mut file = std::fs::File::create("puzzles7.b64l")?;

    for _ in 0..500 {
        let mut num = 0;
        let start = std::time::Instant::now();
        let board = loop {
            if let Some(board) = gen_board(settings) {
                break board;
            }
            num += 1;
        };
        let end = std::time::Instant::now();
        dbg!(num);

        let soln = solve(&board);
        if soln.solved && soln.unique {
            assert!(board == Board::from_b64(&board.b64()));
            writeln!(file, "{}", board.b64())?;
        }

        println!("TTG: {}s", end.duration_since(start).as_secs_f32());
    }

    Ok(())
}
