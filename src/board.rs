mod cell;
use cell::Cell;

pub struct Board {
    cells: Vec<Cell>,
    size: usize,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut newline = 0;
        let mut board_representation = String::new();
        for x in 0..(self.size * self.size) {
            board_representation.push_str(&format!("{}", self.cells[x]));
            newline += 1;
            if newline % self.size == 0 {
                board_representation.push_str("\n");
            }
        }
        write!(f, "{}", board_representation)
    }
}

impl Board {
    pub fn new(size: usize, total_animals: usize) -> Board {
        let cells: Vec<Cell> = (0..size * size).map(|_| Cell::new(total_animals)).collect();
        Board { cells, size }
    }

    pub fn evolve(&mut self) {
        for x in 0..self.size as i32 {
            for y in 0..self.size as i32 {
                let index = self.size * x as usize + y as usize;
                let current_animal = self.cells[index].clone();

                let neighor_locations = vec![
                    (x - 1) * (self.size as i32) + y - 1,
                    (x - 1) * (self.size as i32) + y,
                    (x - 1) * (self.size as i32) + y + 1,
                    (x + 1) * (self.size as i32) + y - 1,
                    (x + 1) * (self.size as i32) + y,
                    (x + 1) * (self.size as i32) + y + 1,
                    (x) * (self.size as i32) + y - 1,
                    (x) * (self.size as i32) + y + 1,
                ];

                let possible_neighbors_inner: Vec<i32> = neighor_locations
                    .into_iter()
                    .filter(|cell| *cell != index as i32)
                    .filter(|cell| (*cell > 0) && (*cell < self.size as i32 * self.size as i32))
                    .filter(|cell| self.cells[*cell as usize].alive)
                    .collect();

                let possible_neighbors = possible_neighbors_inner.len();

                if current_animal.alive {
                    if possible_neighbors < 2 || possible_neighbors > 3 {
                        self.cells[index].alive = false;
                    }
                } else {
                    // Revive as a new animal!
                    if possible_neighbors == 3 {
                        self.cells[index].alive = true;
                    }
                }
            }
        }
    }
}
