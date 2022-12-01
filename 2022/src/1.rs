fn main() {
    let mut arr = include_str!("../inputs/1.txt")
        .split("\n")
        .fold((0, Vec::new()), |mut acc, x| {
            if x == "" {
                acc.1.push(acc.0);
                (0, acc.1)
            } else {
                (acc.0 + x.parse::<i32>().unwrap(), acc.1)
            }
        })
        .1;
    arr.sort();
    arr.reverse();

    println!("part 1: {}", arr[0]);

    println!("part 2: {}", arr[0] + arr[1] + arr[2]);
}
