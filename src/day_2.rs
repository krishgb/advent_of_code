type RGB = (u32, u32, u32);

fn transform(game: &str) -> Vec<RGB> {
    let mut rgbs = vec![];

    let prefix_removed: Vec<&str> = game.split(": ").collect();
    let splitted_sets: Vec<&str> = prefix_removed[1].split("; ").collect();
    for set in splitted_sets {
        let balls: Vec<&str> = set.split(", ").collect();

        let mut rgb = (0, 0, 0);
        for ball in balls {
            let no_and_color: Vec<&str> = ball.split(" ").collect();

            let (no, color) = (no_and_color[0].parse::<u32>().unwrap(), no_and_color[1]);
            match color {
                "red" => rgb.0 = no,
                "green" => rgb.1 = no,
                "blue" => rgb.2 = no,
                _ => (),
            }
        }
        rgbs.push(rgb);
    }
    rgbs
}

pub fn part_1(games: Vec<&str>, (c_r, c_g, c_b): RGB) -> u32 {
    let mut res = 0;

    for i in 0..games.len() {
        let colors = transform(games[i]);
        let mut flag = true;
        for (r, g, b) in colors {
            if !(r <= c_r && g <= c_g && b <= c_b) {
                flag = false;
                break;
            }
        }
        if flag {
            res += (i + 1) as u32;
        }
    }
    res
}

pub fn part_2(games: Vec<&str>) -> u32 {
    let mut res = 0;

    for i in 0..games.len() {
        let colors = transform(games[i]);
        let mut rgb = (0,0,0);
        for (r, g, b) in colors {
            if r > rgb.0 { rgb.0 = r }
            if g > rgb.1 { rgb.1 = g }
            if b > rgb.2 { rgb.2 = b }
        }
        res +=  rgb.0 * rgb.1 * rgb.2;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_day_2() {
        assert_eq!(
            transform("Game 5: 8 green, 6 red, 16 blue; 8 red, 12 green; 1 red, 9 green, 16 blue; 8 red, 3 green; 2 blue, 5 red, 10 green; 15 red, 4 blue, 8 green"),
            vec![(6,8,16), (8,12,0), (1,9,16), (8,3,0), (5,10,2), (15,8,4)]

        );
        assert_eq!(
            transform("Game 100: 9 green, 2 blue, 12 red; 2 blue, 14 red, 2 green; 14 red, 12 green"),
            vec![(12, 9, 2), (14, 2, 2), (14, 12, 0)]
        );

        let words = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        assert_eq!(part_1(words, (12, 13, 14)), 8);


        let words = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        assert_eq!(part_2(words), 2286);
    }
}
