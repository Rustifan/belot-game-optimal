mod game;
use game::{round::Round};

fn main() {
    
    let round = Round::new();

    println!("{round:#?}");
}
