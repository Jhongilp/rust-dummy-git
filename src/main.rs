use std::io;
use std::fmt;
use std::io::{Read,Write};

fn main() -> Result<(), Error> {
    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();
    stdout.write_all(b"hello world\n").unwrap();

    let mut stdin = io::stdin().bytes();
    loop {
        write!(stdout, "Type something > ").unwrap();
        stdout.flush()?;
        
        let byte = match stdin.next() {
            Some(b) => b?,
            None => break,
        };
        
        let c = char::from(byte);

        if c == 'q' {
            break;
        }

        write!(stdout, "You typed '{}'\n", c)?;
        stdout.flush()?;
    }

    crossterm::terminal::disable_raw_mode().unwrap();

    Ok(())
}

#[derive(Debug)]
enum Error {
    CrosstermError(crossterm::ErrorKind),
    IoError(io::Error),
}

impl From<crossterm::ErrorKind> for Error {
    fn from(error: crossterm::ErrorKind) -> Error {
        Error::CrosstermError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CrosstermError(inner) => write!(f, "{}", inner),
            Error::IoError(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::CrosstermError(inner) => Some(inner),
            Error::IoError(inner) => Some(inner),
        }
    }
}