mod console;
mod game_logic;

fn main() -> Result<(), std::io::Error> {
    console::run(&mut std::io::stdout())
}