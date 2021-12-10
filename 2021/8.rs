use std::{collections::HashMap, fmt::format};

fn find_and_replace(target: &str, strings: &Vec<&str>, length: Option<usize>) -> Option<String> {
    let mut ans = String::from(target);

    if let Some(l) = length {
        if target.len() != l {
            return None;
        }
    }

    for i in strings.iter().flat_map(|s| s.chars()) {
        if !ans.contains(i) {
            return None;
        }
        ans = ans.replace(i, "");
    }

    Some(ans)
}

fn compare_strings(a: &str, b: &str) -> bool {
    if a.len() == b.len() {
        for i in 0..a.len() {
            if !a.contains(b.as_bytes()[i] as char) || !b.contains(a.as_bytes()[i] as char) {
                return false;
            }
        }

        return true;
    }

    false
}

fn get_number(pattern: &Vec<&str>, output: &Vec<&str>) -> u32 {
    let mut numbers = [""; 10];

    let right_edge = pattern.iter().filter(|x| x.len() == 2).next().unwrap();
    let right_edge0 = right_edge.chars().nth(0).unwrap();
    let right_edge1 = right_edge.chars().nth(1).unwrap();

    let upper_edge = pattern
        .iter()
        .filter_map(|x| find_and_replace(x, &vec![*right_edge], Some(3)))
        .next()
        .unwrap();

    numbers[4] = pattern.iter().filter(|x| x.len() == 4).next().unwrap();

    let bottom_edge = pattern
        .iter()
        .filter_map(|x| find_and_replace(x, &vec![numbers[4], &upper_edge], Some(6)))
        .next()
        .unwrap();

    let middle_edge = pattern
        .iter()
        .filter_map(|x| find_and_replace(x, &vec![right_edge, &bottom_edge, &upper_edge], Some(5)))
        .next()
        .unwrap();

    numbers[8] = pattern.iter().filter(|x| x.len() == 7).next().unwrap();
    numbers[7] = pattern.iter().filter(|x| x.len() == 3).next().unwrap();
    numbers[1] = right_edge;
    numbers[0] = pattern
        .iter()
        .filter(|x| x.len() == 6 && !x.contains(&middle_edge))
        .next()
        .unwrap();
    numbers[9] = pattern
        .iter()
        .filter(|x| x.len() == 6 && x.contains(right_edge0) && x.contains(right_edge1) && x != &&numbers[0])
        .next()
        .unwrap();
    numbers[6] = pattern
        .iter()
        .filter(|x| x.len() == 6 && x != &&numbers[9] && x != &&numbers[0])
        .next()
        .unwrap();
    numbers[3] = pattern
        .iter()
        .filter(|x| x.len() == 5 && x.contains(right_edge0) && x.contains(right_edge1))
        .next()
        .unwrap();
    

    let right_edge0 = find_and_replace(numbers[8], &vec![numbers[6]], None).unwrap();

    numbers[2] = pattern
        .iter()
        .filter(|x| x.len() == 5 && x != &&numbers[3] && x.contains(&right_edge0))
        .next()
        .unwrap();

    numbers[5] = pattern
        .iter()
        .filter(|x| x.len() == 5 && x != &&numbers[2] && x != &&numbers[3])
        .next()
        .unwrap();

    let mut ans = String::from("");
    for k in output.iter() {
        for (i, j) in numbers.iter().enumerate() {
            if compare_strings(k, j) {
                ans.push_str(&i.to_string());
            }
        }
    }
    
    ans.parse().unwrap()
}

fn main() {
    let input = include_str!("./inputs/8")
        .lines()
        .map(|x| x.split_once("|").unwrap())
        .map(|x| {
            (
                x.0.split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>(),
                x.1.split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>(),
            )
        })
        .collect::<Vec<(Vec<&str>, Vec<&str>)>>();

    let part1 = input
        .iter()
        .flat_map(|x| &x.1)
        .filter(|x| !x.is_empty())
        .fold(0, |acc, x| acc + ([2, 3, 4, 7].contains(&x.len()) as i32));

    let mut ans = 0;
    for i in input {
        ans += get_number(&i.0, &i.1);
    }

    println!("First part result: {:?}", part1);
    println!("Second part result: {:?}", ans);
}
