fn filter(nums: &Vec<&str>, index: usize, greater: bool) -> u32 {
    let ones = nums
        .iter()
        .map(|y| y.chars().nth(index).unwrap())
        .filter(|&z| z == '1')
        .count();

    if greater {
        return (ones as f32 >= nums.len() as f32 / 2.0) as u32;
    } else {
        return !(ones as f32 >= nums.len() as f32 / 2.0) as u32;
    }
}

fn second_part(input: &Vec<&str>, greater: bool) -> Option<u32> {
    let mut filtered: Vec<&str> = input.clone();
    let mut curr = String::from("");

    for i in 0..input[0].len() {
        let curr_digit = filter(&filtered, i, greater);
        curr.push_str(&curr_digit.to_string());

        filtered = filtered
            .iter()
            .filter(|x| x.starts_with(&curr))
            .cloned()
            .collect();

        if filtered.len() == 1 {
            return u32::from_str_radix(filtered[0], 2).ok();
        }
    }

    None
}

fn main() {
    let input: Vec<&str> = include_str!("./inputs/3").lines().collect();

    let epsilon = (0..input[0].len()).fold(String::new(), |acc, x| {
        format!("{}{}", acc, filter(&input, x, true))
    });

    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    println!(
        "First part result: {}",
        epsilon * (!epsilon & ((1 << input[0].len()) - 1))
    );

    println!(
        "Second part result: {}",
        second_part(&input, true).unwrap() * second_part(&input, false).unwrap()
    );
}
