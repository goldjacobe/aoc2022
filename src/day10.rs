use std::fs;

pub fn get_x_values() -> Vec<i32> {
    let mut x = 1;
    return [
        vec![x],
        fs::read_to_string("input/10.txt")
            .unwrap()
            .split('\n')
            .flat_map(|line| match line {
                "noop" => vec![0],
                line => vec![
                    0,
                    line.split(' ')
                        .skip(1)
                        .next()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                ],
            })
            .map(|delta| {
                x += delta;
                return x;
            })
            .collect::<Vec<i32>>(),
    ]
    .concat();
}

pub fn get_sum_strengths() -> i32 {
    let v = get_x_values();
    return v.get(19).unwrap() * 20
        + v.get(59).unwrap() * 60
        + v.get(99).unwrap() * 100
        + v.get(139).unwrap() * 140
        + v.get(179).unwrap() * 180
        + v.get(219).unwrap() * 220;
}

pub fn render_crt() -> String {
    return get_x_values()
        .iter()
        .enumerate()
        .flat_map(|(i, &x)| {
            let mut v = vec![if (i % 40).abs_diff(x as usize) <= 1 {
                '#'
            } else {
                '.'
            }];
            if i % 40 == 39 {
                v.push('\n')
            };
            return v;
        })
        .collect::<String>();
}
