use std::ops::{Index, IndexMut};

use base64::{Engine, engine::general_purpose::STANDARD};
use ratatui::style::Color;

pub use Tile::*;

pub type Grid<T> = Vec<Vec<T>>;
pub type Coord = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Island {
    pub r: usize,
    pub c: usize,
    pub n: usize,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    pub dims: (usize, usize),
    pub tiles: Vec<Tile>,
    pub islands: Vec<Island>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Tile {
    #[default]
    Empty,
    Water,
    Land,
}

impl Board {
    pub fn empty(rows: usize, cols: usize) -> Self {
        Self {
            dims: (rows, cols),
            tiles: vec![Empty; cols * rows],
            islands: vec![],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coord, Tile)> {
        let (h, w) = self.dims();
        (0..h).flat_map(move |r| (0..w).map(move |c| ((r, c), self[(r, c)])))
    }

    pub fn resize(&mut self, (h, w): Coord) {
        let mut new = vec![Empty; h * w];

        let (oh, ow) = self.dims();
        for r in 0..oh {
            for c in 0..ow {
                let o = r * ow + c;
                let i = r * w + c;

                new[i] = self.tiles[o];
            }
        }

        self.dims = (h, w);
        self.tiles = new;
    }

    pub fn dims(&self) -> Coord {
        self.dims
    }

    pub fn contains(&self, (r, c): Coord) -> bool {
        let (h, w) = self.dims();
        r < h && c < w
    }

    pub fn lookup_island(&self, (r, c): Coord) -> Option<Island> {
        self.islands.iter().copied().find(|i| i.r == r && i.c == c)
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
        self[(r, c)] = Land;
        self.islands.push(island);
    }

    pub fn remove_island(&mut self, (r, c): (usize, usize)) {
        self.islands.retain(|i| !(i.r == r && i.c == c));
    }

    pub fn solved(&self) -> bool {
        self.tiles.iter().all(|&tile| tile != Empty)
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Tile]> {
        self.tiles.chunks(self.dims.1)
    }

    pub fn from_b64(input: &str) -> Self {
        let bytes = STANDARD.decode(input).unwrap();

        let mut p = 0;

        let mut next = || {
            let i = bytes[p];
            p += 1;
            i
        };

        let h = next() as usize;
        let w = next() as usize;
        let mut ni = (next() as usize) << 8;
        ni += next() as usize;
        let mut nl = (next() as usize) << 8;
        nl += next() as usize;
        let mut ns = (next() as usize) << 8;
        ns += next() as usize;

        let mut islands = vec![];

        for _ in 0..ni {
            let r = next() as usize;
            let c = next() as usize;
            let n = next() as usize;
            islands.push(Island { r, c, n });
        }

        let mut board = Board::from_islands(h as _, w as _, islands.into_iter());

        for _ in 0..nl {
            let i = next() as usize * w + next() as usize;
            board.tiles[i] = Land;
        }

        for _ in 0..ns {
            let i = next() as usize * w + next() as usize;
            board.tiles[i] = Water;
        }

        board
    }

    pub fn b64(&self) -> String {
        let mut ser: Vec<u8> = vec![];

        let (h, w) = self.dims();
        ser.push(h as u8);
        ser.push(w as u8);

        let land: Vec<usize> = self
            .tiles
            .iter()
            .copied()
            .enumerate()
            .filter(|t| t.1 == Land)
            .map(|t| t.0)
            .collect();
        let seas: Vec<usize> = self
            .tiles
            .iter()
            .copied()
            .enumerate()
            .filter(|t| t.1 == Water)
            .map(|t| t.0)
            .collect();

        let ni: u16 = self.islands.len() as u16;
        let nl: u16 = land.len() as u16;
        let nc: u16 = seas.len() as u16;

        ser.push((ni >> 8) as u8);
        ser.push(ni as u8);
        ser.push((nl >> 8) as u8);
        ser.push(nl as u8);
        ser.push((nc >> 8) as u8);
        ser.push(nc as u8);

        for &Island { r, c, n } in self.islands.iter() {
            ser.push(r as u8);
            ser.push(c as u8);
            ser.push(n as u8);
        }

        for i in land {
            let r = i / w;
            let c = i % w;
            ser.push(r as u8);
            ser.push(c as u8);
        }

        for i in seas {
            let r = i / w;
            let c = i % w;
            ser.push(r as u8);
            ser.push(c as u8);
        }

        STANDARD.encode(ser)
    }

    pub fn slug(&self) -> String {
        let mut ser = vec![];

        let (h, w) = self.dims();
        ser.push(h as u8);
        ser.push(w as u8);

        for &Island { r, c, n } in self.islands.iter() {
            ser.push(r as u8);
            ser.push(c as u8);
            ser.push(n as u8);
        }

        STANDARD.encode(ser)
    }
}

impl Index<Coord> for Board {
    type Output = Tile;
    fn index(&self, (r, c): Coord) -> &Self::Output {
        let (_, w) = self.dims;
        &self.tiles[r * w + c]
    }
}

impl IndexMut<Coord> for Board {
    fn index_mut(&mut self, (r, c): Coord) -> &mut Self::Output {
        let (_, w) = self.dims;
        &mut self.tiles[r * w + c]
    }
}

impl Tile {
    pub fn color(&self) -> Color {
        use Color::*;
        match self {
            Land => Rgb(10, 80, 10),
            Empty => Black,
            Water => Blue,
        }
    }
}

impl From<(usize, usize, usize)> for Island {
    fn from((x, y, n): (usize, usize, usize)) -> Self {
        Self { r: x, c: y, n }
    }
}
