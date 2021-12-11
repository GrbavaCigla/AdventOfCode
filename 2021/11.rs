use std::{collections::{HashSet, VecDeque}, usize};

const WIDTH: usize = 10;

// To lazy to do padding...
fn do_flash(input: &mut Vec<u32>, index: usize, flashed: &mut HashSet<usize>) -> u32 {
    let mut ans = 1;
    let width = WIDTH as i32;
    let mut points = vec![width, -width];

    if index % WIDTH == 0 {
        points.push(width + 1);
        points.push(-width + 1);
        points.push(1);
    } else if (index + 1) % WIDTH == 0 {
        points.push(width - 1);
        points.push(-width - 1);
        points.push(-1);
    } else {
        points.push(width - 1);
        points.push(-width - 1);
        points.push(-1);
        points.push(width + 1);
        points.push(-width + 1);
        points.push(1);
    }

    for i in points {
        let pos = index as i32 + i;
        if pos < 0 || pos >= input.len() as i32 {
            continue;
        }

        let pos = pos as usize;

        input[pos] += 1;
        if input[pos] > 9 && !flashed.contains(&pos) {
            flashed.insert(pos);
            ans += do_flash(input, pos, flashed);
        }
    }

    ans
}

fn next_step(input: &mut Vec<u32>) -> u32 {
    let mut flashes = HashSet::new();

    for (i, energy) in input.iter_mut().enumerate() {
        *energy += 1;

        if *energy > 9 {
            flashes.insert(i);
        }
    }

    let mut flashes_count = 0;
    
    for flash in flashes.clone().iter() {
        flashes_count += do_flash(input, *flash, &mut flashes);
    }

    for i in input.iter_mut() {
        if *i > 9 {
            *i = 0;
        }
    }

    flashes_count
}

fn print_octo(input: &Vec<u32>) {
    for (i, energy) in input.iter().enumerate() {
        if *energy > 9 {
            print!("^");
        } else {
            print!("{}", energy);
        }
        if (i + 1) % WIDTH == 0 {
            println!();
        }
    }
    println!();

}

fn main() {
    let mut input: Vec<u32> = include_str!("./inputs/11")
        .lines()
        .flat_map(|x| x.chars())
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut ans = 0;
    for i in 0..100 {
        ans += next_step(&mut input);
    }

    let mut steps = 100;
    loop {
        next_step(&mut input);
        steps += 1;
        if input.iter().sum::<u32>() == 0 {
            break;
        }
    }

    println!("First part result: {}", ans);
    println!("Second part result: {}", steps);
}
