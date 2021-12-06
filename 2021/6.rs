fn next_day(state: &mut [u64]) {
    let state_zero = state[0];

    for i in 1..state.len() {
        state[i - 1] = state[i];
        state[i] = 0;
    }

    state[6] += state_zero;
    state[8] += state_zero;
}

fn main() {
    let mut lanternfishes = include_str!("./inputs/6")
        .split(",")
        .map(|x| x.parse().unwrap())
        .fold([0; 9], |acc, x: usize| {
            let mut ac = acc;
            ac[x] += 1;
            ac
        });

    for _ in 0..80 {
        next_day(&mut lanternfishes);
    }

    println!("First part result: {}", lanternfishes.iter().sum::<u64>());

    for _ in 80..256 {
        next_day(&mut lanternfishes);
    }

    println!("Second part result: {}", lanternfishes.iter().sum::<u64>());
}
