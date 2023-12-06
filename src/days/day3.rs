


pub fn day3(input: String, part: i32) -> i64 {
    let grid: Vec<char> = input.chars().filter(|char| !char.is_control()).collect();
    let width: i32 = input.find('\n').unwrap() as i32 - 1;
    let height: i32 = input.lines().into_iter().count().try_into().unwrap();
    let get = |x: i32, y: i32| -> char {
        if x < 0 || x >= width || y < 0 || y >= height {
            return '.';
        }
        grid[(x + y * width) as usize]
    };
    // num, numWidth, numX, numY
    let get_num = |mut x: i32, y: i32| -> Option<(i32, i32, i32, i32)> {
        if get(x, y).is_digit(10) {
            while get(x - 1, y).is_digit(10) {
                x -= 1;
            }
    
            let mut digits = String::new();
    
            for x in x..width {
                let char = get(x, y);
                if char.is_digit(10) {
                    digits.push(char);
                } else {
                    break;
                }
            }
    
            Some((digits.parse().unwrap(), digits.len().try_into().unwrap(), x, y))
        } else {
            None
        }
    };
    let get_surrounding_area = |x: i32, y: i32, width: i32| -> Vec<(i32, i32)> {
        let mut area: Vec<(i32, i32)> = vec![];
        for dx in -1..=width {
            area.push((x + dx, y - 1));
            area.push((x + dx, y + 1));
        }
        area.push((x - 1, y));
        area.push((x + width, y));
        area
    };

    let mut total: i64 = 0;

    if part == 1 {
        for y in 0..height {
            let mut x = 0;
            while x < width {
                match get_num(x, y) {
                    None => {
                        x += 1;
                    },
                    Some((num, width, _, _)) => {
                        if get_surrounding_area(x, y, width)
                            .iter().any(|check| {
                                let char = get(check.0, check.1);
                                if char.is_digit(10) { false }
                                else if char == '.' { false }
                                else { true }
                            })
                        {
                            total += i64::from(num);
                        }

                        x += width;
                    }
                }
    
            }
        }
    } else {
        let mut gears: Vec<(i32, i32)> = vec![];
        for x in 0..width {
            for y in 0..height {
                if get(x, y) == '*' {
                    gears.push((x, y));
                }
            }
        }

        gears.iter().for_each(|gear| {
            let mut nums: Vec<(i32, i32, i32, i32)> = vec![];
            get_surrounding_area(gear.0, gear.1, 1).iter().for_each(|check| {
                match get_num(check.0, check.1) {
                    None => {},
                    Some((num, width, x, y)) => {
                        if nums.iter().all(|(_, _, nx, ny)| { nx.to_owned() != x || ny.to_owned() != y }) {
                            nums.push((num, width, x, y));
                        }
                    }
                }
            });
            if nums.len() == 2 {
                total += i64::from(nums[0].0) * i64::from(nums[1].0);
            }
        });
    }

    total
}


