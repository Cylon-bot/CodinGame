use std::{collections::HashMap, io};

use crate::{math_tools::find_distance, parse_input};

use super::human::Human;
pub struct Ash {
    pub x: i32,
    pub y: i32,
    pub nearest_human_id: i32,
    pub distance_to_nearest_human: i32,
}
impl Ash {
    pub fn new() -> Self {
        Ash {
            x: 0,
            y: 0,
            nearest_human_id: 0,
            distance_to_nearest_human: 0,
        }
    }

    pub fn change_position(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        self.x = parse_input!(inputs[0], i32);
        self.y = parse_input!(inputs[1], i32);
    }
    pub fn find_nearest_human(&mut self, humans: &HashMap<i32, Human>) {
        let mut distance_to_human = Vec::new();
        for human in humans.iter() {
            let distance = find_distance(human.1.x, human.1.y, self.x, self.y);
            distance_to_human.push((human.0, distance))
        }
        // Find the tuple with the minimum second element
        if let Some(min_tuple) = distance_to_human.iter().min_by_key(|&tuple| tuple.1) {
            self.nearest_human_id = *min_tuple.0;
            self.distance_to_nearest_human = min_tuple.1;
        }
    }
}
