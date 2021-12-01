use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();

    let mut result = 0;
    for line in input.lines() {
        let parsed_line = line.split(&[' ', ':', '-'][..]);
        let parsed_line: Vec<&str> = parsed_line.collect();

        let lower = parsed_line[0].parse::<usize>().unwrap() - 1;
        let upper = parsed_line[1].parse::<usize>().unwrap() - 1;

        let letter: char = parsed_line[2].parse().unwrap();
        let password = parsed_line[4];

        let first_char = password.chars().nth(lower).unwrap();
        let second_char = password.chars().nth(upper).unwrap();

        if (first_char == letter) != (second_char == letter) {
            result += 1;
        }
    }
    println!("{}", result);
}
