mod console;
mod game_logic;
mod tetromino_generators;

fn main() -> Result<(), std::io::Error> {
    console::run(&mut std::io::stdout())
}