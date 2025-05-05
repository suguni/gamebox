use crate::game::{Board, Direction};
use rand::prelude::*;

mod game;

fn main() {
    let mut board = Board::new();
    board.print();

    let mut rng = rand::rng();
    let dirs = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    while !board.is_end() {
        let dir = dirs.choose(&mut rng).unwrap();
        board.slide(dir);

        println!("{:?}", dir);
        board.print();
    }
}
