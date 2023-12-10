pub fn part_1(words: Vec<&str>) -> u32 {
    let mut res = 0;

    for word in words {
        let mut first = None;
        let mut last = None;
        for i in word.chars() {
            if i.is_digit(10) {
                if first.is_none() {
                    first = i.to_digit(10);
                }
                last = i.to_digit(10);
            }
        }

        if let Some(first) = first {
            res += first * 10 + last.unwrap();
        } 
    }

    res
}

pub fn part_2(words: Vec<&str>) -> u32 {
    let mut res = 0;

    for word in words {
        let mut first = None;
        let mut last = None;

        let mut last_aplha = 0;
        for (index, char) in word.char_indices() {
            if char.is_digit(10) {
                if first.is_none() {
                    first = char.to_digit(10);
                }
                last = char.to_digit(10);
            }

            if index < 1 { continue; }

            let mut alpha_numeric = None;
            let nums_in_words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
            
            for i in 0..nums_in_words.len() {
                let word = &word[last_aplha..=index];

                if word.contains(nums_in_words[i]) {
                    alpha_numeric = Some(i + 1);
                    last_aplha = index;
                }
            }

            if let Some(alpha_numeric) = alpha_numeric{
                if first.is_none() {
                    first = Some(alpha_numeric as u32);
                }
                last = Some(alpha_numeric as u32);
            }
        }

        res += first.unwrap() * 10 + last.unwrap();
    }


    res
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_day_1(){
        let words = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(part_1(words), 142);

        let words = vec!["two1nine","eightwothree","abcone2threexyz","xtwone3four","4nineeightseven2","zoneight234","7pqrstsixteen"];
        assert_eq!(part_2(words), 281);                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              
    }

}
