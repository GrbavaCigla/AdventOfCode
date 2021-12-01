use std::fs::read_to_string;
use std::option::Option;

fn two_point(values: Vec<i32>, target: i32) -> Option<i32> {
    let mut start = 0;
    let mut end = values.len() - 1;

    for i in values.iter() {
        while end > start {
            let cur_sum = *i + values[start] + values[end];

            if cur_sum == target {
                return Some(*i * values[start] * values[end]);
            } else if cur_sum < target {
                start += 1;
            } else {
                end -= 1;
            }
        }
    }
    None
}

fn main() {
    let input = read_to_string("./input").unwrap();

    let mut vec_input: Vec<_> = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    
    vec_input.sort();

    println!("{}", two_point(vec_input, 2020).unwrap());
}