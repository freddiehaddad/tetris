use crate::backend::game::{ActivePiece, Board, Orientation, Tetromino};

pub trait RotationSystem {
    fn rotate(
        &mut self,
        piece: &ActivePiece,
        board: &Board,
        right_turns: i32,
    ) -> Option<ActivePiece>;
    fn place_initial(&mut self, tetromino: Tetromino) -> ActivePiece;
}

/*pub fn rotate_dummy(
    mut piece: ActivePiece,
    board: &Board,
    right_turns: i32,
) -> Option<ActivePiece> {
    piece.orientation = piece.orientation.rotate_r(right_turns);
    piece.fits_at(board, (0, 0))
}*/

#[derive(Debug)]
pub struct Classic;

impl RotationSystem for Classic {
    fn rotate(
        &mut self,
        old_piece: &ActivePiece,
        board: &Board,
        right_turns: i32,
    ) -> Option<ActivePiece> {
        let mut new_piece = *old_piece;
        new_piece.orientation = old_piece.orientation.rotate_r(right_turns);
        let right_rotation = match right_turns.rem_euclid(4) {
            // No rotation occurred.
            0 => return Some(new_piece),
            // One right rotation.
            1 => true,
            // Classic didn't define 180 rotation, just check if the "default" 180 rotation fits.
            2 => {
                return new_piece.fits_at(board, (0, 0));
            }
            // One left rotation.
            3 => false,
            _ => unreachable!(),
        };
        use Orientation::*;
        #[rustfmt::skip]
        let offset = match new_piece.shape {
            Tetromino::O => (0, 0), // ⠶
            Tetromino::I => match old_piece.orientation {
                N | S => (2, -1), // ⠤⠤ -> ⡇
                E | W => (-2, 1), // ⡇  -> ⠤⠤
            },
            Tetromino::S | Tetromino::Z => match old_piece.orientation {
                N | S => (1, 0),  // ⠴⠂ -> ⠳  // ⠲⠄ -> ⠞
                E | W => (-1, 0), // ⠳  -> ⠴⠂ // ⠞  -> ⠲⠄
            },
            Tetromino::T | Tetromino::L | Tetromino::J => match old_piece.orientation {
                N => if right_rotation { (1, -1) } else { (0, -1) } // ⠺  <- ⠴⠄ -> ⠗  // ⠤⠆ <-> ⠧  // ⠦⠄ <-> ⠏
                E => if right_rotation { (-1, 0) } else { (-1, 1) }  // ⠴⠄ <- ⠗  -> ⠲⠂ // ⠧  <-> ⠖⠂ // ⠏  <-> ⠒⠆
                S => if right_rotation { (0, 0) } else { (1, 0) },  // ⠗  <- ⠲⠂ -> ⠺  // ⠖⠂ <-> ⠹  // ⠒⠆ <-> ⠼
                W => if right_rotation { (0, 1) } else { (0, 0) }         // ⠲⠂ <- ⠺  -> ⠴⠄ // ⠹  <-> ⠤⠆ // ⠼  <-> ⠦⠄
            },
        };
        new_piece.fits_at(board, offset)
    }

    fn place_initial(&mut self, shape: Tetromino) -> ActivePiece {
        let pos = match shape {
            Tetromino::O => (4, 20),
            Tetromino::I => (3, 20),
            _ => (3, 20),
        };
        let orientation = Orientation::N;
        ActivePiece {
            shape,
            orientation,
            pos,
        }
    }
}

pub struct Okay;

impl RotationSystem for Okay {
    fn rotate(
        &mut self,
        piece: &ActivePiece,
        board: &Board,
        right_turns: i32,
    ) -> Option<ActivePiece> {
        todo!() // TODO: Implement Okay Rotation System.
    }

    fn place_initial(&mut self, shape: Tetromino) -> ActivePiece {
        let (orientation, pos) = match shape {
            Tetromino::O => (Orientation::N, (4, 20)),
            Tetromino::I => (Orientation::N, (3, 20)),
            Tetromino::S => (Orientation::E, (4, 20)),
            Tetromino::Z => (Orientation::W, (4, 20)),
            Tetromino::T => (Orientation::N, (3, 20)),
            Tetromino::L => (Orientation::E, (4, 20)),
            Tetromino::J => (Orientation::W, (4, 20)),
        };
        ActivePiece {
            shape,
            orientation,
            pos,
        }
    }
}

/* TODO: Improve and implement the 'Okay' Rotation System.
pub fn rotate_okay(piece: ActivePiece, board: &Board, right_turns: i32) -> Option<ActivePiece> {
    let ActivePiece(shape, o, pos) = piece;
    let r = match right_turns.rem_euclid(4) {
        0 => return Some(piece),
        1 => true,
        2 => return first_valid_kick(board, ActivePiece(shape, o.rotate_r(2), pos), 2, std::iter::once((0,0))),
        3 => false,
        _ => unreachable!()
    };
    use Orientation::*;
    #[rustfmt::skip]
    let kicks = match shape {
        Tetromino::O => (0,0), // ⠶ -> ⠶
        Tetromino::I => match o {
            N | S => (2,2), // ⠤⠤ -> ⡇
            E | W => (-2,-2), // ⡇  -> ⠤⠤
        },
        Tetromino::S => match o {
            N | S => (1,1), // ⠴⠂ -> ⠳
            E | W => (-1,-1), // ⠳  -> ⠴⠂
        },
        Tetromino::Z => match o {
            N | S => (1,1), // ⠲⠄ -> ⠞
            E | W => (-1,-1), // ⠞  -> ⠲⠄
        },
        Tetromino::T => match o {
            N => if r {(1,0)} else {(-1,0)}, // ⠴⠄ -> ⠗
            E => if r {(-1,-1)} else {(1,1)}, // ⠗  -> ⠲⠂
            S => if r {(0,1)} else {(0,-1)}, // ⠲⠂ -> ⠺
            W => (0,0), // ⠺  -> ⠴⠄
        },
        Tetromino::L => match o {
            N => if r {(1,0)} else {(-1,0)}, // ⠤⠆ -> ⠧
            E => if r {(-1,-1)} else {(1,1)}, // ⠧  -> ⠖⠂
            S => if r {(0,1)} else {(0,-1)}, // ⠖⠂ -> ⠹
            W => (0,0), // ⠹  -> ⠤⠆
        },
        Tetromino::J => match o {
            N => if r {(1,0)} else {(-1,0)}, // ⠦⠄ -> ⠏
            E => if r {(-1,-1)} else {(1,1)}, // ⠏  -> ⠒⠆
            S => if r {(0,1)} else {(0,-1)}, // ⠒⠆ -> ⠼
            W => (0,0), // ⠼  -> ⠦⠄
        },
    };
    first_valid_kick(board, piece, 1, kicks)
}*/

/* TODO: Implement the Super Rotation System.
pub fn rotate_super(board: Board, piece: ActivePiece, right_turns: i32) -> Option<ActivePiece> {}
*/

/* TODO: Implement the Arika Rotation System.
pub fn rotate_arika(board: Board, piece: ActivePiece, right_turns: i32) -> Option<ActivePiece> {}
*/
