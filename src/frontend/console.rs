use std::{
    collections::HashMap, io::{self, Write}, sync::mpsc, time::{Duration, Instant}
};

//use device_query;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode as ctKeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style,
    terminal,
    ExecutableCommand, QueueableCommand,
};
use device_query::{keymap::Keycode as dqKeyCode, DeviceEvents};

use crate::backend::game::{Button, ButtonChange, Game, Gamemode};

const GAME_FPS: f64 = 60.0; // 60fps

struct Settings {
    keybinds: HashMap<dqKeyCode, Button>,
    //TODO information stored throughout application?
}

enum Menu {
    Title,
    NewGame(Gamemode),
    Game(Box<Game>),
    Pause,
    GameOver,
    GameComplete,
    Options,
    Replay,
    Scores,
    Quit(String),
    ConfigureControls,
}

enum MenuUpdate {
    Pop,
    Push(Menu),
    SetTo(Menu),
}

impl Menu {
    fn title(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!()/*TODO
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

    fn newgame(w: &mut dyn Write, gamemode: &mut Gamemode) -> io::Result<MenuUpdate> {
        todo!() //TODO
    }

    fn game(w: &mut dyn Write, settings: &Settings, game: &mut Game) -> io::Result<MenuUpdate> {
        // Prepare channel with which to communicate Button inputs / game interrupt
        let (sx1, rx) = mpsc::channel();
        let sx2 = sx1.clone();
        let keybinds1 = std::sync::Arc::new(settings.keybinds.clone());
        let keybinds2 = keybinds1.clone();
        // Initialize callbacks which send Button inputs
        let device_state = device_query::DeviceState::new();
        let _guard1 =  device_state.on_key_down(move |key| {
            let instant = Instant::now();
            let signal = match key {
                // Escape pressed: send interrupt
                dqKeyCode::Escape => None,
                _ => match keybinds1.get(key) {
                    // Button pressed with no binding: ignore
                    None => return,
                    // Button pressed with binding
                    Some(&button) => Some((button, true, instant)),
                }
            };
            let _ = sx1.send(signal);
        });
        let _guard2 =  device_state.on_key_up(move |key| {
            let instant = Instant::now();
            let signal = match key {
                // Escape released: ignore
                dqKeyCode::Escape => return,
                _ => match keybinds2.get(key) {
                    // Button pressed with no binding: ignore
                    None => return,
                    // Button released with binding
                    Some(&button) => Some((button, false, instant)),
                }
            };
            let _ = sx2.send(signal);
        });
        // Game Loop
        let game_loop_start = Instant::now();
        for i in 1u32.. {
            let next_frame = game_loop_start + Duration::from_secs_f64(f64::from(i) / GAME_FPS);
            let frame_delay = next_frame - Instant::now();
            let finish_status = match rx.recv_timeout(frame_delay) {
                Ok(None) => {
                    return Ok(MenuUpdate::Push(Menu::Pause))
                }
                Ok(Some((button, is_press_down, instant))) => {
                    let mut changes = ButtonChange::default();
                    changes[button] = Some(is_press_down);
                    game.update(Some(changes), instant)
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    let now = Instant::now();
                    game.update(None, now)
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Ok(MenuUpdate::Pop) //TODO print debug for why game crashes here
                }
            };

            if let Some(good_end) = finish_status {
                let menu = if good_end { Menu::GameComplete } else { Menu::GameOver };
                return Ok(MenuUpdate::Push(menu));
            }

            //TODO draw game
            let visuals = game.get_visuals();
            let stats = game.get_stats();
        }
        Ok(MenuUpdate::Push(Menu::Quit(String::from("TODO (currently Menu::game default exit)")))) //TODO
    }

    fn pause(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() //TODO
    }

    fn gameover(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() //TODO
    }

    fn gamecomplete(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() //TODO
    }

    fn options(w: &mut dyn Write, settings: &mut Settings) -> io::Result<MenuUpdate> {
        todo!() //TODO
    }

    fn configurecontrols(w: &mut dyn Write, settings: &mut Settings) -> io::Result<MenuUpdate> {
        todo!() //TODO
    }

    fn replay(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() //TODO
    }

    fn scores(w: &mut dyn Write) -> io::Result<MenuUpdate> {
        todo!() //TODO
    }
}

pub fn run(w: &mut impl Write) -> io::Result<String> {
    // Initialize console
    terminal::enable_raw_mode()?; //TODO use kitty someday w.execute(event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;
    w.execute(terminal::EnterAlternateScreen)?;
    w.execute(cursor::Hide)?; 
    // Prepare to run main tui loop
    let keybinds = HashMap::from([
        (dqKeyCode::Left, Button::MoveLeft),
        (dqKeyCode::Right, Button::MoveRight),
        (dqKeyCode::A, Button::RotateLeft),
        (dqKeyCode::D, Button::RotateRight),
        (dqKeyCode::Down, Button::DropSoft),
        (dqKeyCode::Up, Button::DropHard),
    ]);
    let mut settings = Settings { keybinds };
    let mut menu_stack = vec![Menu::Game(Box::new(Game::new(Gamemode::marathon())))]; //TODO make this Menu::Title
    let msg = loop {
        // Retrieve active menu, stop application if stack is empty
        let Some(screen) = menu_stack.last_mut() else {
            break String::from("all menus exited");
        };
        // Handle/open menu
        let menu_update = match screen {
            Menu::Title => Menu::title(w),
            Menu::NewGame(gamemode) => Menu::newgame(w, gamemode),
            Menu::Game(game) => Menu::game(w, &settings, game),
            Menu::Pause => Menu::pause(w),
            Menu::GameOver => Menu::gameover(w),
            Menu::GameComplete => Menu::gamecomplete(w),
            Menu::Options => Menu::options(w, &mut settings),
            Menu::ConfigureControls => Menu::configurecontrols(w, &mut settings),
            Menu::Replay => Menu::replay(w),
            Menu::Scores => Menu::scores(w),
            Menu::Quit(string) => break string.clone(), //TODO optimize
        }?;
        // Change screen session depending on what response screen gave
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
    // Deinitialize console
    w.execute(style::ResetColor)?;
    w.execute(cursor::Show)?;
    w.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?; //TODO use kitty someday w.execute(event::PopKeyboardEnhancementFlags)?;
    Ok(msg)
}