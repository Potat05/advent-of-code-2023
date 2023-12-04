


pub fn day3(input: String, part: i32) -> i32 {
    let grid: Vec<char> = input.chars().filter(|char| !char.is_control()).collect();
    let width: i32 = input.find('\n').unwrap() as i32 - 1;
    let height: i32 = input.lines().into_iter().count().try_into().unwrap();
    let get = |x: i32, y: i32| -> char {
        if x < 0 || x >= width || y < 0 || y >= height {
            return '.';
        }
        grid[(x + y * width) as usize]
    };

    let mut total = 0;

    for y in 0..height {
        let mut x = -1;
        while x < width {
            x += 1;

            if !(get(x, y).is_digit(10)) { continue; }


            // Get part number.
            let mut digits: String = String::new();
            // Get full number
            for dx in x..width {
                if get(dx, y).is_digit(10) {
                    digits += &get(dx, y).to_string();
                } else {
                    break;
                }
            }


            // If part is next to symbol
            let mut area: Vec<(i32, i32)> = vec![];
            
            for dx in -1..(digits.len() as i32 + 1) {
                area.push((x + dx, y - 1));
                area.push((x + dx, y + 1));
            }
            area.push((x - 1, y));
            area.push((x + digits.len() as i32, y));

            if area.iter().any(|check| {
                let char = get(check.0, check.1);
                if char.is_digit(10) { false }
                else if char == '.' { false }
                else { true }
            }) {
                total += digits.parse::<i32>().unwrap();
            }


            x += (digits.len() as i32) - 1;

        }
    }

    total
}


