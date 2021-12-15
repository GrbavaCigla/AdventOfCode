use std::collections::HashMap;

// TODO: Add up and left directions too
fn solve(map: &Vec<Vec<u32>>) -> u32 {
    let mut memo = HashMap::<(usize, usize), u32>::new();

    fn path_find(
        x: usize,
        y: usize,
        map: &Vec<Vec<u32>>,
        memo: &mut HashMap<(usize, usize), u32>,
    ) -> u32 {
        if x == 0 && y == 0 {
            return 0;
        }

        if memo.contains_key(&(x, y)) {
            return *memo.get(&(x, y)).unwrap();
        }

        let curr = map[y][x];
        let ans;

        if x > 0 && y > 0 {
            let f = path_find(x - 1, y, map, memo);
            let s = path_find(x, y - 1, map, memo);

            ans = f.min(s) + curr;
        } else if x > 0 {
            ans = path_find(x - 1, y, map, memo) + curr;
        } else {
            ans = path_find(x, y - 1, map, memo) + curr;
        }

        memo.insert((x, y), ans);

        ans
    }

    path_find(map[0].len() - 1, map.len() - 1, map, &mut memo)
}

fn main() {
    let input = include_str!("inputs/15")
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("{}", solve(&input));
}
