use super::*;

pub fn wall_trick(known: &mut Knowledge, board: &Board) {
    use Possibility::*;
    for (c, _) in board.iter() {
        let flipped = [true, false];
        // # of 90 degree rotations
        let rotations = [0, 1, 2, 3];

        for flip in flipped {
            for rot in rotations {
                let coords = transform(c, flip, rot);
                let [wall1, wall2, far, near] = coords;

                // Ensure full square is part of board
                if !coords.iter().all(|&c| board.contains(c)) {
                    continue;
                }

                // Ensure pattern applies
                if board[wall1] != Water || board[wall2] != Water || board[near] != Empty {
                    continue;
                }

                // Ensure far is only reachable through near
                if !known.get(far).is_subset(known.get(near)) {
                    continue;
                }

                let far_paths_all_pass = known
                    .get(far)
                    .clone()
                    .iter()
                    .filter_map(|s| if let Isle(i) = s { Some(i) } else { None })
                    .all(|&i| {
                        known
                            .island_paths(i)
                            .iter()
                            .filter(|p| p.contains(&far))
                            .all(|p| p.contains(&near))
                    });

                // If far is only reachable via near, set near to land
                if far_paths_all_pass {
                    known.set_land(Reason::WallTrick, near);
                }
            }
        }
    }
}

fn transform(c: Coord, flip: bool, rotation: usize) -> [Coord; 4] {
    let (r, c) = c;

    let mut out = [(r, c), (r, c + 1), (r + 1, c + 1), (r + 1, c)];

    if flip {
        out.swap(0, 1);
        out.swap(2, 3);
    }

    for _ in 0..rotation {
        // Not a typo, just weird ordering
        let temp = out[3];
        out[3] = out[2];
        out[2] = out[1];
        out[1] = out[0];
        out[0] = temp;
    }

    out
}
