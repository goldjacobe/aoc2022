use std::fs;

pub fn get_sum_of_priorities() -> u32 {
    return fs::read_to_string("input/3.txt")
        .expect("Failed to read file")
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let comp_size = line.len() / 2;
            let (first, second) = line
                .chars()
                .enumerate()
                .find(|(i, _)| *i == comp_size)
                .map(|(i, _)| {
                    (
                        line.chars().take(i).collect::<Vec<char>>(),
                        line.chars().skip(i).collect::<Vec<char>>(),
                    )
                })
                .expect("Failed to split");
            let &item = first
                .iter()
                .find(|&c| second.iter().any(|d| d == c))
                .expect(format!("failed: {}", line).as_str());
            return get_priority(item);
        })
        .sum();
}

fn get_priority(item: char) -> u32 {
    match item {
        c if c.is_ascii_lowercase() => item as u32 - 'a' as u32 + 1,
        c if c.is_ascii_uppercase() => item as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}
