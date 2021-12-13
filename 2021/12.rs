use std::collections::{HashMap, HashSet};

// Bad solution memory-wise, to lazy to improve
fn dfs<'a>(graph: &HashMap<&str, Vec<&str>>, current: &'a str, passed: HashSet<&'a str>, part2: bool) -> u32 {
    let mut new_passed = HashSet::from(passed);
    if current.to_lowercase() == current {
        new_passed.insert(current);
    }
    
    if current == "end" {
        return 1;
    }
    
    let mut ans = 0;
    for i in graph.get(current).unwrap().iter() {
        if new_passed.contains(i) {
            if part2 && *i != "start" {
                ans += dfs(graph, i, new_passed.clone(), false);
            }
        } else {
            ans += dfs(graph, i, new_passed.clone(), part2);
        }
    }

    ans
}

fn main() {
    let input = include_str!("./inputs/12")
        .lines()
        .map(|x| x.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut graph = HashMap::new();

    for (a, b) in input {
        graph.entry(a).or_insert(Vec::<&str>::new());
        graph.entry(b).or_insert(Vec::<&str>::new());

        graph.get_mut(a).unwrap().push(b);
        graph.get_mut(b).unwrap().push(a);
    }

    println!("{:?}", dfs(&graph, "start", HashSet::new(), false));
    println!("{:?}", dfs(&graph, "start", HashSet::new(), true));
}
