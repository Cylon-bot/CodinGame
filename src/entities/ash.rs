use std::io;

use crate::parse_input;
pub struct Ash {
    pub x: i32,
    pub y: i32,
}
impl Ash {
    pub fn new() -> Self {
        Ash { x: 0, y: 0 }
    }

    pub fn change_position(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        self.x = parse_input!(inputs[0], i32);
        self.y = parse_input!(inputs[1], i32);
    }
}
