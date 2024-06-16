use Tetromino::{O,I,S,Z,T,L,J};
use Dir2::{Hrzt,Vert};
use Dir4::{North,East,South,West};


type Coord = (u8,u8);

const fn add((x0,y0): Coord, (x1,y1): Coord) -> Coord {
    (x0+x1, y0+y1)
}

struct Action {
    
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Tile {
    ty: u8, // TODO right datatype?
}

// Stores the complete game state at a given instant.
#[derive(Debug)]
pub struct Game {
    // Pause, MoveLeft, MoveRight, RotateLeft, RotateRight, SoftDrop, HardDrop
    board: [[Tile; 10]; 20],
    active_piece: (Tetromino, Coord),
    score: u64,
    level: u64,
    time: (), // TODO
    lines_cleared: u64,
}

impl Game {
    fn update()
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
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tetromino {
  O,
  I(Dir2),
  S(Dir2),
  Z(Dir2),
  T(Dir4),
  L(Dir4),
  J(Dir4),
}

impl Tetromino {
    fn ty(&self) -> u8 {
        match self {
            O    => 0,
            I(_) => 1,
            S(_) => 2,
            Z(_) => 3,
            T(_) => 4,
            L(_) => 5,
            J(_) => 6,
        }
    }
    // Given a piece, return a list of (x,y) mino positions
    fn shape(&self) -> Vec<Coord> {
        match self {
            O => vec![(0,0),(1,0),(0,1),(1,1)],
            I(dir) => match dir {
                Hrzt => vec![(0,0),(1,0),(2,0),(3,0)],
                Vert => vec![(0,0),(0,1),(0,2),(0,3)],
            },
            S(dir) => match dir {
                Hrzt => vec![(1,0),(2,0),(0,1),(1,1)],
                Vert => vec![(0,0),(0,1),(1,1),(1,2)],
            },
            Z(dir) => match dir {
                Hrzt => vec![(0,0),(1,0),(1,1),(2,1)],
                Vert => vec![(1,0),(0,1),(1,1),(0,2)],
            },
            T(dir) => match dir {
                North => vec![(1,0),(0,1),(1,1),(2,1)],
                East  => vec![(0,0),(0,1),(1,1),(0,2)],
                South => vec![(0,0),(1,0),(2,0),(1,1)],
                West  => vec![(1,0),(0,1),(1,1),(1,2)],
            },
            L(dir) => match dir {
                North => vec![(2,0),(0,1),(1,1),(2,1)],
                East  => vec![(0,0),(0,1),(0,2),(1,2)],
                South => vec![(0,0),(1,0),(2,0),(0,1)],
                West  => vec![(0,0),(1,0),(1,1),(1,2)],
            },
            J(dir) => match dir {
                North => vec![(0,0),(0,1),(1,1),(2,1)],
                East  => vec![(0,0),(1,0),(0,1),(0,2)],
                South => vec![(0,0),(1,0),(2,0),(2,1)],
                West  => vec![(1,0),(1,1),(0,2),(1,2)],
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Dir2 {
  Hrzt,
  Vert,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Dir4 {
  North,
  East,
  South,
  West,
}