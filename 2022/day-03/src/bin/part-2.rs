use std::fs;
use day_03::process_part2;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("part2: {}", process_part2(&file));
}

