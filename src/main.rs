mod backend;
mod frontend;

fn main() -> Result<(), std::io::Error> {
    let msg = frontend::console::run(&mut std::io::stdout())?;
    println!("{msg}");
    Ok(())
}
