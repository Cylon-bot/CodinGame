use std::collections::HashMap;

use entities::{ash::Ash, human::get_humans, zombie::get_zombies};
mod entities;
mod math_tools;
fn main() {
    // game loop
    let mut ash = Ash::new();
    let mut humans = HashMap::new();
    let mut zombies = HashMap::new();

    loop {
        ash.change_position();
        get_humans(&mut humans);
        get_zombies(&mut zombies);
        for zombie in zombies.values_mut() {
            zombie.find_nearest_human(&humans);
        }
        if let Some(nearest_zombie_from_human) = zombies
            .clone()
            .into_values()
            .min_by_key(|z| z.nearest_human_id)
        {
            println!(
                "{:?} {:?} I WILL KILL YOU",
                nearest_zombie_from_human.next_x, nearest_zombie_from_human.next_y
            );
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
    }
}
