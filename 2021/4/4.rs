const BOARD_SIZE: usize = 5;

fn is_winner(board: &[u32], marked: &[u32]) -> Option<Vec<u32>> {
    let mut xs = [0; BOARD_SIZE];
    let mut ys = [0; BOARD_SIZE];
    let mut unmarked = Vec::new();

    for (i, j) in board.iter().enumerate() {
        if marked.contains(j) {
            xs[i / BOARD_SIZE] += 1;
            ys[i % BOARD_SIZE] += 1;
        } else {
            unmarked.push(*j);
        }
    }

    if xs.contains(&BOARD_SIZE) || ys.contains(&BOARD_SIZE) {
        return Some(unmarked);
    }

    None
}

fn main() {
    let input = include_str!("./input").split_once("\n").unwrap();

    let marked: Vec<u32> = input.0.split(",").filter_map(|x| x.parse().ok()).collect();

    let boards = input
        .1
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    let boards = boards.chunks(BOARD_SIZE * BOARD_SIZE).collect::<Vec<_>>();
    let mut won_boards = vec![false; boards.len()];
    let mut won_boards_count = 0;
    let mut first_puzzle_solved = false;

    'marked: for i in 1..marked.len() {
        for (j, board) in boards.iter().enumerate() {
            let winner = is_winner(board, &marked[0..i]);

            if let Some(w) = winner {
                let remaining: u32 = w.iter().sum();

                if !won_boards[j] {
                    won_boards[j] = true;
                    won_boards_count += 1;
                    let remaining: u32 = w.iter().sum();
                    if won_boards_count == boards.len() {
                        println!("Second part result: {}", remaining * marked[i - 1]);
                        break 'marked;
                    }
                }

                if !first_puzzle_solved {
                    println!("First part result: {}", remaining * marked[i - 1]);
                    first_puzzle_solved = true;
                }
            }
        }
    }
}
