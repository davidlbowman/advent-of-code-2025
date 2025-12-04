use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day04.txt").expect("Failed to read input");
    let lines = input.lines();
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut count: i32 = 0;

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] != '@' {
                continue;
            }

            let max_row = grid.len() - 1;
            let max_col = grid[0].len() - 1;
            let mut neighbors: u32 = 0;

            // Top-left
            if row > 0 && col > 0 && grid[row - 1][col - 1] == '@' {
                neighbors += 1;
            }

            // Top
            if row > 0 && grid[row - 1][col] == '@' {
                neighbors += 1;
            }

            // Top-right
            if row > 0 && col < max_col && grid[row - 1][col + 1] == '@' {
                neighbors += 1;
            }

            // Left
            if col > 0 && grid[row][col - 1] == '@' {
                neighbors += 1;
            }

            // Right
            if col < max_col && grid[row][col + 1] == '@' {
                neighbors += 1;
            }

            // Bottom-left
            if row < max_row && col > 0 && grid[row + 1][col - 1] == '@' {
                neighbors += 1;
            }

            // Bottom
            if row < max_row && grid[row + 1][col] == '@' {
                neighbors += 1;
            }

            // Bottom-right
            if row < max_row && col < max_col && grid[row + 1][col + 1] == '@' {
                neighbors += 1;
            }

            if neighbors < 4 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
