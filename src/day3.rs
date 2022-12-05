use core::panic;
use std::fs;

pub fn get_sum_of_priorities() -> u32 {
    return fs::read_to_string("input/3.txt")
        .expect("Failed to read file")
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let comp_size = line.len() / 2;
            line.chars()
                .enumerate()
                .find(|(i, _)| *i == comp_size)
                .map(|(i, _)| {
                    (
                        line.chars().take(i).collect::<Vec<char>>(),
                        line.chars().skip(i).collect::<Vec<char>>(),
                    )
                })
                .expect("Failed to split")
        })
        .map(|(first, second)| {
            get_priority(
                *first
                    .iter()
                    .find(|&c| second.iter().any(|d| d == c))
                    .expect(format!("failed: {0:?}, {1:?}", first, second).as_str()),
            )
        })
        .sum();
}

fn get_priority(item: char) -> u32 {
    match item {
        c if c.is_ascii_lowercase() => item as u32 - 'a' as u32 + 1,
        c if c.is_ascii_uppercase() => item as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}

pub fn get_sum_of_badge_priorities() -> u32 {
    return fs::read_to_string("input/3.txt")
        .expect("Failed to read file")
        .split('\n')
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .map(|(first, second, third)| {
            for c in first.chars() {
                if second.contains(c) && third.contains(c) {
                    return get_priority(c);
                }
            }
            panic!("No match!");
        })
        .sum();
}
