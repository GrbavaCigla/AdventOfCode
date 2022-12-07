use std::{cmp::min, collections::HashSet};

fn main() {
    println!(
        "part 1: {}",
        include_str!("../inputs/6.txt")
            .chars()
            .enumerate()
            .fold((String::new(), usize::MAX), |accum, x| {
                if accum.0.len() >= 4 {
                    let mut uniq = HashSet::new();
                    if accum.0.chars().all(move |y| uniq.insert(y)) {
                        return (format!("{}{}", &accum.0[1..], x.1), min(x.0, accum.1));
                    }

                    (format!("{}{}", &accum.0[1..], x.1), accum.1)
                } else {
                    (format!("{}{}", accum.0, x.1), accum.1)
                }
            })
            .1
    );

    println!(
        "part 2: {}",
        include_str!("../inputs/6.txt")
            .chars()
            .enumerate()
            .fold((String::new(), usize::MAX), |accum, x| {
                if accum.0.len() >= 14 {
                    let mut uniq = HashSet::new();
                    if accum.0.chars().all(move |y| uniq.insert(y)) {
                        return (format!("{}{}", &accum.0[1..], x.1), min(x.0, accum.1));
                    }

                    (format!("{}{}", &accum.0[1..], x.1), accum.1)
                } else {
                    (format!("{}{}", accum.0, x.1), accum.1)
                }
            })
            .1
    );
}
