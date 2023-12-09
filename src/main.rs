use std::fs;

pub mod day_1;

fn main() {
    let input = fs::read_to_string("input/day_1.txt").unwrap();
    let input: Vec<&str> = input.split("\n").collect();

    // let result_part_1 = day_1::part_1(0input);
    // println!("{:?}", result_part_1);

    let result_part_2 = day_1::part_2(input);
    println!("{:?}", result_part_2);
}

#[cfg(test)]
mod tests {
    use super::day_1;

    #[test]
    pub fn test_day_1(){
        let words = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(day_1::part_1(words), 142);

        let words = vec!["two1nine","eightwothree","abcone2threexyz","xtwone3four","4nineeightseven2","zoneight234","7pqrstsixteen"];
        assert_eq!(day_1::part_2(words), 281);                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              
    }
}
