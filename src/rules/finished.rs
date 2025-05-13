use super::*;

pub fn finished(board: &Board) -> Option<Update> {
    let (h, w) = board.dims();

    let mut update = Update::new(Justification::Finished);
    for r in 0..h {
        for c in 0..w {
            let Some(Island { n, .. }) = board.get_island((r, c)) else {
                continue;
            };

            let (area, _) = island(board, (r, c));

            if area.len() == n {
                for n in surrounding(board, area) {
                    update.set_sea(n);
                }
            }
        }
    }

    update.check(board)
}
