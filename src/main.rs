use anyhow::Result;
use std::io::{self};
use thorsten_interpreter::repl::Repl;

fn main() -> Result<()> {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");

    let repl = Repl::new(io::stdin().lock(), io::stdout().lock());
    repl.start()?;

    Ok(())
}
