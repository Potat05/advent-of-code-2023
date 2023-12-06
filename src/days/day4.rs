use regex::Regex;



pub fn day4(input: String, part: i32) -> i64 {
    input.lines().fold(0, |total, line| {
        let re = Regex::new(r"Card\s+(\d+):((?:\s+\d+)+)\s+\|((?:\s+\d+)+)").unwrap();

        let captures = re.captures(line).unwrap();
        let card = &captures[1].parse::<i32>().unwrap();
        let winners = &captures[2].split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let draws = &captures[3].split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let wins = draws.iter().fold(0, |wins, draw| {
            if winners.contains(draw) {
                wins + 1
            } else {
                wins
            }
        });

        let mut worth = 0;
        for i in 0..wins {
            if i == 0 {
                worth = 1;
            } else {
                worth *= 2;
            }
        }

        total + worth
    })
}


