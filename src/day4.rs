use std::fs;

pub fn get_num_pairs_with_full_contain() -> i32 {
    return fs::read_to_string("input/4.txt")
        .expect("Failed to read file")
        .split('\n')
        .map(|s| {
            s.replace('-', ",")
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| {
            (v.get(0) <= v.get(2) && v.get(1) >= v.get(3)
                || v.get(0) >= v.get(2) && v.get(1) <= v.get(3)) as i32
        })
        .sum();
}

pub fn get_num_pairs_with_overlap() -> i32 {
    return fs::read_to_string("input/4.txt")
        .expect("Failed to read file")
        .split('\n')
        .map(|s| {
            s.replace('-', ",")
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| {
            (v.get(0) <= v.get(2) && v.get(1) >= v.get(2)
                || v.get(0) >= v.get(2) && v.get(0) <= v.get(3)) as i32
        })
        .sum();
}
