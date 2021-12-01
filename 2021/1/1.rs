use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};


fn solve(numbers: &Vec<u32>, group_size: usize) -> u32 {
    let mut ans = 0;

    let mut prevsum = 0;
    for i in 0..group_size {
        prevsum += numbers[i];
    }

    for i in group_size..numbers.len() {
        let mut cursum = 0;
        for j in 0..group_size {
            cursum += numbers[i - j];
        }

        if cursum > prevsum {
            ans += 1;
        }

        prevsum = cursum;
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./1/input")?;
    let reader = BufReader::new(file);

    let lines: Vec<u32> = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    println!("First part result: {}", solve(&lines, 1));
    println!("Second part result: {}", solve(&lines, 3));

    Ok(())
}
