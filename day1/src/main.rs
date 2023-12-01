use std::{fs, env, error::Error};



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Ok(());
    }
    let path = &args[1];
    let input = fs::read_to_string(path)?;



    let total = input.lines().fold(0, |total, line| {
        // Part1
        // let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

        // Part2
        // I'm so sorry this is very jank.
        let nums: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let mut digits: Vec<char> = vec![];
        for i in 0..line.len() {
            let slice = &line[i..];
            for num in &nums {
                let matched = slice.find(num);
                if matched.is_none() { continue; }
                if matched.unwrap() != 0 { continue; }
                digits.push(
                    match num.to_owned() {
                        "1" | "one" => '1',
                        "2" | "two" => '2',
                        "3" | "three" => '3',
                        "4" | "four" => '4',
                        "5" | "five" => '5',
                        "6" | "six" => '6',
                        "7" | "seven" => '7',
                        "8" | "eight" => '8',
                        "9" | "nine" => '9',
                        _ => '0'
                    }
                )
            }
        }

        let joined_digits = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());

        let num = joined_digits.parse::<i32>().unwrap();

        total + num
    });


    println!("Total: {}", total);

    

    Ok(())
}


