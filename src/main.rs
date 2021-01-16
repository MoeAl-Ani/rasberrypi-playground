mod led;
use sensehat::{SenseHat, Colour};

fn main() {
    let hat = sensehat::SenseHat::new();
    println!("{:?}", hat.get_pressure());
    hat.text("Hi!", Colour::RED, Colour::WHITE);
}
