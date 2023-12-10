use std::fs;

pub mod day_2;



fn main() {
    let input = fs::read_to_string("input/day_2.txt").unwrap();
    let input: Vec<&str> = input.split("\n").collect();
    // println!("{:?}", day_2::part_1(input, (12, 13, 14)));
    println!("{:?}", day_2::part_2(input));
}


