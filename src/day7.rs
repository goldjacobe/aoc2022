use std::collections::{HashMap, HashSet};
use std::fs;

pub fn get_sum_of_sizes() -> i32 {
    let input = fs::read_to_string("input/7.txt").expect("Failed to read file");
    let lines = input.split('\n');

    let mut cur_dir: String = "/".to_string();
    let mut dirs: HashSet<String> = HashSet::from(["/".to_string()]);
    let mut parents: HashMap<String, String> = HashMap::new();
    let mut cur_children: Option<Vec<String>> = None;
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    let mut sizes: HashMap<String, i32> = HashMap::new();

    for line in lines {
        let split_line = line.split(' ').collect::<Vec<&str>>();
        match *split_line.get(0).unwrap() {
            "$" => {
                if cur_children.is_some() {
                    children.insert(cur_dir.clone(), cur_children.unwrap());
                    cur_children = None;
                }
                match *split_line.get(1).unwrap() {
                    "cd" => {
                        cur_dir = match split_line.get(2).map(|s| *s) {
                            Some("..") => parents.get(&cur_dir).unwrap().clone(),
                            Some("/") => "/".to_string(),
                            Some(name)
                                if dirs.contains(format!("{}{}/", cur_dir, name).as_str()) =>
                            {
                                format!("{}{}/", cur_dir, &name)
                            }
                            _ => panic!(),
                        }
                    }
                    "ls" => {
                        cur_children = Some(Vec::new());
                    }
                    _ => panic!(),
                }
            }
            "dir" => {
                let name = split_line.get(1).unwrap().to_string();
                let full_name = format!("{}{}/", cur_dir, name);
                cur_children.as_mut().unwrap().push(full_name.clone());
                dirs.insert(full_name.clone());
                parents.insert(full_name, cur_dir.clone());
            }
            size if size.parse::<i32>().is_ok() => {
                let name = split_line.get(1).unwrap().to_string();
                let full_name = format!("{}{}", cur_dir, name);
                cur_children.as_mut().unwrap().push(full_name.clone());
                sizes.insert(full_name, size.parse::<i32>().unwrap());
            }
            _ => panic!(),
        }
    }

    if cur_children.is_some() {
        children.insert(cur_dir, cur_children.unwrap());
    }

    let mut queue = dirs.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let mut result = 0;
    while let Some(dir) = queue.pop() {
        children
            .get(dir)
            .unwrap()
            .iter()
            .fold(Some(0), |accum, item| {
                sizes
                    .get(item)
                    .map(|size| accum.map(|value| value + size))
                    .flatten()
            })
            .map_or_else(
                || queue.insert(0, dir),
                |value| {
                    sizes.insert(dir.to_string(), value);
                    if value <= 100000 {
                        result += value;
                    }
                },
            );
    }
    return result;
}

pub fn get_smallest_to_delete() -> i32 {
    let input = fs::read_to_string("input/7.txt").expect("Failed to read file");
    let lines = input.split('\n');

    let mut cur_dir: String = "/".to_string();
    let mut dirs: HashSet<String> = HashSet::from(["/".to_string()]);
    let mut parents: HashMap<String, String> = HashMap::new();
    let mut cur_children: Option<Vec<String>> = None;
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    let mut sizes: HashMap<String, i32> = HashMap::new();

    for line in lines {
        let split_line = line.split(' ').collect::<Vec<&str>>();
        match *split_line.get(0).unwrap() {
            "$" => {
                if cur_children.is_some() {
                    children.insert(cur_dir.clone(), cur_children.unwrap());
                    cur_children = None;
                }
                match *split_line.get(1).unwrap() {
                    "cd" => {
                        cur_dir = match split_line.get(2).map(|s| *s) {
                            Some("..") => parents.get(&cur_dir).unwrap().clone(),
                            Some("/") => "/".to_string(),
                            Some(name)
                                if dirs.contains(format!("{}{}/", cur_dir, name).as_str()) =>
                            {
                                format!("{}{}/", cur_dir, &name)
                            }
                            _ => panic!(),
                        }
                    }
                    "ls" => {
                        cur_children = Some(Vec::new());
                    }
                    _ => panic!(),
                }
            }
            "dir" => {
                let name = split_line.get(1).unwrap().to_string();
                let full_name = format!("{}{}/", cur_dir, name);
                cur_children.as_mut().unwrap().push(full_name.clone());
                dirs.insert(full_name.clone());
                parents.insert(full_name, cur_dir.clone());
            }
            size if size.parse::<i32>().is_ok() => {
                let name = split_line.get(1).unwrap().to_string();
                let full_name = format!("{}{}", cur_dir, name);
                cur_children.as_mut().unwrap().push(full_name.clone());
                sizes.insert(full_name, size.parse::<i32>().unwrap());
            }
            _ => panic!(),
        }
    }

    if cur_children.is_some() {
        children.insert(cur_dir, cur_children.unwrap());
    }

    let mut queue = dirs.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    while let Some(dir) = queue.pop() {
        children
            .get(dir)
            .unwrap()
            .iter()
            .fold(Some(0), |accum, item| {
                sizes
                    .get(item)
                    .map(|size| accum.map(|value| value + size))
                    .flatten()
            })
            .map_or_else(
                || queue.insert(0, dir),
                |value| {
                    sizes.insert(dir.to_string(), value);
                },
            );
    }

    let available = 70000000 - sizes.get("/").unwrap();
    let mut options = sizes.values().into_iter().map(|&i| i).collect::<Vec<i32>>();
    options.sort();
    for option in options {
        if available + option >= 30000000 {
            return option;
        }
    }
    return 0;
}
