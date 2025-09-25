use super::*;

pub fn sea_complete(knowledge: &mut Knowledge, board: &Board) {
    let sea_size = board.iter().filter(|&(_, t)| t == Water).count();
    let land_size = board.islands.iter().map(|i| i.n).sum::<usize>();
    let (h, w) = board.dims();

    if sea_size + land_size == h * w {
        for (c, t) in board.iter() {
            if t != Empty {
                continue;
            }

            knowledge.set_land(Reason::SeaComplete, c);
        }
    }
}
