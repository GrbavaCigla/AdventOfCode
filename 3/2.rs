use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();

    let vec_lines: Vec<_> = input.lines().collect();

    let limit = vec_lines[0].len();
    
    let steps = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    let mut res: u32 = 1;

    for (x, y) in steps.iter() {
        let mut xcount = 0;
        let mut ans = 0;

        for line in vec_lines[*y as usize..].iter().step_by(*y) {
            xcount += x;
            let current = line.chars().nth(xcount % limit).unwrap();
    
            ans += if current == '#' {1} else {0};
        }
        res *= ans;
    }

    println!("{}", res);
}