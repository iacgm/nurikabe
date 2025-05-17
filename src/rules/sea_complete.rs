use super::*;

pub fn sea_complete(note: &Annotation) -> Option<Update> {
    let sea_size = note.board.iter().filter(|&(_, t)| t == Sea).count();
    let land_size = note.board.islands.iter().map(|i| i.n).sum::<usize>();
    let (h, w) = note.board.dims();

    if sea_size + land_size == h * w {
        let mut update = Update::new(Justification::SeaComplete);

        for (c, t) in note.board.iter() {
            if t != Empty {
                continue;
            }

            update.set(c, Land);
        }

        update.check(note.board)
    } else {
        None
    }
}
