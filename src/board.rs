use std::ops::Index;

use ratatui::style::Color;

pub use Tile::*;

pub type Coord = (usize, usize);

#[derive(Clone, Copy)]
pub struct Island {
    pub r: usize,
    pub c: usize,
    pub n: usize,
}

#[derive(Clone)]
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub islands: Vec<Island>,
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
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
        }
    }

    pub fn check_island(self: &Board, (r, c): Coord) -> Option<Island> {
        self.islands.iter().cloned().find(|i| i.r == r && i.c == c)
    }

    pub fn dims(&self) -> Coord {
        (self.tiles.len(), self.tiles[0].len())
    }

    pub fn from_islands(rows: usize, cols: usize, islands: impl Iterator<Item = Island>) -> Self {
        let mut new = Self::empty(rows, cols);
        for island in islands {
            let Island { r, c, .. } = island;
            new.tiles[r][c] = Land;
            new.islands.push(island);
        }
        new
    }

    pub fn is_finished(&self) -> bool {
        self.tiles.iter().all(|row| row.iter().all(|&tile| tile != Empty))
    }
}

impl Index<Coord> for Board {
    type Output = Tile;
    fn index(&self, (r, c): Coord) -> &Self::Output {
        &self.tiles[r][c]
    }
}

impl Tile {
    pub fn color(&self) -> Color {
        use Color::*;
        match self {
            Land => Rgb(10, 80, 10),
            Empty => DarkGray,
            Sea => Blue,
        }
    }
}

impl From<(usize, usize, usize)> for Island {
    fn from((x, y, n): (usize, usize, usize)) -> Self {
        Self { r: x, c: y, n }
    }
}
