use rand::{thread_rng, Rng};

#[derive(Clone)]
pub enum Animal {
    Dog,
    Cat,
    Mouse,
    Fox,
    Rabbit,
    Bear,
    Panda,
    Koala,
    Cow,
    Lion,
}

impl std::fmt::Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let out = match self {
            Self::Dog => "🐶",
            Self::Cat => "🐱",
            Self::Mouse => "🐭",
            Self::Fox => "🦊",
            Self::Rabbit => "🐰",
            Self::Bear => "🐻",
            Self::Panda => "🐼",
            Self::Koala => "🐨",
            Self::Cow => "🐮",
            Self::Lion => "🦁",
        };
        write!(f, " {} ", out)
    }
}

fn random_animal(total_animals: usize) -> Animal {
    let mut rng = thread_rng();
    let n = rng.gen_range(0, total_animals);
    match n {
        0 => Animal::Dog,
        1 => Animal::Cat,
        2 => Animal::Mouse,
        3 => Animal::Fox,
        4 => Animal::Rabbit,
        5 => Animal::Bear,
        6 => Animal::Panda,
        7 => Animal::Koala,
        8 => Animal::Cow,
        _ => Animal::Lion,
    }
}

#[derive(Clone)]
pub struct Cell {
    pub animal: Animal,
    pub alive: bool,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.alive {
            return self.animal.fmt(f);
        }

        write!(f, " 🪦 ")
    }
}

impl Cell {
    pub fn new(total_animals: usize) -> Cell {
        Cell {
            animal: random_animal(total_animals),
            alive: rand::random(),
        }
    }
}
