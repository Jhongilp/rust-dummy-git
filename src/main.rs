use std::io;
use std::fmt;
use std::io::{Read,Write};
use git2::Repository;
use git2::BranchType;
use chrono::prelude::*;

fn main() -> Result<(), Error> {
    let repo = Repository::open_from_env()?;

    let mut stdout = io::stdout();

    for branch in repo.branches(Some(BranchType::Local))? {
        let (branch, branch_name) = branch?;
        let name = branch.name_bytes()?;
        stdout.write_all(name)?;
        write!(stdout, "\n")?;

        let commit = branch.get().peel_to_commit()?;
        println!("{}", commit.id());

        let time = commit.time();
        let time = NaiveDateTime::from_timestamp(time.seconds(), 0);
        println!("{}", time);
    }

    Ok(())
}


// fn main() -> Result<(), Error> {
//     crossterm::terminal::enable_raw_mode()?;

//     let mut stdout = io::stdout();
//     stdout.write_all(b"hello world\n").unwrap();

//     let mut stdin = io::stdin().bytes();
//     loop {
//         write!(stdout, "Type something > ").unwrap();
//         stdout.flush()?;
        
//         let byte = match stdin.next() {
//             Some(b) => b?,
//             None => break,
//         };
        
//         let c = char::from(byte);

//         if c == 'q' {
//             break;
//         }

//         write!(stdout, "You typed '{}'\n", c)?;
//         stdout.flush()?;
//     }

//     crossterm::terminal::disable_raw_mode().unwrap();

//     Ok(())
// }

// #[derive(Debug, thiserror::Error)]
// enum Error {
//     #[error(transparent)]
//     CrosstermError(#[from] crossterm::ErrorKind),
//     #[error(transparent)]
//     IoError(#[from] io::Error),
//     #[error(transparent)]
//     GitError(#[from] git2::Error),
// }

#[derive(Debug)]
enum Error {
    CrosstermError(crossterm::ErrorKind),
    IoError(io::Error),
    GitError(git2::Error)
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

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Error {
        Error::GitError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CrosstermError(inner) => write!(f, "{}", inner),
            Error::IoError(inner) => write!(f, "{}", inner),
            Error::GitError(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::CrosstermError(inner) => Some(inner),
            Error::IoError(inner) => Some(inner),
            Error::GitError(inner) => Some(inner),
        }
    }
}