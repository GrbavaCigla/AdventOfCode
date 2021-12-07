fn main() {
    let mut input: Vec<i32> = include_str!("./inputs/7")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    input.sort();

    let pos;
    if input.len() % 2 == 0 {
        let half = input.len() / 2;

        pos = (input[half - 1] + input[half]) / 2;
    } else {
        pos = input[input.len() / 2];
    }

    let fuel = input.iter().fold(0, |acc, x| (acc + (pos - x).abs()));
    println!("First part result: {}", fuel);

    let pos = input.iter().sum::<i32>() / input.len() as i32;

    let fuel = input.iter().fold(0, |acc, x| {
        acc + (x - pos).abs() * ((x - pos).abs() + 1) / 2
    });
    println!("First part result: {}", fuel);
}
