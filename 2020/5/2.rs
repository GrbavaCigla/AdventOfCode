use std::fs::read_to_string;

fn decode_seat(seat: &str) -> i32 {
    let mut lower = 0;
    let mut upper = 127;
    let mut left = 0;
    let mut right = 7;

    let mut ans1 = 0;
    let mut ans2 = 0;

    for letter in seat.chars() {
        if letter == 'F' {
            upper = lower + (upper - lower) / 2;
            ans1 = upper;
        }else if letter == 'B' {
            lower = lower + (upper - lower) / 2 + 1;
            ans1 = lower
        }else if letter == 'L'{
            right = left + (right - left) / 2;
            ans2 = right;
        } else {
            left = left + (right - left ) / 2 + 1;
            ans2 = left;
        }
    }
    
    ans1*8 + ans2
}

fn main() {
    let input = read_to_string("./input").unwrap();

    let mut ids: Vec<i32> = input.lines().map(decode_seat).collect();
    ids.sort();

    for (i, j) in ids[1..].iter().enumerate() {
        if j - ids[i] == 2 {
            println!("{}", (j + ids[i]) / 2);
        }
    }
}