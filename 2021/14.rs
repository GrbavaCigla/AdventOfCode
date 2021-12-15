use std::collections::{HashMap, HashSet};

fn next_step(input: &Vec<char>, pairs: &HashMap<&str, char>) -> Vec<char> {
    let mut ans = vec![];

    for i in input.windows(2) {
        let pair = format!("{}{}", i[0], i[1]);
        let slice = &pair[..];

        ans.push(i[0]);
        if pairs.contains_key(&slice) {
            ans.push(*pairs.get(slice).unwrap());
        }
    }
    ans.push(*input.last().unwrap());

    ans
}

fn main() {
    let mut input = include_str!("inputs/14").lines();

    let mut template: Vec<char> = input.next().unwrap().chars().collect();

    input.next();

    let pairs: HashMap<&str, char> = input
        .map(|x| x.split_once(" -> ").unwrap())
        .map(|x| (x.0, x.1.chars().next().unwrap()))
        .collect();

    for i in 0..5 {
        template = next_step(&template, &pairs);
    }

    println!("{:?}", template);

    let chars: HashSet<char> = template.clone().into_iter().collect();
    let chars: Vec<char> = chars.into_iter().collect();

    let occurances = template.iter().fold(vec![0; chars.len()], |acc, x| {
        let mut new_acc = acc.clone();

        new_acc[chars.iter().position(|y| y == x).unwrap()] += 1;

        new_acc
    });

    println!("First part result: {}", occurances.iter().max().unwrap() - occurances.iter().min().unwrap());
}
