use std::env;

use advent::Challenge;

fn main() {
    let args: Vec<String> = env::args().collect();
    let challenge = <&dyn Challenge>::try_from(&args[1]).unwrap();
    println!("Day {} answer {}", args[1], challenge.first().unwrap());
}
