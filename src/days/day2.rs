


pub fn day2(input: String, part: i32) -> i32 {
    let colors: Vec<&str> = vec!("red", "green", "blue");
    let max: Vec<i32> = vec!(12, 13, 14);

    let total = input.lines().fold(0, |total, line| {
        let sections: Vec<&str> = line.split(':').collect();
        
        // God I wish I had Regex.
        let header = sections[0];
        let id = header.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        let body = sections[1];
        let highest = body.split(';').fold(vec!(0, 0, 0), |highest, round| {
            let mut cubes = vec!(0, 0, 0);

            round.split(',').for_each(|cube| {
                let count = cube.trim().split(' ').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                let color = cube.trim().split(' ').collect::<Vec<&str>>()[1];
                let color_index: i32 = colors.iter().enumerate().fold(0, |color_index, (index, current)| {
                    if current.eq_ignore_ascii_case(color) { index.try_into().unwrap() } else { color_index }
                });
                cubes[color_index as usize] = count;
            });

            return vec!(
                if cubes[0] > highest[0] { cubes[0] } else { highest[0] },
                if cubes[1] > highest[1] { cubes[1] } else { highest[1] },
                if cubes[2] > highest[2] { cubes[2] } else { highest[2] }
            );
        });

        if part == 1 {
            if highest.iter().enumerate().any(|(i, _)| highest[i] > max[i]) {
                total
            } else {
                total + id
            }
        } else {
            total + highest[0] * highest[1] * highest[2]
        }

    });

    total
}


