<div align="center"><img width="440" src="https://repository-images.githubusercontent.com/816034047/9eba09ef-d6da-4b4c-9884-630e7f87e102" /></div>


# Tetromino Game Engine + Playable Terminal Application

This repository contains
- `tetrs_terminal`, a simple, polished, efficient, cross-platform TUI implementation of the prototypical game experience, and
- `tetrs_engine`, an abstract tetromino engine implementing a game interface with modern mechanics.


## How to run
*Pre-compiled:*
- Download a release for your platform if available and run the application.

*Compiling yourself:*
- Have [Rust](https://www.rust-lang.org/) installed.
- Download / `git clone` this repository.
- Navigate to `tetrs_terminal/` and `cargo run`.

> [!NOTE]
> Use a terminal like [kitty](<https://sw.kovidgoyal.net/kitty/>) (or any terminal with [support for progressive keyboard enhancement](https://docs.rs/crossterm/latest/crossterm/event/struct.PushKeyboardEnhancementFlags.html)) for smooth gameplay **controls** and/or visual experience. 
> 
> > <details>
> > 
> > <summary> Explanation. </summary>
> > 
> > Terminals do not usually send "key released" signals, which is a problem for mechanics such as "press left to move left repeatedly **until key is released**".
> > [Crossterm](https://docs.rs/crossterm/latest/crossterm/) automatically detects ['kitty-protocol'-compatible terminals]([https://docs.rs/crossterm/latest/crossterm/event/struct.PushKeyboardEnhancementFlags.html](https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement)) where this issue is solved.
> > Otherwise DAS/ARR will be determined by Keyboard/OS/terminal emulator settings.
> > *(This also affects Soft Drop, which with kitty can be held with the piece hitting ground without immediately locking piece.)*
> > 
> > </details>


## Gallery

<!--TODO: GIFs and screenshots.-->


## Features of the Application

**Gamemodes**
- Marathon, Sprint, Ultra, Master, Endless.
- Puzzle Mode: Find all perfect clears through some [*Ocular Rotation System*](#ocular-rotation-system) piece acrobatics (one retry per puzzle stage).
- Custom Mode: level start, level increment, limit *(Time, Score, Pieces, Lines, Level; None)*.

**Gameplay**
- Familiar game experience with moving, rotating, hard- and softdropping *tetrominos*.
- Colored pieces (guideline).
- Next piece preview (N=1).
- Ghost piece.
- Animations for: Hard drops, Line clears and Piece locking.
- Current game stats: Level, Score, Lines, Time, Pieces generated.
- For technical details see [Features of the Tetrs Engine](#features-of-the-tetrs-engine).
  
**Scoreboard**
- (stored to / loaded from local *tetrs_terminal_scores.json* if possible).
  
**Settings**
- Toggleable graphics (colored Unicode <-> oldschool, monochrome ASCII).
- Adjustable render rate and toggleable FPS counter.
- Rotation systems: *Ocular*, *Classic* and *Super*.
- Configurable controls.
  <details>
  
  <summary> Default Game Controls </summary>
  
  | Key | Action |
  | -: | :-: |
  | `A` | Rotate left |
  | `D` | Rotate right |
  | (not set) | Rotate around/180° |
  | `←` | Move left |
  | `→` | Move right |
  | `↓` | Soft drop |
  | `↑` | Hard drop |
  | `Esc` | Pause game |
  | `Ctrl`+`D` | Forfeit game |
  | `Ctrl`+`C` | Exit program |
  
   </details>


## Features of the Tetrs Engine

The frontend application is proof-of-concept;
Ultimately the tetrs engine tries to be modular and shifts the responsibility of detecting player input and chosen time of updates to the client.
Basic interaction with the engine could look like the following:

```rust
// Starting a game.
let game = tetrs_engine::Game::with_gamemode(gamemode, time_started);
// Application loop.
loop {
  // Updating the game with a new button state at a point in time.
  game.update(Some(new_button_state), update_time);
  // Updating the game with *no* change in button state (since the last).
  game.update(None, update_time_2);
  // Retrieving the game state (to render the board, active piece, next pieces, etc.).
  let GameState { board, .. } = game.state();
}
```

<details>

<summary> Use tetrs_engine as a dependency with Cargo </summary>

Adding `tetrs_engine` as a [dependency from git](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) to your project:
```toml
[dependencies]
tetrs_engine = { git = "https://github.com/Strophox/tetrs.git" }
```

</details>


The engine at the time aims to be an interface to a feature-rich session of a singleplayer game.
The goal was to strike a balance between interesting and useful game mechanics as present in modern games, yet leaving away all that "seems unnecessary".
Rust, known for its performance and safety, proved to be an apt choice for this.

<details>

<summary> Game Configuration Aspects </summary>

- Gamemodes: Marathon, Sprint, Ultra, Master; Custom, given a playing limit, start lvl, whether to increment level.
- Rotation Systems: *Ocular Rotation System*, *Classic Rotation System*, *Super Rotation System*.
- Tetromino Generators: *Bag*, *Recency-based*, *Uniformly random*.
- Piece Preview (default N = 1)
- Delayed Auto Shift (default DAS = 200ms)
- Auto Repeat Rate (default ARR = 50ms)
- Soft Drop Factor (default SDF = 15.0)
- Hard drop delay (default at 0.1ms)
- Line clear delay (default at 200ms)
- Appearance Delay (default ARE = 100ms)

Currently, drop delay and lock delay\* *(\*But not total ground time)* are a function of the current level:
- Drop delay (1000ms at lvl 1 to 0.833ms ("20G") at lvl 19).
- 'Timer' variant of Extended Placement Lockdown (step reset); The piece tries to lock every 500ms at lvl 19 to every 150ms at lvl 30, and any given piece may only touch ground for 2250ms in total. See also [Piece Locking](#piece-locking).

</details>

<details>

<summary> Game State Aspects </summary>

- Time: Game time is held abstract as "time elapsed since game started" and is not directly tied to real-world timestamps.
- Game finish: The game knows if it finished, and if session was won or lost. Game Over scenarios are:
  - Block out: newly piece spawn location is occupied.
  - Lock out: a piece was completely locked above the skyline (row 21 and above).
- Event queue: All game events are kept in an internal queue that is stepped through, up to the provided timestamp of a `Game::update` call.
- Buttons pressed state: The game keeps an abstract state of which buttons are currently pressed.
- Board state: Yes.
- Active piece: The active piece is stored as a (tetromino, orientation, position) tuple plus some locking data.
- Next Pieces: Are kept in a queue and can be viewed.
- Pieces played so far: Kept as a stat.
- Lines cleared: <sup>yeah</sup>
- (Speed) Level: Increases every 10 line clears and influences only drop/lock delay.
- Scoring: Line clears trigger a score bonus, which takes into account number of lines cleared, spins, combos, back-to-backs.
  <details>
  
  <summary>Scoring Details</summary>
  
  ```haskell
  score_bonus = 10
              * (lines ^ 2)
              * (if spin then 4 else 1)
              * (if perfect then 16 else 1)
              * combo
              * maximum [1, backToBack * backToBack]
    where lines = "number of lines cleared simultaneously"
          spin = "piece could not move up when locking occurred"
          perfect = "board is empty after line clear"
          combo = "number of consecutive pieces where line clear occurred"
          backToBack = maximum [1, "number of consecutive line clears where spin, perfect or quadruple line clear occurred"]
  ```
  A table of some bonuses is provided:
  | Score bonus | Action |
  | -: | :- |
  | +10 | Single |
  | +40 | Double |
  | +90 | Triple |
  | +160 | Quadruple |
  | +20 | Single (2.combo) |
  | +30 | Single (3.combo) |
  | +80 | Double (2.combo) |
  | +120 | Double (3.combo) |
  | +40 | ?-Spin Single |
  | +160 | ?-Spin Double |
  | +360 | ?-Spin Triple |

  </details>
  
</details>


<details>

<summary> Game Feedback Aspects </summary>

The game provides some useful feedback events upon every `update`, usually used to correctly implement visual effects:
- *Piece locked down*, *Lines cleared*, *Hard drop*, *Accolade* (score bonus info), *Message* (generic message, currently unused)

</details>

Also see documentation (`cargo doc --open`).


## Project Highlights


### Ocular Rotation System

TODO <!--https://youtu.be/6YhkkyXydNI?si=jbVwfNtfl5yFh9Gk&t=674-->


### Piece Locking

TODO


### Scoring

Coming up with a good score system is tough, and experience and playtesting helps, so the one I come up with probably sucks ("how many points should a 'perfect clear' receive?"). Even so, I went along and experimented, since I liked the idea of [rewarding all spins](https://harddrop.com/wiki/List_of_twists).


### Gamemodes

Initially a lot of architectural decisions were not clear; as such the question *what is the goal of this game?*
My findings were:
- There are several game stats one can keep track of, and
- Canonical / commonly found gamemodes can be approximated as a combination of `(stat which is limited so game can complete) + (stat which player aims to optimize)`.
Examples:
- *'Marathon':* limit Level to 20, try for highest score.
- *'Sprint'* / *'40 lines'*: limit lines to 40, try for lowest time.
- *'Ultra'* / *'Time Trial'*: limit time to 2-3min, try for highest score / most lines.
The real implementation additionally stores the (speed) level to start at, and whether clearing lines increments the level.
> [!NOTE]
> Given one stat, how do we know whether we want to maximize or minimize another arbitrary stat?
> I may be overlooking a simpler pattern, but it seems one can order all stats linearly, and given a stat to be fixed/limited, any other stat is maximized/minimized directly depending on whether it's further down/up the sequence:
> <details>
> 
> <summary>Gamemode Stat Relation Table</summary>
> 
> | name | finss | time  | piecs | lines | level | score |
> | ---- | ----- | ----- | ----- | ----- | ----- | ----- |
> |      |  fix  |  MAX  |       |       |       |
> |      |       |       |  MAX  |       |       |
> |      |  fix  |       |       |  MAX  |       |
> |      |  fix  |       |       |       |  MAX  |
> |      |  fix  |       |       |       |       |  MAX
> |      |  MIN  |  fix  |       |       |       |
> |      |       |  fix  |  MAX  |       |       |
> | *'Ultra*' |  |  fix  |       |  MAX  |       |
> |      |       |  fix  |       |       |  MAX  |
> |      |       |  fix  |       |       |       |  MAX
> |      |  MIN  |       |  fix  |       |       |
> |      |       |  MIN  |  fix  |       |       |
> |      |       |       |  fix  |  MAX  |       |
> |      |       |       |  fix  |       |  MAX  |
> |      |       |       |  fix  |       |       |  MAX
> |      |  MIN  |       |       |  fix  |       |
> | *'Sprint'* | |  MIN  |       |  fix  |       |
> |      |       |       |  MIN  |  fix  |       |
> |      |       |       |       |  fix  |  MAX  |
> |      |       |       |       |  fix  |       |  MAX
> |      |  MIN  |       |       |       |  fix  |
> |      |       |  MIN  |       |       |  fix  |
> |      |       |       |  MIN  |       |  fix  |
> |      |       |       |       |  MIN  |  fix  |
> | *'Marathon'* | |     |       |       |  fix  |  MAX
> |      |  MIN  |       |       |       |       |  fix
> |      |       |  MIN  |       |       |       |  fix
> |      |       |       |  MIN  |       |       |  fix
> |      |       |       |       |  MIN  |       |  fix
> |      |       |       |       |       |  MIN  |  fix
> 
> </details>


> So how does 'Puzzle Mode' work? - I can tell you how: with a pinch of state modeling jank and some not-so-secret internal state leakage via `Game::set_modifier`.

### Controls

Quick research on the 'best' or 'most ergonomic' game keybinds was [inconclusive](https://youtube.com/watch?v=6YhkkyXydNI&t=809). Upon sampling a few dozen on reddit posts it seems a 50/50 split on `←` `→` / `a` `d` or `z` `x` / `←` `→` for **move** / **rotate**. "Choose what feels best for you" some said - they're probably right. *(\*Even so, one should *not* hammer the spacebar for hard drops, the only button guideline suggests.)*

### Miscellaneous Author Notes

This project allowed me to have first proper learning experience with programming a larger Rust project, an interactive game (in the console), and the intricacies of the Game mechanics themselves (see [Features of the Tetrs Engine](#features-of-the-tetrs-engine)).

On the Rust side of things I learned about;
- Some [coding](https://docs.kernel.org/rust/coding-guidelines.html) [style](https://doc.rust-lang.org/nightly/style-guide/) [guidelines](https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/style.md#getters--setters) & `cargo fmt` (~`#[rustfmt::skip]`~),
- "[How to order Rust code](https://deterministic.space/how-to-order-rust-code.html)",
- introduction to [writing](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html) [documentation](https://rust-lang.github.io/api-guidelines/documentation.html) (and the fact they can [contain tested examples](https://blog.guillaume-gomez.fr/articles/2020-03-12+Guide+on+how+to+write+documentation+for+a+Rust+crate#Hiding-lines)) & `cargo doc`,
- the [`std` traits](https://rust-lang.github.io/api-guidelines/interoperability.html),
- using [serde](https://serde.rs/derive.html) a little for a hacky way to [save some structured data locally](https://stackoverflow.com/questions/62771576/how-do-i-save-structured-data-to-file),
- [conditionally derive](https://stackoverflow.com/questions/42046327/conditionally-derive-based-on-feature-flag) feature flags & `cargo check --features serde`,
- [clap](https://docs.rs/clap/latest/clap/) to parse simple command line arguments & `cargo run -- --fps=60`,
- [formatting](https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.format) the time with [chrono](https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#display-formatted-date-and-time) my favourite way,
- the `format!` macro (which I discovered is the analogue to Python's f-strings my beloved),
- using [Crossterm](https://crates.io/crates/crossterm) for the inputs (instead of something like [device_query](https://crates.io/crates/device_query) - also I did not end up using [ratatui](https://crates.io/crates/ratatui/) :c Someone will have to write a frontend with that)
- the [annoyances](https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement) of terminal emulators,
- the handy drop-in [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html) wrapper to diminish flickering,
- more practice with Rust's [module system](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html),
- multithreading with [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/)
- [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) to fully separate frontend and backend,
- [cargo git dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories) so other people *could* reuse the backend,
- and finally, [cross-compilation](https://blog.logrocket.com/guide-cross-compilation-rust/#how-rust-represents-platforms) for releases.

Gamedev-wise I learned about the [modern](https://gafferongames.com/post/fix_your_timestep/) [game](http://gameprogrammingpatterns.com/game-loop.html) [loop](https://dewitters.com/dewitters-gameloop/) and finding the proper abstraction for `Game::update` (allow arbitrary-time user input, make updates decoupled from framerate). I also spent time looking at the menu navigation of [Noita](https://noitagame.com/) to help me come up with my own.

<sup>~~Lastly, I also found that there already *are*, like, a billion other [`tetrs`](https://github.com/search?q=%22tetrs%22&type=repositories)'s on GitHub, oops.~~</sup>

*„Piecement Places.“* - [CTWC 2016](https://www.youtube.com/watch?v=RlnlDKznIaw&t=121).
