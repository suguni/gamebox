use crate::game::{Board, Direction};
use rand::prelude::*;
use std::io;
use std::io::{stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod game;

fn main() -> Result<(), std::io::Error> {
    let mut board = Board::new();

    let mut stdout = io::stdout().into_raw_mode()?;
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;

    board.print(&mut stdout)?;

    let stdin = stdin();
    for c in stdin.events() {
        let dir = match c? {
            termion::event::Event::Key(Key::Up) => Some(&Direction::Up),
            termion::event::Event::Key(Key::Down) => Some(&Direction::Down),
            termion::event::Event::Key(Key::Left) => Some(&Direction::Left),
            termion::event::Event::Key(Key::Right) => Some(&Direction::Right),
            termion::event::Event::Key(Key::Char('q')) => None,
            _ => None,
        };

        if let Some(dir) = dir {
            board.slide(dir);
            println!("{:?}\r", dir);
            board.print(&mut stdout)?;
            stdout.flush()?;
        } else {
            break;
        }

        if board.is_end() {
            println!("Game Over!");
            break;
        }
    }

    Ok(())
}
