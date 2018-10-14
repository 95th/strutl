extern crate gs;

use gs::JaroWinkler;

fn main() {
    let jw = JaroWinkler::new();
    let score = jw.apply("Cheeseburger", "Cheese fries");
    println!("Got score: {}", score);
}
