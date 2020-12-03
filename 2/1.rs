use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();

    let mut result = 0;
    for line in input.lines() {
        let parsed_line = line.split(&[' ', ':', '-'][..]);
        let parsed_line: Vec<&str> = parsed_line.collect();

        let amount = parsed_line[4].matches(parsed_line[2]).count();
        
        let lower: usize = parsed_line[0].parse().unwrap();
        let upper: usize = parsed_line[1].parse().unwrap();

        if lower <= amount && amount <= upper {
            result += 1;
        }
    }
    println!("{}", result);
}
