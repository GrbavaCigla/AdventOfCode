use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./1/input")?;
    let reader = BufReader::new(file);

    let mut ans = 0;

    let lines: Vec<i32> = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    let mut prev: i32 = lines[0];

    for num in lines.iter() {
        if *num > prev {
            ans += 1;
        }
        prev = *num;
    }

    println!("First part result: {}", ans);

    ans = 0;
    let mut prevsum = lines[0] + lines[1] + lines[2];

    for (i, num) in lines.iter().enumerate().skip(3) {
        let cursum = num + lines[i - 1] + lines[i - 2];

        if cursum > prevsum {
            ans += 1;
        }

        prevsum = cursum;

    }

    println!("Second part result: {}", ans);

    Ok(())
}
