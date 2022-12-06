fn main() {
    println!(
        "part 1: {}",
        include_str!("../inputs/4.txt")
            .lines()
            .map(|x| x.split(&['-', ','][..]).collect::<Vec<&str>>())
            .map(|x| x.iter().map(|y| y.parse().unwrap()).collect::<Vec<u32>>())
            .map(|x| x[0] >= x[2] && x[1] <= x[3] || x[0] <= x[2] && x[1] >= x[3])
            .map(|x| x as usize)
            .sum::<usize>()
    );

    println!(
        "part 2: {}",
        include_str!("../inputs/4.txt")
            .lines()
            .map(|x| x.split(&['-', ','][..]).collect::<Vec<&str>>())
            .map(|x| x.iter().map(|y| y.parse().unwrap()).collect::<Vec<u32>>())
            .map(|x| x[0] >= x[2] && x[0] <= x[3]
                || x[1] >= x[2] && x[1] <= x[3]
                || x[2] >= x[0] && x[2] <= x[1]
                || x[3] >= x[0] && x[3] <= x[1])
            .map(|x| x as usize)
            .sum::<usize>()
    );
}
