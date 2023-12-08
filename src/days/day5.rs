


pub fn day5(input: String, part: i32) -> i64 {
    let fixed = input.replace("\r\n", "\n");
    let sections: Vec<&str> = fixed.split("\n\n").collect();

    let mut values: Vec<i64> = sections[0].split(" ").collect::<Vec<&str>>()[1..].iter().map(|s| s.parse().unwrap()).collect();
    // (dest, source, len)
    let mappers: Vec<Vec<(i64, i64, i64)>> = sections[1..].iter().map(|section| {
        section.lines().collect::<Vec<&str>>()[1..].iter().map(|mapper| {
            let values: Vec<i64> = mapper.split(' ').map(|s| s.parse().unwrap()).collect();
            ( values[0], values[1], values[2] )
        }).collect()
    }).collect();

    for i in 0..values.len() {
        for mapper_group in mappers.iter() {
            for mapper in mapper_group {
                if (values[i] >= mapper.1) && (values[i] < mapper.1 + mapper.2) {
                    values[i] -= mapper.1 - mapper.0;
                    break;
                }
            }
        }
    }

    println!("{}", values.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));

    values.iter().fold(i64::MAX, |min, value| if value < &min { *value } else { min })
}


