use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day04.txt").expect("Failed to read input");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut count: i32 = 0;

    for row in 0..height {
        for col in 0..width {
            if grid[row as usize][col as usize] != '@' {
                continue;
            }

            let mut neighbors: u32 = 0;

            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = row + dr;
                    let nc = col + dc;
                    if nr >= 0
                        && nr < height
                        && nc >= 0
                        && nc < width
                        && grid[nr as usize][nc as usize] == '@'
                    {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
