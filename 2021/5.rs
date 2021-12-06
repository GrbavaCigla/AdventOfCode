use std::{
    cmp::{max, min},
    collections::HashMap,
};

fn get_points(line: &(i32, i32, i32, i32), count_diags: bool) -> Vec<(i32, i32)> {
    let &(x1, y1, x2, y2) = line;

    if x1 == x2 {
        return (min(y1, y2)..=max(y1, y2)).map(|x| (x1, x)).collect();
    } else if y1 == y2 {
        return (min(x1, x2)..=max(x1, x2)).map(|x| (x, y1)).collect();
    } else if x1 - x2 == y1 - y2 && count_diags {
        return (min(x1, x2)..=max(x1, x2))
            .zip(min(y1, y2)..=max(y1, y2))
            .collect();
    } else if -(x1 - x2) == y1 - y2 && count_diags {
        return (min(x1, x2)..=max(x1, x2)).rev()
            .zip(min(y1, y2)..=max(y1, y2))
            .collect();
    } else {
        return vec![];
    }
}

fn solve(input: &Vec<(i32, i32, i32, i32)>, count_diags: bool) -> usize {
    let points = input
        .iter()
        .flat_map(|x| get_points(x, count_diags))
        .collect::<Vec<(i32, i32)>>();
    let mut counter: HashMap<(i32, i32), i32> = HashMap::new();

    for point in points {
        *counter.entry(point).or_insert(0) += 1;
    }

    counter.iter().filter(|x| *x.1 > 1).count()
}

fn main() {
    let input: Vec<(i32, i32, i32, i32)> = include_str!("./inputs/5")
        .lines()
        .map(|x| {
            x.split("->")
                .flat_map(|x| x.split(","))
                .map(|x| x.trim())
                .collect::<Vec<&str>>()
        })
        .map(|x| (x[0], x[1], x[2], x[3]))
        .map(|x| {
            (
                x.0.parse::<i32>().unwrap(),
                x.1.parse::<i32>().unwrap(),
                x.2.parse::<i32>().unwrap(),
                x.3.parse::<i32>().unwrap(),
            )
        })
        .collect();

        println!("First part result: {}", solve(&input, false));
        println!("Second part result: {}", solve(&input, true));
}
