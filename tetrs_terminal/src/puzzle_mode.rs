use std::{collections::VecDeque, num::NonZeroU32};

use tetrs_engine::{
    Feedback, FeedbackEvents, Game, GameConfig, GameMode, GameOver, GameState, InternalEvent, Limits, Tetromino
};

const MAX_STAGE_ATTEMPTS: usize = 3;
const SPEED_LEVEL: u32 = 3;

pub fn make_game() -> Game {
    #[rustfmt::skip]
    let puzzles = [
        /* Puzzle template.
        ("puzzlename", vec![
            b"OOOOOOOOOO",
            b"OOOOOOOOOO",
            b"OOOOOOOOOO",
            b"OOOOOOOOOO",
        ], VecDeque::from([Tetromino::I,])),
        */
        // I-spins.
        ("I-spin", vec![
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OOOO    OO",
            ], VecDeque::from([Tetromino::I,Tetromino::I])),
        ("I-spin", vec![
            b"OOOOO  OOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OO    OOOO",
            ], VecDeque::from([Tetromino::I,Tetromino::J])),
        ("I-spin Triple", vec![
            b"OO  O   OO",
            b"OO    OOOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::I,Tetromino::L,Tetromino::O,])),
        ("I-spin trial", vec![
            b"OOOOO  OOO",
            b"OOO OO OOO",
            b"OOO OO OOO",
            b"OOO     OO",
            b"OOO OOOOOO",
            ], VecDeque::from([Tetromino::I,Tetromino::I,Tetromino::L,])),
        // S/Z-spins.
        ("S-spin", vec![
            b"OOOO  OOOO",
            b"OOO  OOOOO",
            ], VecDeque::from([Tetromino::S,])),
        ("S-spins", vec![
            b"OOOO    OO",
            b"OOO    OOO",
            b"OOOOO  OOO",
            b"OOOO  OOOO",
            ], VecDeque::from([Tetromino::S,Tetromino::S,Tetromino::S,])),
        ("Z-spin galore", vec![
            b"O  OOOOOOO",
            b"OO  OOOOOO",
            b"OOO  OOOOO",
            b"OOOO  OOOO",
            b"OOOOO  OOO",
            b"OOOOOO  OO",
            b"OOOOOOO  O",
            b"OOOOOOOO  ",
            ], VecDeque::from([Tetromino::Z,Tetromino::Z,Tetromino::Z,Tetromino::Z,])),
        ("SuZ-spin trial", vec![
            b"OOOO  OOOO",
            b"OOO  OOOOO",
            b"OO    OOOO",
            b"OO    OOOO",
            b"OOO    OOO",
            b"OO  OO  OO",
            ], VecDeque::from([Tetromino::S,Tetromino::S,Tetromino::I,Tetromino::I,Tetromino::Z,])),
        // L/J-spins.
        ("J-spin", vec![
            b"OO     OOO",
            b"OOOOOO OOO",
            b"OOOOO  OOO",
            ], VecDeque::from([Tetromino::J,Tetromino::I,])),
        ("L/J-spin", vec![
            b"OO      OO",
            b"OO OOOO OO",
            b"OO  OO  OO",
            ], VecDeque::from([Tetromino::J,Tetromino::L,Tetromino::I])),
        ("L-spin", vec![
            b"OOOOO OOOO",
            b"OOO   OOOO",
            ], VecDeque::from([Tetromino::L,])),
        ("L/J-spin trial", vec![
            b"O   OO   O",
            b"O O OO O O",
            b"O   OO   O",
            ], VecDeque::from([Tetromino::J,Tetromino::L,Tetromino::J,Tetromino::L,])),
        // L/J-turns.
        ("L-turn", vec![
            b"OOOO  OOOO",
            b"OOOO  OOOO",
            b"OOOO   OOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::L,Tetromino::O,])),
        ("77-turn", vec![
            b"OOOO  OOOO",
            b"OOOOO OOOO",
            b"OOO   OOOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::L,Tetromino::L,])),
        ("L-turn revisited", vec![
            b"OOOOO  OOO",
            b"OOO    OOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::L,Tetromino::O,])),
        ("L-turn trial", vec![
            b"OOOO  OOOO",
            b"OOOO  OOOO",
            b"OO     OOO",
            b"OOO  OOOOO",
            b"OOO OOOOOO",
            ], VecDeque::from([Tetromino::L,Tetromino::L,Tetromino::O,])),
        // T-spins.
        ("T-spin", vec![
            b"OOOO    OO",
            b"OOO   OOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::T,Tetromino::I])),
        ("T-spin", vec![
            b"OOOO    OO",
            b"OOO   OOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::T,Tetromino::L])),
        ("T-turn", vec![
            b"OOO   OOOO",
            b"OOOO  OOOO",
            b"OOOO   OOO",
            ], VecDeque::from([Tetromino::T,Tetromino::T])),
        ("Tetrs T-spin", vec![
            b"OOO  OOOOO",
            b"OOO  OOOOO",
            b"OOOO   OOO",
            b"OOOOO OOOO",
            ], VecDeque::from([Tetromino::T,Tetromino::O])),
        ("Tetrs T-spin Triple", vec![
            b"OOO   OOOO",
            b"OOO  OOOOO",
            b"OOOO   OOO",
            b"OOOOO OOOO",
            b"OOOOO  OOO",
            b"OOOOO OOOO",
            ], VecDeque::from([Tetromino::T,Tetromino::J,Tetromino::L])),
    ];
    let mut current_puzzle = 0;
    let mut current_puzzle_attempt = 0;
    let mut current_puzzle_piececnt_limit = 0;
    let puzzle_num = NonZeroU32::try_from(u32::try_from(puzzles.len()).unwrap()).unwrap();
    let puzzle_modifier = move |
            config: &mut GameConfig,
            _mode: &mut GameMode,
            state: &mut GameState,
            feedback_events: &mut FeedbackEvents,
            before_event: Option<InternalEvent>,
    | {
        let game_piececnt = usize::try_from(state.pieces_played.iter().sum::<u32>()).unwrap();

        if before_event.is_some() {
            state.level = NonZeroU32::try_from(SPEED_LEVEL).unwrap();
        } else {
            state.level = NonZeroU32::try_from(u32::try_from(current_puzzle).unwrap()).unwrap();
            // Delete accolades.
            feedback_events.retain(|evt| !matches!(evt, (_, Feedback::Accolade { .. })));
        }
        if before_event != Some(InternalEvent::Spawn) {
            return;
        }
        // End of puzzle / start of new one.
        if game_piececnt == current_puzzle_piececnt_limit {
            let puzzle_done = state
                .board
                .iter()
                .all(|line| line.iter().all(|cell| cell.is_none()));
            if !puzzle_done && current_puzzle_attempt >= MAX_STAGE_ATTEMPTS {
                // Run out of attempts, game over.
                state.end = Some(Err(GameOver::ModeLimit));
            } else {
                // Change puzzle number or repeat attempt.
                if puzzle_done {
                    current_puzzle += 1;
                    current_puzzle_attempt = 1;
                } else {
                    current_puzzle_attempt += 1;
                }
                if current_puzzle == puzzles.len() {
                    // Done with all puzzles, game completed.
                    state.end = Some(Ok(()));
                } else {
                    // Load in new puzzle.
                    let (puzzle_name, puzzle_lines, puzzle_pieces) = &puzzles[current_puzzle];
                    current_puzzle_piececnt_limit = game_piececnt + puzzle_pieces.len();
                    state.consecutive_line_clears = 0;
                    // Game message.
                    feedback_events.push((
                        state.game_time,
                        Feedback::Message(if current_puzzle_attempt == 1 {
                            format!("Stage {}: {}", 1+current_puzzle, puzzle_name.to_ascii_uppercase())
                        } else {
                            format!(
                                "{}.RETRY ({})",
                                current_puzzle_attempt - 1,
                                puzzle_name.to_ascii_uppercase()
                            )
                        }),
                    ));
                    // Queue pieces and lines.
                    state.next_pieces.clone_from(puzzle_pieces);
                    // Additional piece for consistent end preview.
                    state.next_pieces.push_back(Tetromino::I);
                    // Load in pieces.
                    for (puzzle_line, board_line) in puzzle_lines
                        .iter()
                        .rev()
                        .map(|line| {
                            line.map(|b| {
                                if b == b' ' {
                                    None
                                } else {
                                    Some(unsafe { NonZeroU32::new_unchecked(255) })
                                }
                            })
                        })
                        .chain(std::iter::repeat(Default::default()))
                        .zip(state.board.iter_mut())
                    {
                        *board_line = puzzle_line;
                    }
                }
            }
        }
        config.preview_count = current_puzzle_piececnt_limit - game_piececnt;
    };
    let mut game = Game::new(GameMode {
        name: "Puzzle".to_string(),
        start_level: NonZeroU32::MIN.saturating_add(1),
        increment_level: false,
        limits: Limits { level: Some((true, puzzle_num)), ..Default::default() },
    });
    unsafe { game.add_modifier(Box::new(puzzle_modifier)) };
    game
}
