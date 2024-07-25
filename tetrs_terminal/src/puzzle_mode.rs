use std::{collections::VecDeque, num::NonZeroU32, time::Duration};

use tetrs_engine::{
    Feedback, FeedbackEvents, Game, GameConfig, GameOver, GameState, Gamemode, InternalEvent, Stat,
    Tetromino,
};

pub fn make_game() -> Game {
    const SPEED_LEVEL: NonZeroU32 = NonZeroU32::MIN.saturating_add(1);
    let mut init = false;
    let mut puzzle_num = 0;
    let mut puzzle_piece_stamp = 0;
    #[allow(non_snake_case)]
    // SAFETY: 255 > 0.
    #[rustfmt::skip]
    let puzzles = [
        ("Intro", vec![
            b"OOO    OOO",
            b"OOOO  OOOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
        ], VecDeque::from([Tetromino::I,Tetromino::L])),
        ("I-spin (i)", vec![
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OOOO    OO",
        ], VecDeque::from([Tetromino::I,Tetromino::I])),
        ("I-spin (ii)", vec![
            b"OOOOO  OOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OO    OOOO",
        ], VecDeque::from([Tetromino::I,Tetromino::J])),
        ("I-spin (iii)", vec![
            b"OOOOO  OOO",
            b"OOO OO OOO",
            b"OOO OO OOO",
            b"OOO     OO",
            b"OOO OOOOOO",
        ], VecDeque::from([Tetromino::I,Tetromino::I,Tetromino::L,])),
        ("I-spin (iv)", vec![
            b"OO  O   OO",
            b"OO    OOOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
        ], VecDeque::from([Tetromino::I,Tetromino::L,Tetromino::O,])),
        /* Stage Template.
        ("puzzlename", vec![
            b"OOOOOOOOOO",
            b"OOOOOOOOOO",
            b"OOOOOOOOOO",
            b"OOOOOOOOOO",
        ], VecDeque::from([Tetromino::I,])),
        */
    ];
    let total_lines = puzzles
        .iter()
        .map(|(_, puzzle_lines, _)| puzzle_lines.len())
        .sum::<usize>();
    let mut puzzles = puzzles.into_iter();
    let game_modifier = move |upcoming_event: Option<InternalEvent>,
                              config: &mut GameConfig,
                              state: &mut GameState,
                              feedback_events: &mut FeedbackEvents| {
        // Initialize internal game state.
        if !init {
            config.preview_count = 1;
            init = true;
        }
        // Puzzle may have failed.
        let game_piece_stamp = state.pieces_played.iter().sum::<u32>();
        if upcoming_event == Some(InternalEvent::Spawn) && game_piece_stamp == puzzle_piece_stamp {
            // If board is cleared successfully load in next batch.
            if state.board.iter().all(|line| {
                line.iter().all(|cell| cell.is_none()) || line.iter().all(|cell| cell.is_some())
            }) {
                // Load in new puzzle.
                if let Some((puzzle_name, puzzle_lines, puzzle_pieces)) = puzzles.next() {
                    state.consecutive_line_clears = 0;
                    // Game messages.
                    puzzle_num += 1;
                    feedback_events.push((
                        state.game_time,
                        Feedback::Message(format!("# Puzzle {puzzle_num}: {puzzle_name}")),
                    ));
                    // Queue pieces and lines.
                    puzzle_piece_stamp =
                        game_piece_stamp + u32::try_from(puzzle_pieces.len()).unwrap();
                    state.next_pieces = puzzle_pieces;
                    // Additional piece for consistent end preview.
                    state.next_pieces.push_back(Tetromino::O);
                    for (y, line_template) in puzzle_lines.iter().rev().enumerate() {
                        state.board[y] = line_template.map(|b| {
                            if b == b' ' {
                                None
                            } else {
                                Some(unsafe { NonZeroU32::new_unchecked(255) })
                            }
                        });
                        // Set puzzle limit
                    }
                }
            } else {
                // Otherwise game failed
                state.finished = Some(Err(GameOver::Fail));
            }
        }
        // Hacky way to show the puzzle level.
        if upcoming_event.is_some() {
            state.level = SPEED_LEVEL;
        } else {
            state.level = NonZeroU32::try_from(puzzle_num).unwrap();
        }
    };
    let mut game = Game::with_gamemode(Gamemode::custom(
        "Puzzle".to_string(),
        SPEED_LEVEL,
        false,
        Some(Stat::Lines(total_lines)),
        Stat::Time(Duration::ZERO),
    ));
    game.set_modifier(Some(Box::new(game_modifier)));
    game
}
