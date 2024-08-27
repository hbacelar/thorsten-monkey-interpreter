use anyhow::{Context, Result};
use std::io::{BufRead, Read, Write};

use crate::{environment::Environment, evaluator::Evaluator, lexer::Lexer, parser::Parser};

pub struct Repl<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> Repl<R, W>
where
    R: Read + BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Repl { reader, writer }
    }
    pub fn start(mut self) -> Result<()> {
        let mut repl_env = Environment::default();
        loop {
            write!(self.writer, ">> ").context("unable to write to stdout")?;
            self.writer.flush().context("unable to flush writer")?;
            let mut buffer = String::new();
            self.reader
                .read_line(&mut buffer)
                .context("failed to read line")?;

            let lexer = Lexer::new(&buffer);
            let parser = Parser::new(lexer);

            let program = parser.parse_program();
            if !program.errors.is_empty() {
                for err in program.errors {
                    writeln!(self.writer, "Error {}", err).context("unable to write to stdout")?;
                }
            } else {
                let obj = Evaluator::eval(program, &mut repl_env)?;
                writeln!(self.writer, "{}", obj).context("unable to write to stdout")?;
            }
        }
    }
}
