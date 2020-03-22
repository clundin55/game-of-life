mod board;
use board::Board;

pub fn start(size: usize, total_animals: usize) {
    println!("Starting Conway's game of life!");
    let mut b = Board::new(size, total_animals);
    loop {
        println!("{}", b);
        b.evolve();
        println!("-------------------------------------------------");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
