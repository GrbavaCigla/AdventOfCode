
// This can be much improved
// Instead of addition, I can compare elements first and last elements instead
// Also using zip, filter, count... would decrease code size significantly
fn solve(numbers: &Vec<u32>, group_size: usize) -> u32 {
    let mut ans = 0;

    let mut prevsum = 0;
    for i in 0..group_size {
        prevsum += numbers[i];
    }

    for i in group_size..numbers.len() {
        let mut cursum = 0;
        for j in 0..group_size {
            cursum += numbers[i - j];
        }

        if cursum > prevsum {
            ans += 1;
        }

        prevsum = cursum;
    }

    ans
}

fn main() {
    let lines: Vec<u32> = include_str!("./input")
        .split('\n')
        .filter_map(|x| x.parse().ok())
        .collect();

    println!("First part result: {}", solve(&lines, 1));
    println!("Second part result: {}", solve(&lines, 3));
}
