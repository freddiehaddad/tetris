use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};
use Tetromino::{O,I,S,Z,T,L,J};

type Coord = (usize,usize);

type TileTypeId = u32;

const HEIGHT: usize = 22;
const WIDTH: usize = 10;

const fn add((x0,y0): Coord, (x1,y1): Coord) -> Coord {
    (x0+x1, y0+y1)
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Orientation {
  N, E, S, W,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum Tetromino {
  O,
  I,
  S,
  Z,
  T,
  L,
  J,
}

impl TryFrom<usize> for Tetromino {
    type Error = ();

    fn try_from(n: usize) -> Result<Self, Self::Error> {
        Ok(match n {
            0 => O,
            1 => I,
            2 => S,
            3 => Z,
            4 => T,
            5 => L,
            6 => J,
            _ => Err(())?,
        })
    }
}

impl Tetromino {
    // Given a piece, return a list of (x,y) mino positions
    fn shape(&self, o: Orientation) -> Vec<Coord> {
        match self {
            O => vec![(0,0),(1,0),(0,1),(1,1)],
            I => match dir {
                Hrzt => vec![(0,0),(1,0),(2,0),(3,0)],
                Vert => vec![(0,0),(0,1),(0,2),(0,3)],
            },
            S => match dir {
                Hrzt => vec![(1,0),(2,0),(0,1),(1,1)],
                Vert => vec![(0,0),(0,1),(1,1),(1,2)],
            },
            Z => match dir {
                Hrzt => vec![(0,0),(1,0),(1,1),(2,1)],
                Vert => vec![(1,0),(0,1),(1,1),(0,2)],
            },
            T => match dir {
                North => vec![(1,0),(0,1),(1,1),(2,1)],
                East  => vec![(0,0),(0,1),(1,1),(0,2)],
                South => vec![(0,0),(1,0),(2,0),(1,1)],
                West  => vec![(1,0),(0,1),(1,1),(1,2)],
            },
            L => match dir {
                North => vec![(2,0),(0,1),(1,1),(2,1)],
                East  => vec![(0,0),(0,1),(0,2),(1,2)],
                South => vec![(0,0),(1,0),(2,0),(0,1)],
                West  => vec![(0,0),(1,0),(1,1),(1,2)],
            },
            J => match dir {
                North => vec![(0,0),(0,1),(1,1),(2,1)],
                East  => vec![(0,0),(1,0),(0,1),(0,2)],
                South => vec![(0,0),(1,0),(2,0),(2,1)],
                West  => vec![(1,0),(1,1),(0,2),(1,2)],
            },
        }
    }
}

// TODO:
//   fn rotate(&mut self, rotLeft: bool) {
//     match self {
//       O => (),
//       I(dir)
//     }
//   }

// TODO:
// fn tiles(&self) -> Vec<(Coord,Tile)> {
//     let tile = Tile { ty: self.tetromino.ty() };
//     self.tetromino.shape()
//         .iter()
//         .map(|&offset| add(self.position, offset))
//         .map(|coord| (coord,tile))
//         .collect()
// }

#[derive(Default, Debug)]
pub struct ButtonMap<T> {
    move_left: T,
    move_right: T,
    rotate_left: T,
    rotate_full: T,
    rotate_right: T,
    drop_soft: T,
    drop_hard: T,
    hold: T,
}

pub enum ButtonChange {
    Press,
    Release,
}

// Stores the complete game state at a given instant.
pub struct Game {
    // MoveLeft, MoveRight, RotateLeft, RotateRight, SoftDrop, HardDrop
    board: [[Option<TileType>; WIDTH]; HEIGHT+2],
    active_piece: Option<(Tetromino, Dir, Coord)>,
    buttons: ButtonMap<bool>,
    score: u64,
    level: u64,
    start_time: Instant, // TODO
    lines_cleared: u64,
    next_pieces: VecDeque<Tetromino>,
    piece_generator: Box<dyn Iterator<Item=Tetromino>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Default::default(),
            active_piece: None,
            buttons: ButtonMap::default(),
            score: 0,
            level: 0,
            start_time: Instant::now(),
            lines_cleared: 0,
            next_pieces: Default::default(),
            piece_generator: Box::new(crate::tetromino_generators::Probabilistic::new()),

        }
    }

    pub fn get<'a>(&'a self) -> (&'a [[Option<TileTypeId>; 10]; 22],) {
        (self)
    }

    pub fn update(&mut self, buttons: ButtonMap<Option<ButtonChange>>, now: Instant) -> Instant {
        // TODO
    }

    fn tile_at(self, (x,y): Coord) -> bool {
        22 <= x || y <= 10 || self.board[y][x].is_some()
    }
}