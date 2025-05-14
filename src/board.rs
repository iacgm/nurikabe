use std::ops::{Index, IndexMut};

use ratatui::style::Color;

pub use Tile::*;

pub type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Island {
    pub r: usize,
    pub c: usize,
    pub n: usize,
}

#[derive(Clone)]
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub islands: Vec<Island>,
    pub island_map: Vec<Vec<Option<Island>>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Tile {
    #[default]
    Empty,
    Sea,
    Land,
}

impl Board {
    pub fn empty(rows: usize, cols: usize) -> Self {
        Self {
            tiles: vec![vec![Empty; cols]; rows],
            islands: vec![],
            island_map: vec![vec![None; cols]; rows],
        }
    }

    pub fn resize(&mut self, (h, w): Coord) {
        for row in &mut self.tiles {
            row.resize(w, Empty);
        }
        self.tiles.resize(h, vec![Empty; w]);

        for row in &mut self.island_map {
            row.resize(w, None);
        }
        self.island_map.resize(h, vec![None; w]);
    }

    pub fn dims(&self) -> Coord {
        (self.tiles.len(), self.tiles[0].len())
    }

    pub fn from_islands(rows: usize, cols: usize, islands: impl Iterator<Item = Island>) -> Self {
        let mut new = Self::empty(rows, cols);
        for island in islands {
            new.add_island(island);
        }
        new
    }

    pub fn add_island(&mut self, island: Island) {
        let Island { r, c, .. } = island;
        self.remove_island((r, c));
        self.tiles[r][c] = Land;
        self.islands.push(island);
        self.island_map[r][c] = Some(island);
    }

    pub fn remove_island(&mut self, (r, c): (usize, usize)) {
        self.islands.retain(|i| !(i.r == r && i.c == c));
        self.island_map[r][c] = None;
    }

    pub fn solved(&self) -> bool {
        self.tiles
            .iter()
            .all(|row| row.iter().all(|&tile| tile != Empty))
    }
}

impl Index<Coord> for Board {
    type Output = Tile;
    fn index(&self, (r, c): Coord) -> &Self::Output {
        &self.tiles[r][c]
    }
}

impl IndexMut<Coord> for Board {
    fn index_mut(&mut self, (r, c): Coord) -> &mut Self::Output {
        &mut self.tiles[r][c]
    }
}

impl Tile {
    pub fn color(&self) -> Color {
        use Color::*;
        match self {
            Land => Rgb(10, 80, 10),
            Empty => Black,
            Sea => Blue,
        }
    }
}

impl From<(usize, usize, usize)> for Island {
    fn from((x, y, n): (usize, usize, usize)) -> Self {
        Self { r: x, c: y, n }
    }
}
