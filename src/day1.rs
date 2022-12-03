use std::fs;

pub fn main() {
    let contents = fs::read_to_string("input/1.txt").expect("Failed to read file");
    let list: Vec<&str> = contents.split('\n').collect();
    println!("List: {:?}", list);
}
