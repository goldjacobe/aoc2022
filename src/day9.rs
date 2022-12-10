use std::{collections::HashSet, fs};

pub fn get_num_positions() -> usize {
    let input = fs::read_to_string("input/9.txt").expect("Failed to read file");
    let lines = input.split('\n');

    let (mut head_x, mut head_y, mut tail_x, mut tail_y) = (0, 0, 0, 0);
    let positions = lines
        .flat_map(|line| {
            let split_line: Vec<&str> = line.split(" ").collect();
            let direction = match *split_line.get(0).unwrap() {
                "U" => (0, -1),
                "D" => (0, 1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => panic!(),
            };
            let steps = split_line.get(1).unwrap().parse::<i32>().unwrap();
            let mut positions = Vec::<(i32, i32)>::new();
            for _ in 0..steps {
                head_x += direction.0;
                head_y += direction.1;
                let (dx, dy) = (head_x - tail_x, head_y - tail_y);
                (tail_x, tail_y) = (
                    match dx {
                        0 => tail_x,
                        -1 | 1 => {
                            if dy > 1 || dy < -1 {
                                tail_x + dx
                            } else {
                                tail_x
                            }
                        }
                        dx if dx > 1 => tail_x + 1,
                        dx if dx < -1 => tail_x - 1,
                        _ => panic!(),
                    },
                    match dy {
                        0 => tail_y,
                        -1 | 1 => {
                            if dx > 1 || dx < -1 {
                                tail_y + dy
                            } else {
                                tail_y
                            }
                        }
                        dy if dy > 1 => tail_y + 1,
                        dy if dy < -1 => tail_y - 1,
                        _ => panic!(),
                    },
                );
                positions.push((tail_x, tail_y))
            }
            return positions;
        })
        .collect::<HashSet<(i32, i32)>>();

    return positions.len();
}

pub fn get_num_positions_2() -> usize {
    let input = fs::read_to_string("input/9.txt").expect("Failed to read file");
    let lines = input.split('\n');

    let mut knots = [(0, 0); 10];

    let positions = lines
        .flat_map(|line| {
            let split_line: Vec<&str> = line.split(" ").collect();
            let direction = match *split_line.get(0).unwrap() {
                "U" => (0, -1),
                "D" => (0, 1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => panic!(),
            };
            let steps = split_line.get(1).unwrap().parse::<i32>().unwrap();
            let mut positions = Vec::<(i32, i32)>::new();
            for _ in 0..steps {
                let head = knots.get_mut(0).unwrap();
                head.0 += direction.0;
                head.1 += direction.1;
                for head in 0..9 {
                    let ((head_x, head_y), (tail_x, tail_y)) = (
                        knots.get(head).unwrap().clone(),
                        knots.get_mut(head + 1).unwrap(),
                    );
                    let (dx, dy) = (head_x - *tail_x, head_y - *tail_y);
                    (*tail_x, *tail_y) = (
                        match dx {
                            0 => *tail_x,
                            -1 | 1 => {
                                if dy > 1 || dy < -1 {
                                    *tail_x + dx
                                } else {
                                    *tail_x
                                }
                            }
                            dx if dx > 1 => *tail_x + 1,
                            dx if dx < -1 => *tail_x - 1,
                            _ => panic!(),
                        },
                        match dy {
                            0 => *tail_y,
                            -1 | 1 => {
                                if dx > 1 || dx < -1 {
                                    *tail_y + dy
                                } else {
                                    *tail_y
                                }
                            }
                            dy if dy > 1 => *tail_y + 1,
                            dy if dy < -1 => *tail_y - 1,
                            _ => panic!(),
                        },
                    );
                }
                let tail = knots.get(9).unwrap();
                positions.push(tail.clone())
            }
            return positions;
        })
        .collect::<HashSet<(i32, i32)>>();

    return positions.len();
}
