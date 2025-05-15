use super::*;

pub fn sea_trapped( note: &Annotation) -> Option<Update> {
    let mut update = Update::new(Justification::SeaTrapped);

    let board = note.board;
    let (h, w) = board.dims();
    for r in 0..h {
        for c in 0..w {
            let coord = (r, c);
            let tile = board[coord];

            if tile != Empty {
                continue;
            }

            let area = area(board, coord);
            let surrounding = surrounding(board, &area);

            let trapped = surrounding.iter().all(|coord| board[*coord] == Land);
            let mut islands = surrounding
                .iter()
                .filter_map(|&(r, c)| note.island((r, c)))
                .collect::<Vec<_>>();

            islands.sort();
            islands.dedup();

            if trapped && islands.len() == 1 {
                for &t in &area {
                    update.set(t, Land);
                }
            }
        }
    }

    update.check(board)
}
