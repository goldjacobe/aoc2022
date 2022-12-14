use std::fs;

pub fn get_total_score() -> i32 {
    return fs::read_to_string("input/2.txt")
        .expect("Failed to read file")
        .split('\n')
        .filter(|line| line.len() > 2)
        .map(|line| {
            (
                map_left_char(line.chars().nth(0).expect("should have a char")),
                map_right_char(line.chars().nth(2).expect("should have a char")),
            )
        })
        .map(|(abc, xyz)| (xyz - abc + 4) % 3 * 3 + xyz + 1)
        .sum();
}

fn map_left_char(c: char) -> i32 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => panic!("oopsie, [{}]", c),
    }
}

fn map_right_char(c: char) -> i32 {
    match c {
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!("oopsie, [{}]", c),
    }
}

pub fn get_total_score_2() -> i32 {
    return fs::read_to_string("input/2.txt")
        .expect("Failed to read file")
        .split('\n')
        .filter(|line| line.len() > 2)
        .map(|line| {
            (
                map_left_char(line.chars().nth(0).expect("should have a char")),
                map_right_char(line.chars().nth(2).expect("should have a char")),
            )
        })
        .map(|(abc, xyz)| (xyz + 2 + abc) % 3 + 1 + xyz * 3)
        .sum();
}
