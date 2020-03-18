pub mod game {
    mod game_logic {
        use rand::{thread_rng, Rng};

        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Animal {
            Human,
            Bird,
            Shark,
        }

        impl Animal {
            pub fn get_color(&self) -> Box<dyn termion::color::Color> {}
            pub fn get_symbol(&self) -> u32 {
                match self {
                    Animal::Human => 1,
                    Animal::Bird => 2,
                    Animal::Shark => 3,
                }
            }
        }
        #[derive(Clone, Copy, Debug)]
        struct Cell {
            x: usize,
            y: usize,
            alive: bool,
            animal: Animal,
        }

        impl Cell {
            pub fn new(x: usize, y: usize) -> Cell {
                let alive: u8;
                let symbol: char;
                let mut rng = thread_rng();
                let animal = match rng.gen_range(0, 10) {
                    0 => Animal::Human,
                    1 => Animal::Bird,
                    _ => Animal::Shark,
                };
                Cell {
                    x: x,
                    y: y,
                    alive: rand::random(),
                    animal: animal,
                }
            }
        }
        impl std::fmt::Display for Cell {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.alive)
            }
        }
        #[derive(Debug)]
        pub struct Board {
            cells: Vec<Vec<Cell>>,
        }
        impl Board {
            pub fn new(size: usize) -> Board {
                let mut outer: Vec<Vec<Cell>> = Vec::with_capacity(size);
                for x in 0..size {
                    let mut inner: Vec<Cell> = Vec::with_capacity(size);
                    for y in 0..size {
                        inner.push(Cell::new(x, y));
                    }
                    outer.push(inner);
                }
                Board { cells: outer }
            }
            pub fn print(&self) {
                for x in &self.cells {
                    for y in x {
                        if y.alive {
                            match y.animal {
                                Animal::Human => {
                                    print!("{} {}", y.animal.get_symbol(), termion::color::Red);
                                }
                                Animal::Bird => {
                                    print!("{} {}", y.animal.get_symbol(), termion::color::Cyan);
                                }
                                Animal::Shark => {
                                    print!("{} {}", y.animal.get_symbol(), termion::color::Blue);
                                }
                            };
                        }
                        print!("{} ", y.alive);
                        print!("{}", termion::color::Fg(termion::color::Reset));
                    }
                    println!("");
                }
            }
            pub fn evolve(&mut self) {
                let cells_clone = self.cells.clone();
                for (i, x) in (&mut self.cells).iter_mut().enumerate() {
                    for (j, y) in x.iter_mut().enumerate() {
                        let mut neighbors = 0;
                        if i > 0
                            && cells_clone[i - 1][j].alive
                            && cells_clone[i - 1][j].animal == y.animal
                        {
                            neighbors += 1;
                        }
                        if j > 0
                            && cells_clone[i][j - 1].alive
                            && cells_clone[i][j - 1].animal == y.animal
                        {
                            neighbors += 1;
                        }
                        if i > 0
                            && j > 0
                            && cells_clone[i - 1][j - 1].alive
                            && cells_clone[i - 1][j - 1].animal == y.animal
                        {
                            neighbors += 1
                        }
                        if i > 0
                            && j < cells_clone.len() - 1
                            && cells_clone[i - 1][j + 1].alive
                            && cells_clone[i - 1][j + 1].animal == y.animal
                        {
                            neighbors += 1;
                        }
                        if i < cells_clone.len() - 1
                            && j > 0
                            && cells_clone[i + 1][j - 1].alive
                            && cells_clone[i + 1][j - 1].animal == y.animal
                        {
                            neighbors += 1;
                        }
                        if i < cells_clone.len() - 1
                            && cells_clone[i + 1][j].alive
                            && cells_clone[i + 1][j].animal == y.animal
                        {
                            neighbors += 1;
                        }
                        if j < cells_clone.len() - 1
                            && cells_clone[i][j + 1].alive
                            && cells_clone[i][j + 1].animal == y.animal
                        {
                            neighbors += 1
                        }
                        if i < cells_clone.len() - 1
                            && j < cells_clone.len() - 1
                            && cells_clone[i + 1][j + 1].alive
                            && cells_clone[i + 1][j + 1].animal == y.animal
                        {
                            neighbors += 1;
                        }

                        if y.alive {
                            if neighbors < 2 || neighbors > 3 {
                                y.alive = false;
                            }
                        } else {
                            if neighbors == 3 {
                                y.alive = true;
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn start() {
        println!("Game loop!");
        let mut b = game_logic::Board::new(30);
        loop {
            b.print();
            b.evolve();
            println!("-------------------------------------------------");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
