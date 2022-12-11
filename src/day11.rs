use std::{fs, num::ParseIntError};

struct Monkey {
    starting_items: Vec<i32>,
    operation_sign: char,
    operation_val: Result<i32, ParseIntError>,
    test_divisor: i32,
    true_monkey: usize,
    false_monkey: usize,
}

pub fn get_monkey_business() -> i32 {
    let monkeys = parse_monkeys();

    let mut num_inspected = vec![0; monkeys.len()];
    let mut held_items: Vec<Vec<i32>> = monkeys
        .iter()
        .map(|monkey| monkey.starting_items.clone())
        .collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get(i).unwrap();
            for new_worry in std::mem::replace(&mut held_items[i], Vec::new())
                .iter()
                .map(|item| {
                    (match monkey.operation_sign {
                        '*' => |x, y| x * y,
                        '+' => |x, y| x + y,
                        _ => panic!(),
                    })(
                        item,
                        match monkey.operation_val {
                            Ok(val) => val,
                            Err(_) => *item,
                        },
                    ) / 3
                })
            {
                num_inspected[i] += 1;
                let throw_to = if new_worry % monkey.test_divisor == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                let mut new_held_items = held_items.get(throw_to).unwrap().clone();
                new_held_items.push(new_worry);
                held_items[throw_to] = new_held_items;
            }
        }
    }

    num_inspected.sort_by(|a, b| b.cmp(a));

    return num_inspected[0] * num_inspected[1];
}

pub fn get_monkey_business_2() -> i64 {
    let monkeys = parse_monkeys();

    let mut num_inspected = vec![0; monkeys.len()];
    let mut held_items: Vec<Vec<i32>> = monkeys
        .iter()
        .map(|monkey| monkey.starting_items.clone())
        .collect();

    let modulus: i32 = monkeys.iter().map(|monkey| monkey.test_divisor).product();
    println!("{}", modulus);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get(i).unwrap();
            for new_worry in std::mem::replace(&mut held_items[i], Vec::new())
                .iter()
                .map(|&item| {
                    i32::try_from(
                        (match monkey.operation_sign {
                            '*' => |x, y| x * y,
                            '+' => |x, y| x + y,
                            _ => panic!(),
                        })(
                            i64::from(item),
                            i64::from(match monkey.operation_val {
                                Ok(val) => val,
                                Err(_) => item,
                            }),
                        ) % i64::from(modulus),
                    )
                    .unwrap()
                })
            {
                num_inspected[i] += 1;
                let throw_to = if new_worry % monkey.test_divisor == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                let mut new_held_items = held_items.get(throw_to).unwrap().clone();
                new_held_items.push(new_worry);
                held_items[throw_to] = new_held_items;
            }
        }
    }

    num_inspected.sort_by(|a, b| b.cmp(a));

    return i64::from(num_inspected[0]) * i64::from(num_inspected[1]);
}

fn parse_monkeys() -> Vec<Monkey> {
    return fs::read_to_string("input/11.txt")
        .unwrap()
        .split("Monkey")
        .map(|s| s.to_string())
        .filter(|s| s.len() > 0)
        .map(|s| parse_monkey(s))
        .collect();
}

fn parse_monkey(s: String) -> Monkey {
    let lines = s
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let starting_items = lines
        .get(1)
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(", ")
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut operation_line = lines.get(2).unwrap().split(' ').skip(6);
    let operation_sign = operation_line.next().unwrap().chars().next().unwrap();
    let operation_val = operation_line.next().unwrap().parse::<i32>();

    let test_divisor = lines
        .get(3)
        .unwrap()
        .split(' ')
        .skip(5)
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let true_monkey = lines
        .get(4)
        .unwrap()
        .split(' ')
        .skip(9)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let false_monkey = lines
        .get(5)
        .unwrap()
        .split(' ')
        .skip(9)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    return Monkey {
        starting_items,
        operation_sign,
        operation_val,
        test_divisor,
        true_monkey,
        false_monkey,
    };
}
