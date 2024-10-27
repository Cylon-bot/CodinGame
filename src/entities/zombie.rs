use std::{collections::HashMap, io};

use crate::parse_input;

pub struct Zombie {
    pub x: i32,
    pub y: i32,
    pub next_x: i32,
    pub next_y: i32,
}
impl Zombie {
    pub fn new(x: i32, y: i32, next_x: i32, next_y: i32) -> Self {
        Zombie {
            x,
            y,
            next_x,
            next_y,
        }
    }
    fn change_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn change_next_position(&mut self, x: i32, y: i32) {
        self.next_x = x;
        self.next_y = y;
    }
}

pub fn get_zombies(zombies: &mut HashMap<i32, Zombie>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let zombie_count = parse_input!(input_line, i32);
    for _ in 0..zombie_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        zombies
            .entry(parse_input!(inputs[0], i32))
            .and_modify(|h: &mut Zombie| {
                h.change_position(parse_input!(inputs[1], i32), parse_input!(inputs[2], i32));
                h.change_next_position(parse_input!(inputs[3], i32), parse_input!(inputs[4], i32));
            })
            .or_insert(Zombie::new(
                parse_input!(inputs[1], i32),
                parse_input!(inputs[2], i32),
                parse_input!(inputs[3], i32),
                parse_input!(inputs[4], i32),
            ));
    }
}
