use std::{
    collections::VecDeque,
    num::NonZeroU32,
    time::Duration,
};

use tetrs_engine::{
    Feedback, FeedbackEvents, Game, GameConfig, GameOver, GameState,
    Gamemode, InternalEvent, Stat, Tetromino,
};


pub fn make_game() -> Game {
    let mut init = false;
    let mut puzzle_num = 0;
    let mut puzzle_piece_stamp = 0;
    #[allow(non_snake_case)]
    // SAFETY: 255 > 0.
    let (r, W) = (None, Some(unsafe { NonZeroU32::new_unchecked(255) }));
    #[rustfmt::skip]
    let puzzles = [
        // 1: I Spin
        ("I-Spin (I)", vec![
            [W,W,W,W,W,r,W,W,W,W],
            [W,W,W,W,W,r,W,W,W,W],
            [W,W,W,W,W,r,W,W,W,W],
            [W,W,W,W,W,r,W,W,W,W],
            [W,W,W,W,r,r,r,r,W,W],
        ], VecDeque::from([Tetromino::I,Tetromino::I])),
        ("r", vec![
            [W,W,W,W,W,W,W,W,W,r],
            [W,W,W,W,W,W,W,W,W,r],
            [W,W,W,W,W,W,W,W,W,r],
            [W,W,W,W,W,W,W,W,W,r],
        ], VecDeque::from([Tetromino::I])),
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
                    // Game messages.
                    if puzzle_num > 0 {
                        feedback_events.push((
                            state.game_time,
                            Feedback::Message("# Puzzle completed!".to_string()),
                        ));
                    }
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
                    for (y, line) in puzzle_lines.into_iter().rev().enumerate() {
                        state.board[y] = line;
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
            state.level = NonZeroU32::MIN;
        } else {
            state.level = NonZeroU32::try_from(puzzle_num).unwrap();
        }
    };
    let mut game = Game::with_gamemode(Gamemode::custom(
        "Puzzle".to_string(),
        NonZeroU32::MIN,
        false,
        Some(Stat::Lines(total_lines)),
        Stat::Time(Duration::ZERO),
    ));
    game.set_modifier(Some(Box::new(game_modifier)));
    game
}
