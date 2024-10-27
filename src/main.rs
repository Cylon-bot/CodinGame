use std::collections::HashMap;

use entities::{ash::Ash, human::get_humans, zombie::get_zombies};
mod entities;

fn main() {
    // game loop
    let mut ash = Ash::new();
    let mut humans = HashMap::new();
    let mut zombies = HashMap::new();

    loop {
        ash.change_position();
        get_humans(&mut humans);
        get_zombies(&mut zombies);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("500 500"); // Your destination coordinates
    }
}
