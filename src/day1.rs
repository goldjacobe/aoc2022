use std::fs;

pub fn get_max_calories() -> i32 {
    return fs::read_to_string("input/1.txt")
        .expect("Failed to read file")
        .split('\n')
        .collect::<Vec<&str>>()
        .split(|&line| line == "")
        .map(|l| {
            l.iter()
                .map(|&s| str::parse::<i32>(s).expect("Failed to parse string"))
                .sum()
        })
        .max()
        .unwrap_or_default();
}
