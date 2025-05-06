use rand::prelude::*;
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

const BOARD_SIZE: usize = 4;

pub struct Board {
    nums: [u32; BOARD_SIZE * BOARD_SIZE],
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Board {
    pub fn new_with_nums(nums: [u32; BOARD_SIZE * BOARD_SIZE]) -> Self {
        Self { nums }
    }

    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut indices: Vec<usize> = (0..(BOARD_SIZE * BOARD_SIZE)).collect();
        indices.shuffle(&mut rng);

        let mut nums = [0; BOARD_SIZE * BOARD_SIZE];
        nums[indices[0]] = 2;
        nums[indices[1]] = 2;

        Self { nums }
    }

    pub fn print(&self, stdout: &mut RawTerminal<Stdout>) -> Result<(), std::io::Error> {
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                let value = self.nums[r * BOARD_SIZE + c];
                if value == 0 {
                    print!("|{:^5}", "");
                } else {
                    print!("|{:^5}", value);
                }
            }
            print!("|");
            println!("\r");
        }
        stdout.flush()?;
        Ok(())
    }

    pub fn is_end(&self) -> bool {
        if self.nums.iter().any(|&x| x == 0) {
            return false;
        }

        for a in 0..BOARD_SIZE {
            for b in 0..BOARD_SIZE - 1 {
                // row, a = row, b = column
                if self.nums[a * BOARD_SIZE + b] == self.nums[a * BOARD_SIZE + b + 1] {
                    return false;
                }

                // column, a = column, b = row
                if self.nums[b * BOARD_SIZE + a] == self.nums[(b + 1) * BOARD_SIZE + a] {
                    return false;
                }
            }
        }

        true
    }

    pub fn slide(&mut self, dir: &Direction) -> bool {
        if *dir == Direction::Up || *dir == Direction::Down {
            self.transpose();
        }

        let merge_dir = match *dir {
            Direction::Left | Direction::Up => MergeDir::Start,
            Direction::Right | Direction::Down => MergeDir::End,
        };

        let mut merged = false;
        for stack in self.nums.chunks_mut(BOARD_SIZE) {
            if can_merge(stack, merge_dir) {
                merge(stack, merge_dir);
                merged = true;
            }
        }

        if *dir == Direction::Up || *dir == Direction::Down {
            self.transpose();
        }

        merged || self.spawn_new_number()
    }

    fn spawn_new_number(&mut self) -> bool {
        let indices: Vec<usize> = self
            .nums
            .iter()
            .enumerate()
            .filter(|(_, num)| **num == 0)
            .map(|(i, _)| i)
            .collect();

        let mut rng = rand::rng();
        if let Some(pos) = indices.choose(&mut rng) {
            self.nums[*pos] = 2;
            true
        } else {
            false
        }
    }

    fn transpose(&mut self) {
        for row in 0..BOARD_SIZE {
            for col in row..BOARD_SIZE {
                if row != col {
                    let a = self.nums[row * BOARD_SIZE + col];
                    let b = self.nums[col * BOARD_SIZE + row];
                    self.nums[row * BOARD_SIZE + col] = b;
                    self.nums[col * BOARD_SIZE + row] = a;
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum MergeDir {
    Start,
    End,
}

fn merge(stack: &mut [u32], dir: MergeDir) {
    if stack.len() == 0 {
        return;
    }

    if dir == MergeDir::End {
        stack.reverse();
    }

    let mut pos = 0;

    for i in 0..stack.len() {
        if stack[i] == 0 || pos == i {
        } else {
            if stack[pos] == stack[i] {
                stack[pos] += stack[i];
                stack[i] = 0;
                pos += 1;
            } else {
                while pos != i && stack[pos] != 0 {
                    pos += 1;
                }

                if pos != i {
                    stack[pos] = stack[i];
                    stack[i] = 0;
                }
            }
        }
    }

    if dir == MergeDir::End {
        stack.reverse();
    }
}

fn can_merge(stack: &[u32], dir: MergeDir) -> bool {
    if dir == MergeDir::Start {
        for i in 1..stack.len() {
            if stack[i - 1] == 0 {
                for j in i..stack.len() {
                    if stack[j] != 0 {
                        return true;
                    }
                }
            } else if stack[i - 1] == stack[i] {
                return true;
            }
        }
    } else {
        for i in 1..stack.len() {
            if stack[stack.len() - i] == 0 {
                for j in i..stack.len() {
                    if stack[stack.len() - j - 1] != 0 {
                        return true;
                    }
                }
            } else if stack[stack.len() - i - 1] == stack[stack.len() - i] {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::MergeDir::*;
    use super::*;

    #[test]
    fn test_merge_empty() {
        let mut stack = vec![];
        merge(&mut stack, Start);
        assert_eq!(stack, Vec::<u32>::new());

        let mut stack = vec![];
        merge(&mut stack, End);
        assert_eq!(stack, Vec::<u32>::new());
    }

    #[test]
    fn test_merge_to_start() {
        let mut stack = vec![1, 1, 0, 0];
        merge(&mut stack, Start);
        assert_eq!(stack, vec![2, 0, 0, 0]);

        let mut stack = vec![1, 1, 1, 0];
        merge(&mut stack, Start);
        assert_eq!(stack, vec![2, 1, 0, 0]);

        let mut stack = vec![1, 1, 1, 1];
        merge(&mut stack, Start);
        assert_eq!(stack, vec![2, 2, 0, 0]);

        let mut stack = vec![1, 2, 1, 2];
        merge(&mut stack, Start);
        assert_eq!(stack, vec![1, 2, 1, 2]);

        let mut stack = vec![1, 2, 2, 2];
        merge(&mut stack, Start);
        assert_eq!(stack, vec![1, 4, 2, 0]);

        let mut stack = vec![2, 2, 2, 1];
        merge(&mut stack, Start);
        assert_eq!(stack, vec![4, 2, 1, 0]);

        let mut stack = vec![2, 0, 2, 0, 2, 0, 1];
        merge(&mut stack, Start);
        assert_eq!(stack, vec![4, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_merge_to_end() {
        let mut stack = vec![0, 0, 1, 1];
        merge(&mut stack, End);
        assert_eq!(stack, vec![0, 0, 0, 2]);

        let mut stack = vec![0, 1, 1, 1];
        merge(&mut stack, End);
        assert_eq!(stack, vec![0, 0, 1, 2]);

        let mut stack = vec![1, 1, 1, 1];
        merge(&mut stack, End);
        assert_eq!(stack, vec![0, 0, 2, 2]);

        let mut stack = vec![1, 2, 1, 2];
        merge(&mut stack, End);
        assert_eq!(stack, vec![1, 2, 1, 2]);

        let mut stack = vec![1, 2, 2, 2];
        merge(&mut stack, End);
        assert_eq!(stack, vec![0, 1, 2, 4]);
    }

    #[test]
    fn test_is_end() {
        let board = Board::new_with_nums([0_u32; BOARD_SIZE * BOARD_SIZE]);
        assert_eq!(board.is_end(), false);

        let board = Board::new_with_nums([2_u32; BOARD_SIZE * BOARD_SIZE]);
        assert_eq!(board.is_end(), false);

        let board = Board::new_with_nums([1, 2, 3, 4, 4, 5, 6, 7, 7, 8, 9, 10, 10, 11, 12, 13]);
        assert_eq!(board.is_end(), true);

        let board = Board::new_with_nums([1, 2, 1, 2, 2, 1, 2, 1, 2, 8, 9, 10, 10, 11, 12, 13]);
        assert_eq!(board.is_end(), false);
    }

    #[test]
    fn test_can_merge() {
        assert_eq!(can_merge(&[], Start), false);
        assert_eq!(can_merge(&[], End), false);

        assert_eq!(can_merge(&[1], Start), false);
        assert_eq!(can_merge(&[1], End), false);

        assert_eq!(can_merge(&[0], Start), false);
        assert_eq!(can_merge(&[0], End), false);

        assert_eq!(can_merge(&[1, 1], Start), true);
        assert_eq!(can_merge(&[1, 1], End), true);

        assert_eq!(can_merge(&[1, 2], Start), false);
        assert_eq!(can_merge(&[1, 2], End), false);

        assert_eq!(can_merge(&[1, 2, 0, 0], Start), false);
        assert_eq!(can_merge(&[1, 2, 0, 0], End), true);

        assert_eq!(can_merge(&[0, 0, 1, 2], Start), true);
        assert_eq!(can_merge(&[0, 0, 1, 2], End), false);

        assert_eq!(can_merge(&[0, 1, 2, 1], Start), true);
        assert_eq!(can_merge(&[0, 1, 2, 1], End), false);

        assert_eq!(can_merge(&[1, 0, 0, 1], Start), true);
        assert_eq!(can_merge(&[1, 0, 0, 1], End), true);
    }
}
