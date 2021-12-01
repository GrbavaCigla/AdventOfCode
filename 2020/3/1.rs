use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();

    let vec_lines: Vec<_> = input.lines().collect();

    let limit = vec_lines[0].len();
    let mut xcount = 0;
    let mut ans = 0;

    for line in vec_lines[1..].iter() {
        xcount += 3;
        let current = line.chars().nth(xcount % limit).unwrap();

        ans += if current == '#' {1} else {0};
    }

    println!("{}", ans);
}