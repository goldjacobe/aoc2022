use std::fs;

pub fn get_tops_of_stacks() -> String {
    let input = fs::read_to_string("input/5.txt").expect("Failed to read file");
    let lines = input.split('\n');

    let mut input_part1 = lines
        .take_while(|line| line.len() > 0)
        .collect::<Vec<&str>>();
    input_part1.reverse();
    let mut input_part1_iter = input_part1.iter();

    let num_stacks: usize = input_part1_iter
        .next()
        .unwrap()
        .chars()
        .rev()
        .skip(1)
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();

    let mut stacks = vec![Vec::<char>::new(); num_stacks];
    for line in input_part1_iter {
        for (index, char) in line.chars().enumerate() {
            if char.is_alphabetic() {
                stacks.get_mut(index / 4).map(|stack| stack.push(char));
            }
        }
    }

    let remaining = input.split('\n').skip_while(|line| line.len() > 0).skip(1);
    for line in remaining {
        let relevant = line
            .split(' ')
            .map(|s| s.parse::<usize>())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<usize>>();

        let &num_to_move = relevant.get(0).unwrap();
        let from = *relevant.get(1).unwrap() - 1;
        let to = *relevant.get(2).unwrap() - 1;

        for _ in 0..num_to_move {
            let char = stacks.get_mut(from).unwrap().pop().unwrap();

            stacks.get_mut(to).unwrap().push(char)
        }
    }

    return stacks
        .iter()
        .map(|stack| stack.iter().rev().next().unwrap())
        .collect::<String>();
}

pub fn get_tops_of_stacks_2() -> String {
    let input = fs::read_to_string("input/5.txt").expect("Failed to read file");
    let lines = input.split('\n');

    let mut input_part1 = lines
        .take_while(|line| line.len() > 0)
        .collect::<Vec<&str>>();
    input_part1.reverse();
    let mut input_part1_iter = input_part1.iter();

    let num_stacks: usize = input_part1_iter
        .next()
        .unwrap()
        .chars()
        .rev()
        .skip(1)
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();

    let mut stacks = vec![Vec::<char>::new(); num_stacks];
    for line in input_part1_iter {
        for (index, char) in line.chars().enumerate() {
            if char.is_alphabetic() {
                stacks.get_mut(index / 4).map(|stack| stack.push(char));
            }
        }
    }

    let remaining = input.split('\n').skip_while(|line| line.len() > 0).skip(1);
    for line in remaining {
        let relevant = line
            .split(' ')
            .map(|s| s.parse::<usize>())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<usize>>();

        let &num_to_move = relevant.get(0).unwrap();
        let from = *relevant.get(1).unwrap() - 1;
        let to = *relevant.get(2).unwrap() - 1;

        let mut temp = Vec::new();
        for _ in 0..num_to_move {
            let char = stacks.get_mut(from).unwrap().pop().unwrap();
            temp.push(char);
        }
        for _ in 0..num_to_move {
            let char = temp.pop().unwrap();
            stacks.get_mut(to).unwrap().push(char)
        }
    }

    return stacks
        .iter()
        .map(|stack| stack.iter().rev().next().unwrap())
        .collect::<String>();
}
