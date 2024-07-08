use crate::backend::game::{Button, ButtonChange, ButtonMap, Game, Gamemode};

use std::{
    collections::HashMap, io::Write, num::NonZeroU32, sync::mpsc, time::{Duration, Instant}
};

//use device_query;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style,
    terminal,
    ExecutableCommand, QueueableCommand,
};
use device_query::{keymap as dq, DeviceEvents};

type ScreenStackUpdate = (usize, Vec<Screen>);

const GAME_FPS: f64 = 60.0; // 60fps

struct Settings {
    keybinds: HashMap<dq::Keycode, Button>,
    //TODO information stored throughout application?
}

enum Screen {
    Title, //TODO Store selected gamemode or smth for the selection screen for convenience
    Gaming(Box<Game>),
    Settings, //TODO Get inspired by Noita's system on how to handle exiting to main menu (or not) or how to start a new game while within a game and pausing/opening settings
}

fn enter_title_screen(w: &mut dyn Write) -> std::io::Result<ScreenStackUpdate> {
    Ok((0, vec![Screen::Gaming(Box::new(Game::new(Gamemode::endless())))]))
    /*TODO make title screen
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
                // TODO handle key inputs!
            }
            Event::Resize(cols, rows) => {
                // TODO handle resize
            }
            // Console lost focus: Pause, re-enter update loop
            Event::FocusLost => {
                // TODO actively UNfocus application (requires flag)?
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

fn enter_settings(w: &mut dyn Write, settings: &mut Settings) -> std::io::Result<ScreenStackUpdate> {
    //TODO implement options overlay
    Ok((1,Vec::new()))
}

fn enter_game(w: &mut dyn Write, settings: &Settings, game: &mut Game) -> std::io::Result<ScreenStackUpdate> {
    // Prepare channel with which to communicate Button inputs / game interrupt
    let (sx1, rx) = mpsc::channel();
    let sx2 = sx1.clone();
    let keybinds1 = std::sync::Arc::new(settings.keybinds.clone());
    let keybinds2 = keybinds1.clone();
    // Initialize callbacks which send Button inputs
    let device_state = device_query::DeviceState::new();
    let _guard1 =  device_state.on_key_down(move |key| {
        let signal = match key {
            // Escape pressed: send interrupt
            dq::Keycode::Escape => None,
            _ => match keybinds1.get(key) {
                // Button pressed with no binding: ignore
                None => return,
                // Button pressed with binding
                Some(&button) => Some((button, true)),
            }
        };
        let _ = sx1.send(signal);
    });
    let _guard2 =  device_state.on_key_up(move |key| {
        let signal = match key {
            // Escape released: ignore
            dq::Keycode::Escape => return,
            _ => match keybinds2.get(key) {
                // Button pressed with no binding: ignore
                None => return,
                // Button released with binding
                Some(&button) => Some((button, false)),
            }
        };
        let _ = sx2.send(signal);
    });
    // Game Loop
    let game_loop_start = Instant::now();
    for i in 1u32.. {
        let next_frame = game_loop_start + Duration::from_secs_f64(f64::from(i) / GAME_FPS);
        let frame_delay = next_frame - Instant::now();
        match rx.recv_timeout(frame_delay) {
            Ok(None) => return Ok((0,vec![Screen::Settings])),
            Ok(Some((button, is_press_down))) => {
                let now = Instant::now();
                let mut changes = ButtonMap::default();
                changes[button] = Some(is_press_down);
                game.update(Some(changes), now);
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let now = Instant::now();
                game.update(None, now);
            },
            Err(mpsc::RecvTimeoutError::Disconnected) => todo!(),
        };
        //TODO draw game!
    }
    Ok((1,vec![]))
}

pub fn run(w: &mut impl Write) -> std::io::Result<()> {
    // Initialize console
    terminal::enable_raw_mode()?;
    w.execute(terminal::EnterAlternateScreen)?;
    w.execute(cursor::Hide)?;
    //TODO use kitty someday w.execute(event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;
    // Prepare and run main update loop
    let keybinds = HashMap::from([
        (dq::Keycode::Left, Button::MoveLeft),
        (dq::Keycode::Right, Button::MoveRight),
        (dq::Keycode::A, Button::RotateLeft),
        (dq::Keycode::D, Button::RotateRight),
        (dq::Keycode::Down, Button::DropSoft),
        (dq::Keycode::Up, Button::DropHard),
    ]);
    let mut settings = Settings { keybinds }; // Application settings
    let mut active_screens = vec![Screen::Title]; // Active screens
    loop {
        // Retrieve active screen, stop application if all exited
        let Some(screen) = active_screens.last_mut() else {
            break;
        };
        // Enter screen until it returns what to do next
        let (screens_pop, screens_push) = match screen {
            Screen::Title => enter_title_screen(w),
            Screen::Settings => enter_settings(w, &mut settings),
            Screen::Gaming(game) => enter_game(w, &settings, game),
        }?;
        // Change screen session depending on what response screen gave
        active_screens.truncate(active_screens.len() - screens_pop);
        active_screens.extend(screens_push);
    }
    // Deinitialize console
    w.execute(style::ResetColor)?;
    w.execute(cursor::Show)?;
    w.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    //TODO use kitty someday w.execute(event::PopKeyboardEnhancementFlags)?;
    Ok(())
}