use std::fs::read_to_string;

fn count_neighbors(grid: &[Vec<char>], row: usize, col: usize) -> u32 {
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

    neighbors
}

fn main() {
    let input = read_to_string("src/input/day04.txt").expect("Failed to read input");
    let lines = input.lines();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let mut total_removed: i32 = 0;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] != '@' {
                    continue;
                }

                if count_neighbors(&grid, row, col) < 4 {
                    to_remove.push((row, col));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (row, col) in &to_remove {
            grid[*row][*col] = '.';
        }

        total_removed += to_remove.len() as i32;
    }

    println!("{}", total_removed);
}
