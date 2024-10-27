pub fn find_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt() as i32
}
