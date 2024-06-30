use crate::game_logic::Game;
use std::{io::Write, thread::current};
use std::time::{Duration, Instant};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style,
    terminal,
    ExecutableCommand, QueueableCommand,
};

const REFRESH_PER_S: f64 = 180.0;
const DRAW_RATE: u64 = 3; // 60fps

struct Settings {
    // TODO information stored throughout application
}

enum ScreenUpdate {
    Remove,
    Keep,
    Add(Screen),
}

enum Screen {
    Main,
    Options,
    Gaming(Game),
}

impl Screen {
    fn update(&self, settings: &mut Settings, time: Instant) -> std::io::Result<ScreenUpdate> {
        match self {
            Screen::Main => {
                todo!() // TODO update_main(settings);
            }
            Screen::Options => {
                todo!() // TODO update_options(settings);
            }
            Screen::Gaming(game) => {
                game.update(settings, time)
            }
        }
    }

    fn draw(&self, w: &mut impl Write) -> std::io::Result<()> {
        match self {
            Screen::Main => {
                todo!() // TODO draw_main(w);
            }
            Screen::Options => {
                todo!() // TODO draw_options(w);
            }
            Screen::Gaming(g) => {
                todo!() // TODO draw_game(w, g)
            }
        }
    }
}

fn draw_main(w: &mut dyn Write) -> std::io::Result<()> {
    todo!() // TODO implement drawing main screen
}

fn draw_options(w: &mut dyn Write) -> std::io::Result<()> {
    todo!() // TODO implement drawing options screen
}

fn draw_game(w: &mut dyn Write, g: &Game) -> std::io::Result<()> {
    todo!() // TODO implement drawing game
}

fn update_main(settings: &Settings) -> std::io::Result<ScreenUpdate> {
    todo!() // TODO implement handle main screen
}

fn update_options(settings: &mut Settings) -> std::io::Result<ScreenUpdate> {
    todo!() // TODO implement handle options screen
}

pub fn run(w: &mut impl Write) -> std::io::Result<()> {
    // Setup console
    w.execute(terminal::EnterAlternateScreen)?;
    // TODO support kitty someday w.execute(event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;
    terminal::enable_raw_mode()?;

    // Prepare and run main update loop
    let mut settings = Settings {}; // Application settings
    let mut active_screens = vec![Screen::Main]; // Active screens
    'update_loop: for tick in 0u64.. {
        let time_start = Instant::now();

        // Retrieve active screen, stop application if all were dropped
        let Some(screen) = active_screens.last() else {
            break;
        };

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
        }

        // Update state
        match screen.update(&mut settings, time_start)? {
            ScreenUpdate::Remove => { active_screens.pop(); },
            ScreenUpdate::Keep => { }
            ScreenUpdate::Add(new_screen) => { active_screens.push(new_screen); }
        }

        // Possibly do draw this frame
        if tick % DRAW_RATE == 0 {
            screen.draw(w)?;
        }

        // Idle the remaining time of this frame
        let delay = Duration::from_secs_f64(1.0 / REFRESH_PER_S);
        let elapsed = Instant::now() - time_start;
        std::thread::sleep(delay - elapsed);
    }
    
    w.execute(style::ResetColor)?;
    w.execute(cursor::Show)?;
    w.execute(terminal::LeaveAlternateScreen)?;
    // TODO support kitty someday w.execute(event::PopKeyboardEnhancementFlags)?;
    terminal::disable_raw_mode()?;
    Ok(())
}