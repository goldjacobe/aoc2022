use std::{collections::HashSet, fs};

pub fn get_first_pos() -> usize {
    let binding = fs::read_to_string("input/6.txt").expect("Failed to read file");
    let mut input = binding.chars();

    let mut three = input.next().unwrap();
    let mut two = input.next().unwrap();
    let mut one = input.next().unwrap();
    let mut zero: char = input.next().unwrap();
    let mut result: usize = 4;
    while three == two || three == one || three == zero || two == one || two == zero || one == zero
    {
        three = two;
        two = one;
        one = zero;
        zero = input.next().unwrap();
        result = result + 1;
    }
    return result;
}

pub fn get_first_pos_2() -> usize {
    let input = fs::read_to_string("input/6.txt").expect("Failed to read file");

    let mut result: usize = 14;

    while HashSet::<char>::from_iter(input.get(result - 14..result).unwrap().chars()).len() < 14 {
        result = result + 1;
    }
    return result;
}
