use super::*;
use rustc_hash::FxHashMap as HashMap;

pub fn guess(known: &mut Knowledge) {
    let islands = known.island_set().clone();
    let path_map = islands
        .iter()
        .map(|&i| (i, enumerate_island_paths(known, i).collect::<Vec<_>>()))
        .collect::<HashMap<_, _>>();

    let Some(min) = path_map
        .values()
        .map(|ps| ps.len())
        .filter(|&l| l > 1)
        .min()
    else {
        return;
    };

    let Some((&guess_island, guesses)) = path_map.iter().find(|(_, ps)| ps.len() == min) else {
        return;
    };

    for guess in guesses {
        let mut bifurcation = known.bifurcate();
        for &cell in guess {
            bifurcation.set_land(Reason::Bifurcation, cell);
            for &island in islands.iter() {
                if island == guess_island {
                    continue;
                }

                bifurcation.elim_island(Reason::Bifurcation, cell, island);
            }
        }

        solve_knowing(&mut bifurcation);

        if bifurcation.solved() {
            for &cell in guess {
                known.set_land(Reason::Bifurcation, cell);
                for &island in islands.iter() {
                    if island == guess_island {
                        continue;
                    }

                    known.elim_island(Reason::Bifurcation, cell, island);
                }
            }
        }
    }
}
