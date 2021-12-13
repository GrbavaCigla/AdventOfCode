use image::{ImageBuffer, RgbImage};
use std::{collections::HashSet, fs};

const WIDTH: usize = 10;
const CELL_SIZE: usize = 20;

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

fn create_image(input: &Vec<u32>, step: u32) {
    let size = (WIDTH * CELL_SIZE) as u32;

    let mut img = ImageBuffer::from_fn(size, size, |x, y| {
        let x = x / CELL_SIZE as u32;
        let y = y / CELL_SIZE as u32;

        let index = x as usize + y as usize * WIDTH;

        image::Luma([(input[index] as f32 / 9.0 * 256.0) as u8])
    });

    println!("{}", format!("/tmp/vis/{:0>3}.png", step));
    img.save(format!("/tmp/vis/{:0>3}.png", step)).unwrap();
}

fn main() {
    let mut input: Vec<u32> = include_str!("./inputs/11")
        .lines()
        .flat_map(|x| x.chars())
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    fs::create_dir("/tmp/vis");

    let mut step = 0;
    loop {
        next_step(&mut input);
        create_image(&input, step);
        step += 1;
        if input.iter().sum::<u32>() == 0 {
            break;
        }
    }
}
