use super::*;

pub fn finished(knowledge: &mut Knowledge) {
    use Possibility::*;
    let board = knowledge.board();
    for (coord, t) in board.iter() {
        if t != Land {
            continue;
        }

        let Some(Isle(Island { n, .. })) = knowledge.if_known(coord) else {
            continue;
        };

        let area = area(&board, coord);

        if area.len() == n {
            for n in surrounding(&board, &area) {
                knowledge.set_sea(Reason::Finished, n);
            }
        }
    }
}
