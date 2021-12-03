fn filter_out(input: &Vec<&str>, start: usize, length: usize) -> Option<(u32, u32)> {
    let mut epsilon = String::new();
    let mut gamma = String::new();

    for i in start..length {
        let zeros = input
            .iter()
            .map(|x| x.chars().nth(i))
            .filter(|x| x == &Some('0'))
            .count();

        let ones = length - zeros;

        println!("1: {} - 0: {}", ones, zeros);

        if ones > zeros {
            epsilon.push('1');
            gamma.push('0');
        } else if ones < zeros {
            epsilon.push('0');
            gamma.push('1');
        } else {
            epsilon.push('1');
            gamma.push('0');
        }
    }

    let epsilon = u32::from_str_radix(&epsilon, 2).ok();
    let gamma = u32::from_str_radix(&gamma, 2).ok();

    if epsilon.is_some() && gamma.is_some() {
        return Some((epsilon.unwrap(), gamma.unwrap()));
    } else {
        return None;
    }
}

fn main() {
    let mut input: Vec<&str> = include_str!("./input").lines().collect();
    input.sort();

    let first = filter_out(&input, 0, input.len());

    println!("First part result: {}", first.unwrap().1 * first.unwrap().0);

}
