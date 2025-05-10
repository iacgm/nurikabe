use ratatui::style::Color;

#[derive(Clone)]
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
    Number(usize),
}

impl Board {
    pub fn empty(rows: usize, cols: usize) -> Self {
        Self {
            tiles: vec![vec![Tile::Empty; cols]; rows],
            islands: vec![],
        }
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }

    pub fn from_islands(rows: usize, cols: usize, islands: impl Iterator<Item = Island>) -> Self {
        let mut new = Self::empty(rows, cols);
        for island in islands {
            let Island { r, c, n } = island;
            new.tiles[r][c] = Tile::Number(n);
            new.islands.push(island);
        }
        new
    }
}

impl Tile {
    pub fn land(&self) -> bool {
        use Tile::*;
        matches!(self, Land | Number(_))
    }

    pub fn color(&self) -> Color {
        use Color::*;
        use Tile::*;
        match self {
            Land | Number(_) => Rgb(10, 80, 10),
            Empty => DarkGray,
            Sea => Cyan,
        }
    }
}

impl From<(usize, usize, usize)> for Island {
    fn from((x, y, n): (usize, usize, usize)) -> Self {
        Self { r: x, c: y, n }
    }
}
