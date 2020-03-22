use clap::{value_t, App, Arg};
use game_of_life::start;

fn main() {
    let matches = App::new("Game of Rife")
        .version("1.0")
        .author("Carl Lundin <carllundin55@gmail.com>")
        .about("Conway's Game of Life Implemented in Rust!")
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("SIZE")
                .help("Sets the grid size for the game of life. Must be at least 5.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("total-animals")
                .short("n")
                .long("total-animals")
                .value_name("TOTAL-ANIMALS")
                .help("Sets the amount of animals for the game of life. Max is 10!")
                .takes_value(true),
        )
        .get_matches();

    let size = value_t!(matches, "size", usize).unwrap_or(25);
    if size < 5 {
        panic!("Board must be larger than 5x5.");
    }
    let total_animals = value_t!(matches, "total-animals", usize).unwrap_or(3);

    if total_animals > 10 || total_animals < 1 {
        panic!("Only 1-10 animals are supported!");
    }

    start(size, total_animals);
}
