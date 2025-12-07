use std::collections::HashSet;
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

    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);

    let mut split_count = 0;

    for row in grid.iter().skip(1) {
        let mut next_beams: HashSet<usize> = HashSet::new();

        for &beam_x in &beams {
            if beam_x >= row.len() {
                continue;
            }

            match row[beam_x] {
                '.' => {
                    next_beams.insert(beam_x);
                }
                '^' => {
                    split_count += 1;
                    if beam_x > 0 {
                        next_beams.insert(beam_x - 1);
                    }
                    if beam_x + 1 < row.len() {
                        next_beams.insert(beam_x + 1);
                    }
                }
                _ => {
                    next_beams.insert(beam_x);
                }
            }
        }

        beams = next_beams;
    }

    println!("{split_count}");
}
