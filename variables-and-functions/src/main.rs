extern crate rand;

use rand::Rng;

fn main() {
    let name = "Racer";
    let mut range = rand::thread_rng();
    let pos = range.gen_range(1, 20);
    let mut message = String::new();

    if pos <= 2 {
        message = String::from("Congrats, you're starting in the front row!");
    } else if pos <= 18 {
        message = String::from("You're starting in the midfield, fight 'em hard!");
    } else {
        message = String::from("Well you'll be a backmarker in this race, good luck out there!");
    }

    println!("Hello, {}!", name);
    println!("Your starting position will be number #{}", pos);
    println!("{}", message);
}
