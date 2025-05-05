use std::io::{Stdout, Write};
use rand::prelude::*;
use termion::raw::RawTerminal;

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

const BOARD_SIZE: usize = 4;

impl Board {
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
        !self.nums.iter().any(|&x| x == 0)
    }

    pub fn slide(&mut self, dir: &Direction) -> bool {
        if *dir == Direction::Up || *dir == Direction::Down {
            self.transpose();
        }

        let merge_dir = match *dir {
            Direction::Left | Direction::Up => MergeDir::Start,
            Direction::Right | Direction::Down => MergeDir::End,
        };

        for stack in self.nums.chunks_mut(BOARD_SIZE) {
            merge(stack, merge_dir);
        }

        if *dir == Direction::Up || *dir == Direction::Down {
            self.transpose();
        }

        // TODO 변경이 없으면 새로 spawn 하면 안됨
        self.spawn_new_number()
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
}
