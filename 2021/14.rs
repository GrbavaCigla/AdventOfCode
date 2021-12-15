use std::collections::HashMap;

fn solve(depth: usize, template: &Vec<char>, pairs: &HashMap<(char, char), char>) -> u64 {
    let mut pair_count = HashMap::<(char, char), u64>::new();

    for i in template.windows(2) {
        *pair_count.entry((i[0], i[1])).or_insert(0) += 1;
    }

    for _ in 0..depth {
        for (pair, count) in pair_count.clone() {
            if let Some(x) = pairs.get(&pair) {
                *pair_count.entry((pair.0, *x)).or_insert(0) += count;
                *pair_count.entry((*x, pair.1)).or_insert(0) += count;
                *pair_count.get_mut(&pair).unwrap() -= count;
            }
        }
    }

    let mut counts = HashMap::<char, u64>::new();
    for i in pair_count.iter() {
        *counts.entry(i.0.0).or_insert(0) += i.1;
        *counts.entry(i.0.1).or_insert(0) += i.1;
    }

    *counts.entry(*template.first().unwrap()).or_insert(0) += 1;
    *counts.entry(*template.last().unwrap()).or_insert(0) += 1;

    (counts.values().max().unwrap() - counts.values().min().unwrap()) / 2
}

fn main() {
    let mut input = include_str!("inputs/14").lines();

    let template: Vec<char> = input.next().unwrap().chars().collect();

    input.next();

    let pairs: HashMap<(char, char), char> = input
        .map(|x| x.split_once(" -> ").unwrap())
        .map(|x| {
            (
                (x.0.chars().nth(0).unwrap(), x.0.chars().nth(1).unwrap()),
                x.1.chars().next().unwrap(),
            )
        })
        .collect();

    println!("First part result: {}", solve(10, &template, &pairs));
    println!("Second part result: {}", solve(40, &template, &pairs));
}
