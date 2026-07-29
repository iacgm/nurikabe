use nurikabe::*;
use std::io::*;

fn main() -> Result<()> {
    let fs = std::fs::read_to_string("puzzles.b64l")?;
    let mut file = std::fs::File::create("slugs.b64l")?;

    for line in fs.lines().map(|s| s.trim()) {
        writeln!(file, "{}", Board::from_b64(line).slug())?;
    }

    Ok(())
}
