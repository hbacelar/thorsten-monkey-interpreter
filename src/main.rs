use anyhow::Result;
use thorsten_interpreter::repl::Repl;
use std::io::{self};

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
