use std::{collections::HashSet, fs};

pub fn get_num_visible() -> usize {
    let input = fs::read_to_string("input/8.txt").expect("Failed to read file");
    let lines: Vec<&str> = input.split('\n').collect();
    let digits: Vec<Vec<_>> = lines
        .iter()
        .map(|&s| s.split("").filter(|&t| t.len() > 0).collect::<Vec<&str>>())
        .filter(|v| v.len() > 0)
        .collect();
    let map: Vec<Vec<u32>> = digits
        .iter()
        .map(|v| v.iter().map(|s| s.parse().unwrap()).collect())
        .collect();

    let width = map.get(0).unwrap().len();
    let height = map.len();

    let mut visible: HashSet<String> = HashSet::new();
    for i in 1..(width - 1) {
        let mut max_from_top = map.get(0).unwrap().get(i).unwrap();
        let mut max_from_bot = map.get(height - 1).unwrap().get(i).unwrap();
        for j in 1..(height - 1) {
            let from_top_tree = map.get(j).unwrap().get(i).unwrap();
            if from_top_tree > max_from_top {
                max_from_top = from_top_tree;

                visible.insert(format!("{}-{}", i, j));
            }
            let from_bot_tree = map.get(height - 1 - j).unwrap().get(i).unwrap();
            if from_bot_tree > max_from_bot {
                max_from_bot = from_bot_tree;

                visible.insert(format!("{}-{}", i, height - 1 - j));
            }
        }
    }
    for i in 1..(height - 1) {
        let mut max_from_left = map.get(i).unwrap().get(0).unwrap();
        let mut max_from_right = map.get(i).unwrap().get(width - 1).unwrap();
        for j in 1..(width - 1) {
            let from_left_tree = map.get(i).unwrap().get(j).unwrap();
            if from_left_tree > max_from_left {
                max_from_left = from_left_tree;

                visible.insert(format!("{}-{}", j, i));
            }
            let from_right_tree = map.get(i).unwrap().get(width - 1 - j).unwrap();
            if from_right_tree > max_from_right {
                max_from_right = from_right_tree;

                visible.insert(format!("{}-{}", width - 1 - j, i));
            }
        }
    }

    return visible.len() + 2 * width + 2 * height - 4;
}

pub fn get_max_score() -> i32 {
    let input = fs::read_to_string("input/8.txt").expect("Failed to read file");
    let lines: Vec<&str> = input.split('\n').collect();
    let digits: Vec<Vec<_>> = lines
        .iter()
        .map(|&s| s.split("").filter(|&t| t.len() > 0).collect::<Vec<&str>>())
        .filter(|v| v.len() > 0)
        .collect();
    let map: Vec<Vec<u32>> = digits
        .iter()
        .map(|v| v.iter().map(|s| s.parse().unwrap()).collect())
        .collect();

    let width = map.get(0).unwrap().len();
    let height = map.len();

    let mut max_score = 0;
    for i in 0..height {
        for j in 0..width {
            let cur_tree_height = map.get(i).unwrap().get(j).unwrap();

            let mut up_score = 0;
            for up in 1..(i + 1) {
                up_score += 1;
                let up_tree_height = map.get(i - up).unwrap().get(j).unwrap();
                if cur_tree_height <= up_tree_height {
                    break;
                }
            }

            let mut down_score = 0;
            for down in 1..(height - i) {
                down_score += 1;
                let down_tree_height = map.get(i + down).unwrap().get(j).unwrap();
                if cur_tree_height <= down_tree_height {
                    break;
                }
            }

            let mut left_score = 0;
            for left in 1..(j + 1) {
                left_score += 1;
                let left_tree_height = map.get(i).unwrap().get(j - left).unwrap();
                if cur_tree_height <= left_tree_height {
                    break;
                }
            }

            let mut right_score = 0;
            for right in 1..(width - j) {
                right_score += 1;
                let right_tree_height = map.get(i).unwrap().get(j + right).unwrap();
                if cur_tree_height <= right_tree_height {
                    break;
                }
            }

            let total_score = up_score * down_score * left_score * right_score;
            if total_score > max_score {
                max_score = total_score
            }
        }
    }

    return max_score;
}
