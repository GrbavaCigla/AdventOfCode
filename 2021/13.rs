use std::collections::HashSet;

fn fold(coords: &mut Vec<(u32, u32)>, fold: (&str, u32)) {
    if fold.0 == "y" {
        for (x, y) in coords.iter_mut() {
            if *y > fold.1 {
                *y = 2 * fold.1 - *y;
            }
        }
    } else {
        for (x, y) in coords.iter_mut() {
            if *x > fold.1 {
                *x = 2 * fold.1 - *x;
            }
        }
    }
}

fn main() {
    let input = include_str!("inputs/13").lines();

    let mut coords = input
        .clone()
        .filter_map(|x| x.split_once(","))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<(u32, u32)>>();

    let folds = input
        .filter(|x| x.starts_with("fold"))
        .map(|x| x[11..].split_once("=").unwrap())
        .map(|(x, y)| (x, y.parse().unwrap()))
        .collect::<Vec<(&str, u32)>>();

    fold(&mut coords, *folds.first().unwrap());

    let dots: HashSet<(u32, u32)> = coords.clone().into_iter().collect();

    println!("First part result: {}", dots.len());

    for i in folds.iter().skip(1) {
        fold(&mut coords, *i);
    }

    let dots: HashSet<(u32, u32)> = coords.clone().into_iter().collect();

    let width = dots.iter().max_by_key(|x| x.0).unwrap().0;
    let height = dots.iter().max_by_key(|x| x.1).unwrap().1;

    println!("Second part result:");
    for y in 0..height + 1 {
        for x in 0..width + 1 {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
