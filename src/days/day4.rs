use regex::Regex;



pub fn day4(input: String, part: i32) -> i64 {
    // I'm sorry this is horribly structured but I'm too lazy to change it.
    let cards: Vec<&str> = input.lines().collect();
    let wins: Vec<i32> = cards.iter().map(|card| {
        let re = Regex::new(r"Card\s+(\d+):((?:\s+\d+)+)\s+\|((?:\s+\d+)+)").unwrap();
        let captures = re.captures(card).unwrap();
        // let card_num = &captures[1].parse::<i32>().unwrap();
        let winners = &captures[2].split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let draws = &captures[3].split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        draws.iter().fold(0, |wins, draw| {
            if winners.contains(draw) {
                wins + 1
            } else {
                wins
            }
        })
    }).collect();

    if part == 1 {
        wins.iter().fold(0, |total, wins| {
            let mut worth = 0;
            for i in 0..*wins {
                if i == 0 {
                    worth = 1;
                } else {
                    worth *= 2;
                }
            }
            total + worth
        })
    } else {
        let mut counts: Vec<i32> = cards.iter().map(|_| 1).collect();
        let mut total = 0;

        for i in 0..(wins.len()) {
            for _ in 0..counts[i] {
                for j in 1..=(wins[i] as usize) {
                    if (i + j) > counts.len() {
                        break;
                    }
                    counts[i + j] += 1;
                }

                total += 1;
            }
        }

        total
    }
}


