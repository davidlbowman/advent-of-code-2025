use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day07.txt").expect("Failed to read input");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut start_col = 0;
    for row in grid.iter() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_col = col_idx;
                break;
            }
        }
        if start_col != 0 {
            break;
        }
    }

    let mut timelines: HashMap<usize, u64> = HashMap::new();
    timelines.insert(start_col, 1);

    for row in grid.iter().skip(1) {
        let mut next_timelines: HashMap<usize, u64> = HashMap::new();

        for (&beam_x, &count) in &timelines {
            if beam_x >= row.len() {
                *next_timelines.entry(beam_x).or_insert(0) += count;
                continue;
            }

            match row[beam_x] {
                '.' => {
                    *next_timelines.entry(beam_x).or_insert(0) += count;
                }
                '^' => {
                    if beam_x > 0 {
                        *next_timelines.entry(beam_x - 1).or_insert(0) += count;
                    }
                    if beam_x + 1 < row.len() {
                        *next_timelines.entry(beam_x + 1).or_insert(0) += count;
                    }
                }
                _ => {
                    *next_timelines.entry(beam_x).or_insert(0) += count;
                }
            }
        }

        timelines = next_timelines;
    }

    let total: u64 = timelines.values().sum();
    println!("{total}");
}
