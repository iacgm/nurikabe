use super::*;

pub fn finished(note: &Annotation) -> Option<Update> {
    let mut update = Update::new(Justification::Finished);
    let board = note.board;
    for (coord, t) in board.iter() {
        if t != Land {
            continue;
        }

        let Some(Island { n, .. }) = note.island(coord) else {
            continue;
        };

        let area = area(board, coord);

        if area.len() == n {
            for n in surrounding(board, &area) {
                update.set(n, Sea);
            }
        }
    }

    update.check(board)
}
