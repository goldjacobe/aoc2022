use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn get_shortest_path_length() -> i32 {
    let map = fs::read_to_string("input/12.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, &char)| if char == 'S' { Some((i, j)) } else { None })
        })
        .unwrap();
    let end = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, &char)| if char == 'E' { Some((i, j)) } else { None })
        })
        .unwrap();
    let heights = map
        .iter()
        .map(|line| {
            line.iter()
                .map(|&c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    c => c,
                })
                .collect()
        })
        .collect::<Vec<Vec<char>>>();

    let mut memo = HashMap::new();
    get_shortest_path_to(&heights, start, end, HashSet::new(), &mut memo).unwrap()
}

fn get_shortest_path_to(
    heights: &Vec<Vec<char>>,
    from: (usize, usize),
    to: (usize, usize),
    seen: HashSet<(usize, usize)>,
    memo: &mut HashMap<(usize, usize), i32>,
) -> Option<i32> {
    if from.0 == to.0 && from.1 == to.1 {
        return Some(0);
    } else if memo.contains_key(&from) {
        return Some(*memo.get(&from).unwrap());
    }

    let &from_height = heights.get(from.0).unwrap().get(from.1).unwrap();
    let mut options = vec![
        if from.1 > 0 {
            Some((from.0, from.1 - 1))
        } else {
            None
        },
        if from.0 > 0 {
            Some((from.0 - 1, from.1))
        } else {
            None
        },
        if from.1 < heights.get(from.0).unwrap().len() - 1 {
            Some((from.0, from.1 + 1))
        } else {
            None
        },
        if from.0 < heights.len() - 1 {
            Some((from.0 + 1, from.1))
        } else {
            None
        },
    ]
    .iter()
    .filter_map(|&x| x)
    .filter(|option| !seen.contains(option))
    .map(|option| {
        (
            option,
            *heights.get(option.0).unwrap().get(option.1).unwrap(),
        )
    })
    .filter_map(|(option, height)| {
        if (0..2).contains(&(height as i8 - from_height as i8)) {
            Some((option, height))
        } else {
            None
        }
    })
    .collect::<Vec<((usize, usize), char)>>();

    let mut new_seen = seen.clone();
    new_seen.insert(from);
    println!("{:?}", options);
    options.sort_by(|a, b| b.1.cmp(&a.1));

    options
        .iter()
        .filter_map(|&option| {
            get_shortest_path_to(heights, option.0, to, new_seen.to_owned(), memo).map(|shortest| {
                memo.insert(option.0, shortest);
                shortest
            })
        })
        .min()
        .map(|min| min + 1)
}
