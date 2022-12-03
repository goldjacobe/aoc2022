use std::fs;
use std::vec::Vec;

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

pub fn get_top3_calories() -> i32 {
    let mut elves = fs::read_to_string("input/1.txt")
        .expect("Failed to read file")
        .split('\n')
        .collect::<Vec<&str>>()
        .split(|&line| line == "")
        .map(|l| {
            l.iter()
                .map(|&s| str::parse::<i32>(s).expect("Failed to parse string"))
                .sum()
        })
        .collect::<Vec<i32>>();
    elves.sort_by(|a, b| b.cmp(a));
    return elves.iter().take(3).sum();
}
