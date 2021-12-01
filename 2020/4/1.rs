use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();

    let vec_input: Vec<_> = input
        .split("\n\n")
        .map(|x| x
            .replace('\n', " ")
            .replace(' ', ":")
        ).collect();

    let mut ans = 0;
    for i in vec_input.iter() {
        let fields: Vec<_> = i.split(":").step_by(2).collect();
        
        let cid_exists = fields.contains(&"cid");
        let len = fields.len();
        
        if len == 8 || (len == 7 && !cid_exists) {
            ans += 1;
        }
    }

    println!("{}", ans);
}