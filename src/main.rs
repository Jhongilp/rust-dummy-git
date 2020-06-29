use std::io;
use std::io::{Read,Write};

fn main() -> Result<(), crossterm::ErrorKind> {
    // println!("Hello, world!");
    
    // crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();
    stdout.write_all(b"hello world\n").unwrap();

    let mut stdin = io::stdin().bytes();
    // for byte in stdin {
    loop {
        // println!("byte {}", byte.unwrap());
        write!(stdout, "Type something > ").unwrap();
        
        // stdout.flush().unwrap();
        stdout.flush()?;
        
        // let byte = stdin.next().unwrap().unwrap();
        let byte = match stdin.next() {
            Some(b) => b?,
            None => break,
        };
        
        let c = char::from(byte);

        if c == 'q' {
            break;
        }

        // write!(stdout, "You typed '{}'\n", c).unwrap();
        write!(stdout, "You typed '{}'\n", c)?;
        stdout.flush()?;
    }

    crossterm::terminal::disable_raw_mode().unwrap();

    Ok(())
}
// fn main() {
//     // println!("Hello, world!");
    
//     crossterm::terminal::enable_raw_mode().unwrap();

//     let mut stdout = io::stdout();
//     stdout.write_all(b"hello world\n").unwrap();

//     let mut stdin = io::stdin().bytes();
//     // for byte in stdin {
//     loop {
//         // println!("byte {}", byte.unwrap());
//         write!(stdout, "Type something > ").unwrap();
//         stdout.flush().unwrap();

//         // let c = char::from(byte.unwrap());
//         let c = char::from(stdin.next().unwrap().unwrap());

//         if c == 'q' {
//             break;
//         }

//         // write!(stdout, "You typed '{}'\n", c).unwrap();
//         write!(stdout, "You typed '{}'\n", c).unwrap();
//         stdout.flush().unwrap();
//     }

//     crossterm::terminal::disable_raw_mode().unwrap();
// }
