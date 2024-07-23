<div align="center"><img width="440" src="https://repository-images.githubusercontent.com/816034047/9eba09ef-d6da-4b4c-9884-630e7f87e102" /></div>


# Tetromino Game Engine + A Playable Terminal Implementation.

## How to run `tetrs_terminal`
Pre-compiled:
- Download a release for your platform if available and run the application.

Compiling yourself:
- Have the [Rust](https://www.rust-lang.org/) compiler (and Cargo) installed.
- [Download](<https://github.com/Strophox/tetrs/archive/refs/heads/main.zip>) (or `git clone`) this repo.
- Navigate to `tetrs/` (or `tetrs_terminal/`) and compile with `cargo run`.
- (Relevant keys [`Esc`,`Enter`,`←`,`→`,`↑`,`↓`,`A`,`D`] also shown inside the application)

Additional notes:
- Set the game framerate with `./tetrs_terminal --fps=60` (or `cargo run -- --fps=60`) (default is 30fps).
- Use a terminal like [kitty](<https://sw.kovidgoyal.net/kitty/>) for smoothest gameplay and visual experience. *Explanation:* Terminals do not usually send "key released" signals, which is a problem for mechanics such as "press left to move left repeatedly **until key is released**". We rely on [Crossterm](https://docs.rs/crossterm/latest/crossterm/event/struct.PushKeyboardEnhancementFlags.html) to automatically detect kitty-protocol-compatible terminals where this is not an issue (check page). In all other cases DAS/ARR is be determined by Keyboard/OS/terminal settings.)

## Usage of the `tetrs_engine`
Adding `tetrs_engine` as a [dependency from git](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) to your project:
```toml
[dependencies]
tetrs_engine = { git = "https://github.com/Strophox/tetrs.git" }
```
The tetrs engine shifts the responsibility of detecting player input, and choosing the time to update, to the user of the engine.
The key interactions with the engine look the following:
```rust
// Starting a game:
let game = tetrs_engine::Game::with_gamemode(gamemode, time_started);

// Updating the game with a new button state at a point in time:
game.update(Some(new_button_state), update_time);
// Updating the game with *no* change in button state (since the last):
game.update(None, update_time_2);

// Retrieving the game state (to render the board, active piece, next pieces, etc.):
let GameState { board, .. } = game.state();
```

# Features

## Frontend
TODO: `all` the tui pain here.
TODO: GIFs and screenshots.

## Engine
TODO: `all` the features here.


# Idea
