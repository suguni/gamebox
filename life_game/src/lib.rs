use rand::Rng;

pub struct Grid {
    pub cols: u32,
    pub rows: u32,
    pub cells: Vec<bool>,
}

impl Grid {
    pub fn new(cols: u32, rows: u32) -> Self {
        Self {
            cols,
            rows,
            cells: vec![false; (cols * rows) as usize],
        }
    }

    pub fn random(cols: u32, rows: u32) -> Self {
        let mut rng = rand::rng();
        let cells = (0..cols * rows).map(|_| rng.random::<bool>()).collect();
        Self { cols, rows, cells }
    }

    fn from_str(grid: &str) -> Self {
        let cells = grid
            .trim()
            .lines()
            .filter(|line| !line.is_empty())
            .flat_map(|line| line.chars().map(|c| c != '.').collect::<Vec<bool>>())
            .collect::<Vec<bool>>();
        let rows = grid.lines().count() as u32;
        let cols = cells.len() as u32 / rows;
        Self { cols, rows, cells }
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u32 {
        let left = i32::max(0, x as i32 - 1) as u32;
        let right = u32::min(self.cols - 1, x + 1);
        let top = i32::max(0, y as i32 - 1) as u32;
        let bottom = u32::min(self.rows - 1, y + 1);

        let mut count = 0;

        for row in top..=bottom {
            for col in left..=right {
                if !(row == y && col == x) && self.cells[(row * self.cols + col) as usize] {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn next_generation(&self) -> Grid {
        let mut next_cells = self.cells.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let p = (row * self.cols + col) as usize;
                let neighbor_count = self.live_neighbor_count(col, row);
                if self.cells[p] && (neighbor_count == 2 || neighbor_count == 3) {
                    next_cells[p] = true;
                } else if self.cells[p] && (neighbor_count <= 1 || neighbor_count >= 4) {
                    next_cells[p] = false;
                } else if !self.cells[p] && neighbor_count == 3 {
                    next_cells[p] = true;
                }
            }
        }
        Grid {
            cols: self.cols,
            rows: self.rows,
            cells: next_cells,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(3, 3);
        assert_eq!(grid.cols, 3);
        assert_eq!(grid.rows, 3);
        assert_eq!(grid.cells, vec![false; 9]);
    }

    #[test]
    fn test_from_str() {
        let grid = Grid::from_str(
            "...\n\
             .#.\n\
             ...",
        );

        assert_eq!(grid.cols, 3);
        assert_eq!(grid.rows, 3);
        assert_eq!(
            grid.cells,
            vec![false, false, false, false, true, false, false, false, false]
        );
    }

    #[test]
    fn test_live_neighbor_count() {
        let mut grid = Grid::new(3, 3);
        assert_eq!(grid.live_neighbor_count(1, 1), 0);

        grid.cells[0] = true;
        grid.cells[1] = true;
        grid.cells[2] = true;
        grid.cells[3] = true;
        grid.cells[5] = true;
        grid.cells[6] = true;
        grid.cells[7] = true;
        grid.cells[8] = true;

        assert_eq!(grid.live_neighbor_count(1, 1), 8);
        assert_eq!(grid.live_neighbor_count(0, 0), 2);
        assert_eq!(grid.live_neighbor_count(2, 0), 2);
        assert_eq!(grid.live_neighbor_count(0, 1), 4);
        assert_eq!(grid.live_neighbor_count(0, 2), 2);
    }

    #[test]
    fn test_next_generation() {
        let grid = Grid::new(3, 3);

        let grid = grid.next_generation();
        assert_dead_all(&grid);

        let grid = Grid::from_str(
            "...\n\
             .#.\n\
             ...",
        );

        let gen1 = Grid::from_str(
            "...\n\
             ...\n\
             ...",
        );

        assert_eq!(grid.next_generation().cells, gen1.cells);

        let gen0 = Grid::from_str(
            ".....\n\
             .###.\n\
             .....",
        );
        let gen1 = Grid::from_str(
            "..#..\n\
             ..#..\n\
             ..#..",
        );
        assert_eq!(gen0.next_generation().cells, gen1.cells);
        assert_eq!(gen1.next_generation().cells, gen0.cells);
    }

    #[test]
    fn block_pattern() {
        let grid = Grid::from_str(
            "....\n\
             .##.\n\
             .##.\n\
             ....",
        );
        assert_eq!(grid.next_generation().cells, grid.cells);
    }

    #[test]
    fn glider_pattern() {
        let g1 = Grid::from_str(
            ".....\n\
             ..#..\n\
             #.#..\n\
             .##..\n\
             .....",
        );

        let g2 = Grid::from_str(
            ".....\n\
             .#...\n\
             ..##.\n\
             .##..\n\
             .....",
        );

        let g3 = Grid::from_str(
            ".....\n\
             ..#..\n\
             ...#.\n\
             .###.\n\
             .....",
        );

        let g4 = Grid::from_str(
            ".....\n\
             .....\n\
             .#.#.\n\
             ..##.\n\
             ..#..",
        );

        let g5 = Grid::from_str(
            ".....\n\
             .....\n\
             ...#.\n\
             .#.#.\n\
             ..##.",
        );

        assert_eq!(g1.next_generation().cells, g2.cells);
        assert_eq!(g2.next_generation().cells, g3.cells);
        assert_eq!(g3.next_generation().cells, g4.cells);
        assert_eq!(g4.next_generation().cells, g5.cells);
    }

    fn assert_dead_all(grid: &Grid) {
        assert_eq!(grid.cells, vec![false; (grid.cols * grid.rows) as usize]);
    }
}
