use crate::game::{Board, SlideDir};
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
            termion::event::Event::Key(Key::Up) => Some(SlideDir::Up),
            termion::event::Event::Key(Key::Down) => Some(SlideDir::Down),
            termion::event::Event::Key(Key::Left) => Some(SlideDir::Left),
            termion::event::Event::Key(Key::Right) => Some(SlideDir::Right),
            termion::event::Event::Key(Key::Char('q')) => None,
            _ => None,
        };

        if let Some(dir) = dir {
            if board.slide(dir) {
                println!("{:?}\r", dir);
                board.print(&mut stdout)?;
                stdout.flush()?;
            } else {
                println!("{:?} Block!\r", dir);
            }
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
