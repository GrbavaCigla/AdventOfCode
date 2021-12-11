use std::collections::VecDeque;

const OPEN_BRACKETS: [char; 4] = ['(', '[', '{', '<'];
const CLOSE_BRACKETS: [char; 4] = [')', ']', '}', '>'];
const POINTS: [u64; 4] = [3, 57, 1197, 25137];

fn solve(x: &str, part1: bool) -> u64 {
    let mut stack = VecDeque::new();

    for i in x.chars() {
        if OPEN_BRACKETS.contains(&i) {
            stack.push_back(i);
        } else if CLOSE_BRACKETS.contains(&i) {
            let index = CLOSE_BRACKETS.iter().position(|&x| x == i).unwrap();

            if stack.pop_back().unwrap() != OPEN_BRACKETS[index] {
                if part1 {
                    return POINTS[index];
                } else {
                    return 0;
                }
            }
        }
    }

    if part1 {
        return 0;
    } else {
        let mut ans = 0;

        for i in stack.iter().rev() {
            let index = OPEN_BRACKETS.iter().position(|x| x == i).unwrap() as u64;
            ans *= 5;

            ans += index + 1;
        }

        return ans;
    }
}

fn main() {
    let input = include_str!("inputs/10").lines().collect::<Vec<&str>>();

    let res1: u64 = input.iter().map(|x| solve(x, true)).sum();

    println!("First part result: {}", res1);

    let mut part2 = input
        .iter()
        .filter(|x| solve(x, true) == 0)
        .map(|x| solve(x, false))
        .collect::<Vec<u64>>();
    
    part2.sort();

    let part2 = part2[part2.len() / 2];
    
    println!("Second part result: {}", part2);
}
