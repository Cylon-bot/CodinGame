use std::{
    collections::{HashMap, HashSet},
    io,
};

use crate::parse_input;

pub struct Human {
    pub x: i32,
    pub y: i32,
}
impl Human {
    fn new(x: i32, y: i32) -> Self {
        Human { x, y }
    }
    fn change_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub fn get_humans(humans: &mut HashMap<i32, Human>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let human_count = parse_input!(input_line, i32);
    let mut alive_humans = Vec::new();
    for _ in 0..human_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs: Vec<&str> = input_line.split(" ").collect::<Vec<_>>();
        humans
            .entry(parse_input!(inputs[0], i32))
            .and_modify(|h: &mut Human| {
                h.change_position(parse_input!(inputs[1], i32), parse_input!(inputs[2], i32))
            })
            .or_insert(Human::new(
                parse_input!(inputs[1], i32),
                parse_input!(inputs[2], i32),
            ));
        alive_humans.push(parse_input!(inputs[0], i32).to_owned());
    }
    let alive_humans: HashSet<i32> = alive_humans.into_iter().collect();
    humans.retain(|key, _| alive_humans.contains(key));
}
