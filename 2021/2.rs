fn main() {
    let input: Vec<(&str, u32)> = include_str!("./inputs/2")
        .split('\n')
        .map(|x| x.split_whitespace().collect())
        .filter(|x: &Vec<&str>| x.len() == 2)
        .filter_map(|x: Vec<&str>| {
            let parsed = x[1].parse();

            if parsed.is_ok() {
                Some((x[0], parsed.unwrap()))
            } else {
                None
            }
        })
        .collect();

    let depth_positive: u32 = input.iter().filter(|x| x.0 == "down").map(|x| x.1).sum();
    let depth_negative: u32 = input.iter().filter(|x| x.0 == "up").map(|x| x.1).sum();
    let horizontal: u32 = input.iter().filter(|x| x.0 == "forward").map(|x| x.1).sum();

    println!(
        "First part result: {}",
        (depth_positive - depth_negative) * horizontal
    );

    let (aim, depth, horizontal) = input.iter().fold((0, 0, 0), |acc, x| match x.0 {
        "down" => (acc.0 + x.1, acc.1, acc.2),
        "up" => (acc.0 - x.1, acc.1, acc.2),
        "forward" => (acc.0, acc.1 + acc.0 * x.1, acc.2 + x.1),
        _ => acc,
    });

    println!("Second part result: {}", depth * horizontal);
}
