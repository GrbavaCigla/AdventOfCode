use std::{collections::{HashSet, VecDeque}, usize};

fn check_surrounding(input: &VecDeque<u32>, width: usize, index: usize) -> Option<u32> {
    let width = width as isize;

    let points = [
        -1,
        1,
        // width - 1,
        width,
        // width + 1,
        // -1 - width,
        -width,
        // 1 - width,
    ];

    for (i, j) in points.iter().enumerate() {
        let pos = (index as isize + j) as usize;

        if let Some(p) = input.iter().nth(pos) {
            if input[pos] <= input[index] {
                return None;
            }
        }
    }

    Some(input[index] + 1)
}

fn expand_basin(input: &VecDeque<u32>, width: usize, index: usize, seen: &mut HashSet<usize>) -> u32 {
    let width = width as isize;

    let points = [
        -1,
        1,
        // width - 1,
        width,
        // width + 1,
        // -1 - width,
        -width,
        // 1 - width,
    ];

    let mut ans = 0;

    for (i, j) in points.iter().enumerate() {
        let pos = (index as isize + j) as usize;

        if let Some(p) = input.iter().nth(pos) {
            if input[pos] > input[index] && input[pos] != 9 && !seen.contains(&pos) {
                seen.insert(pos);
                ans += expand_basin(input, width as usize, pos, seen) + 1;
            }
        }
    }

    ans
}

fn main() {
    let input = include_str!("./inputs/9");
    let width = input.lines().nth(0).unwrap().len() + 2;

    let heatmap = input
        .lines()
        .map(|x| {
            let mut res = x
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<VecDeque<u32>>();

            res.push_front(9);
            res.push_back(9);

            res
        })
        .flatten()
        .collect::<VecDeque<u32>>();

    let mut res = 0;
    let mut basins = vec![];

    for (i, _) in heatmap.iter().enumerate() {
        let low_point = check_surrounding(&heatmap, width, i).unwrap_or_else(|| 0);
        res += low_point;

        if low_point > 0 {
            basins.push(expand_basin(&heatmap, width, i, &mut HashSet::new()) + 1);
        }

    }

    basins.sort();
    basins.reverse();

    println!("First part result: {}", res);
    println!("Second part result: {}", basins[0] * basins[1] * basins[2]);
}
