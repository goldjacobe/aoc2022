use std::fs;

pub fn get_num_pairs_with_full_contain() -> usize {
    return fs::read_to_string("input/4.txt")
        .expect("Failed to read file")
        .split('\n')
        // .filter(|&line| line.len() > 0)
        .map(|s| {
            let v = s
                .replace('-', ",")
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if v.get(0) <= v.get(2) && v.get(1) >= v.get(3)
                || v.get(0) >= v.get(2) && v.get(1) <= v.get(3)
            {
                return 1;
            } else {
                return 0;
            };
        })
        .sum();
}

pub fn get_num_pairs_with_overlap() -> usize {
    return fs::read_to_string("input/4.txt")
        .expect("Failed to read file")
        .split('\n')
        // .filter(|&line| line.len() > 0)
        .map(|s| {
            let v = s
                .replace('-', ",")
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if v.get(0) <= v.get(2) && v.get(1) >= v.get(2)
                || v.get(0) >= v.get(2) && v.get(0) <= v.get(3)
            {
                return 1;
            } else {
                return 0;
            };
        })
        .sum();
}
