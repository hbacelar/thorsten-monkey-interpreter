use anyhow::{Context, Result};
use std::io::{BufRead, Read, Write};

use crate::{lexer::Lexer, token::Token};

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
        loop {
            write!(self.writer, ">> ").context("unable to write to stdout")?;
            self.writer.flush().context("unable to flush writer")?;
            let mut buffer = String::new();
            self.reader
                .read_line(&mut buffer)
                .context("failed to read line")?;

            let mut lexer = Lexer::new(buffer);
            loop {
                let token = lexer.next_token();
                writeln!(self.writer, "{:?}", token).context("unable to write to stdout")?;
                if token == Token::Eof {
                    break;
                }
            }
        }
    }
}
