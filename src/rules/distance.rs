use super::*;

pub fn distance(note: &mut Knowledge) {
    let board = note.board();

    for ((r, c), tile) in board.iter() {
        if tile != Empty {
            continue;
        }

        let mut too_far = true;
        for island in &board.islands {
            let Island { r: ri, c: ci, n } = *island;

            let dr = r.abs_diff(ri);
            let dc = c.abs_diff(ci);

            if dr + dc <= n - 1 {
                too_far = false;
                break;
            }
        }

        if too_far {}
    }
}
