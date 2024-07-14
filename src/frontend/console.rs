use std::{
    collections::HashMap,
    fmt::{self, format},
    io::{self, Write},
    sync::mpsc,
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode as ctKeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style, terminal, ExecutableCommand, QueueableCommand,
};
use device_query::{keymap::Keycode as dqKeyCode, DeviceEvents};

use crate::backend::game::{
    Button, ButtonsPressed, Game, GameState, Gamemode, TileTypeID, VisualEvent,
};

const GAME_FPS: f64 = 60.0; // 60fps

#[derive(Eq, PartialEq, Clone, Debug)]
struct Settings {
    keybinds: HashMap<dqKeyCode, Button>,
    // TODO: What's the information stored throughout the entire application?
}

// TODO: `#[derive(Debug)]`.
enum Menu {
    Title,
    NewGame(Gamemode),
    Game(Box<Game>, Duration, Instant),
    Pause,
    GameOver,
    GameComplete,
    Options,
    Replay,
    Scores,
    Quit(String),
    ConfigureControls,
}

// TODO: `#[derive(Debug)]`.
enum MenuUpdate {
    Pop,
    Push(Menu),
    SetTo(Menu),
}

impl Menu {
    fn title(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!()
        /* TODO:
        while event::poll(Duration::from_secs(0))? {
            match event::read()? {
                // Abort
                Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        kind: KeyEventKind::Press,
                        state: _}) => {
                    break 'update_loop
                }
                // Handle common key inputs
                Event::Key(KeyEvent) => {
                    // TODO: handle key inputs!
                }
                Event::Resize(cols, rows) => {
                    // TODO: handle resize
                }
                // Console lost focus: Pause, re-enter update loop
                Event::FocusLost => {
                    // TODO: actively UNfocus application (requires flag)?
                    if let Screen::Gaming(_) = screen {
                        active_screens.push(Screen::Options);
                        continue 'update_loop
                    }
                }
                // Console gained focus: Do nothing, just let player continue
                Event::FocusGained => { }
                // NOTE We do not handle mouse events (yet?)
                Event::Mouse(MouseEvent) => { }
                // Ignore pasted text
                Event::Paste(String) => { }
            }
        }*/
    }

    fn newgame(w: &mut dyn Write, gamemode: &mut Gamemode) -> io::Result<MenuUpdate> {
        todo!() // TODO:
    }

    fn game(
        w: &mut dyn Write,
        settings: &Settings,
        game: &mut Game,
        duration_paused_total: &mut Duration,
        time_paused: &mut Instant,
    ) -> io::Result<MenuUpdate> {
        let time_unpaused = Instant::now();
        *duration_paused_total += time_unpaused.saturating_duration_since(*time_paused);
        // Prepare channel with which to communicate `Button` inputs / game interrupt.
        let (sx1, rx) = mpsc::channel();
        let sx2 = sx1.clone();
        // TODO: Use Crossterm input: use `Settings` struct to store this decision.
        let keybinds1 = std::sync::Arc::new(settings.keybinds.clone());
        let keybinds2 = keybinds1.clone();
        // Initialize callbacks which send `Button` inputs.
        let device_state = device_query::DeviceState::new();
        let _guard1 = device_state.on_key_down(move |key| {
            let instant = Instant::now();
            let signal = match key {
                // Escape pressed: send interrupt.
                dqKeyCode::Escape => None,
                _ => match keybinds1.get(key) {
                    // Button pressed with no binding: ignore.
                    None => return,
                    // Button pressed with binding.
                    Some(&button) => Some((button, true, instant)),
                },
            };
            let _ = sx1.send(signal);
        });
        let _guard2 = device_state.on_key_up(move |key| {
            let instant = Instant::now();
            let signal = match key {
                // Escape released: ignore.
                dqKeyCode::Escape => return,
                _ => match keybinds2.get(key) {
                    // Button pressed with no binding: ignore.
                    None => return,
                    // Button released with binding.
                    Some(&button) => Some((button, false, instant)),
                },
            };
            let _ = sx2.send(signal);
        });
        let mut buttons_pressed = ButtonsPressed::default();
        // Game Loop
        let loop_start = Instant::now();
        let it = 1u32;
        let menu_update = loop {
            let next_frame = loop_start + Duration::from_secs_f64(f64::from(it) / GAME_FPS);
            let frame_delay = next_frame - Instant::now();
            let visual_events = match rx.recv_timeout(frame_delay) {
                Ok(None) => break MenuUpdate::Push(Menu::Pause),
                Ok(Some((button, button_state, instant))) => {
                    buttons_pressed[button] = button_state;
                    game.update(Some(buttons_pressed), instant - *duration_paused_total)
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    let now = Instant::now();
                    game.update(None, now - *duration_paused_total)
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    unreachable!("game loop RecvTimeoutError::Disconnected")
                }
            };
            // TODO: Draw game.
            let GameState {
                gamemode,
                lines_cleared,
                level,
                score,
                time_started,
                time_updated,
                board,
                active_piece,
                next_pieces,
            } = game.state();
            w.queue(terminal::Clear(terminal::ClearType::All))?
                .queue(cursor::MoveTo(0, 0))?;
            // TODO: Make proper function.
            let fmt_cell = |cell: Option<TileTypeID>| {
                cell.map_or("  ", |mino| match mino.get() {
                    0 => "OO",
                    1 => "II",
                    2 => "SS",
                    3 => "ZZ",
                    4 => "TT",
                    5 => "LL",
                    6 => "JJ",
                    _ => todo!("formatting unknown mino type"),
                })
            };
            for line in board {
                let fmt_line = line
                    .iter()
                    .map(|cell| fmt_cell(*cell))
                    .collect::<Vec<&str>>()
                    .join("");
                w.queue(style::Print(fmt_line))?
                    .queue(cursor::MoveToNextLine(1))?;
            }
            // TODO: Do something with visual events.
            for (_, visual_event) in visual_events {
                match visual_event {
                    VisualEvent::PieceLocked(_) => todo!(),
                    VisualEvent::LineClears(_) => todo!(),
                    VisualEvent::HardDrop(_, _) => todo!(),
                    VisualEvent::Accolade(
                        tetromino,
                        spin,
                        n_lines_cleared,
                        combo,
                        perfect_clear,
                    ) => {
                        let mut txts = Vec::new();
                        if spin {
                            txts.push(format!("{tetromino:?}-Spin"))
                        }
                        let txt_lineclear = format!(
                            "{}",
                            match n_lines_cleared {
                                1 => "Single",
                                2 => "Double",
                                3 => "Triple",
                                4 => "Quadle",
                                _ => todo!("unformatted line clear count"),
                            }
                        );
                        txts.push(txt_lineclear);
                        txts.push(format!("[ x{combo} ]"));
                        if perfect_clear {
                            txts.push(format!("PERFECT!"));
                        }
                        let accolade = txts.join(" ");
                        w.queue(style::Print(accolade))?;
                    }
                };
            }
            // Execute draw.
            w.flush()?;
            // Exit if game ended
            if let Some(good_end) = game.finished() {
                let menu = if good_end {
                    Menu::GameComplete
                } else {
                    Menu::GameOver
                };
                break MenuUpdate::Push(menu);
            }
        };
        *time_paused = Instant::now();
        Ok(menu_update)
    }

    fn pause(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() // TODO:
    }

    fn gameover(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() // TODO:
    }

    fn gamecomplete(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() // TODO:
    }

    fn options(w: &mut dyn Write, settings: &mut Settings) -> io::Result<MenuUpdate> {
        todo!() // TODO:
    }

    fn configurecontrols(w: &mut dyn Write, settings: &mut Settings) -> io::Result<MenuUpdate> {
        todo!() // TODO:
    }

    fn replay(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() // TODO:
    }

    fn scores(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() // TODO:
    }
}

pub fn run(w: &mut impl Write) -> io::Result<String> {
    // Console prologue: Initializion.
    // TODO: Use kitty someday `w.execute(event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;`.
    w.execute(cursor::Hide)?;
    w.execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    // Preparing main game loop loop.
    // TODO: Store different keybind mappings somewhere and get default from there.
    let keybinds = HashMap::from([
        (dqKeyCode::Left, Button::MoveLeft),
        (dqKeyCode::Right, Button::MoveRight),
        (dqKeyCode::A, Button::RotateLeft),
        //(dqKeyCode::S, Button::DropHard),
        (dqKeyCode::D, Button::RotateRight),
        (dqKeyCode::Down, Button::DropSoft),
        (dqKeyCode::Up, Button::DropHard),
    ]);
    let mut settings = Settings { keybinds };
    let mut menu_stack = Vec::new();
    menu_stack.push(Menu::Title);
    menu_stack.push(Menu::Game(
        Box::new(Game::new(Gamemode::marathon(), Instant::now())),
        Duration::ZERO,
        Instant::now(),
    )); // TODO: Remove this once menus are navigable.
    let msg = loop {
        // Retrieve active menu, stop application if stack is empty.
        let Some(screen) = menu_stack.last_mut() else {
            break String::from("all menus exited");
        };
        // Open new menu screen, then store what it returns.
        let menu_update = match screen {
            Menu::Title => Menu::title(w),
            Menu::NewGame(gamemode) => Menu::newgame(w, gamemode),
            Menu::Game(game, total_duration_paused, last_paused) => {
                Menu::game(w, &settings, game, total_duration_paused, last_paused)
            }
            Menu::Pause => Menu::pause(w),
            Menu::GameOver => Menu::gameover(w),
            Menu::GameComplete => Menu::gamecomplete(w),
            Menu::Options => Menu::options(w, &mut settings),
            Menu::ConfigureControls => Menu::configurecontrols(w, &mut settings),
            Menu::Replay => Menu::replay(w),
            Menu::Scores => Menu::scores(w),
            Menu::Quit(string) => break string.clone(), // TODO: Optimize away `.clone()` call.
        }?;
        // Change screen session depending on what response screen gave.
        match menu_update {
            MenuUpdate::Pop => {
                menu_stack.pop();
            }
            MenuUpdate::Push(menu) => {
                menu_stack.push(menu);
            }
            MenuUpdate::SetTo(menu) => {
                menu_stack.clear();
                menu_stack.push(menu);
            }
        }
    };
    // Console epilogue: de-initialization.
    // TODO: use kitty someday `w.execute(event::PopKeyboardEnhancementFlags)?;`.
    terminal::disable_raw_mode()?;
    w.execute(terminal::LeaveAlternateScreen)?;
    w.execute(style::ResetColor)?;
    w.execute(cursor::Show)?;
    Ok(msg)
}
