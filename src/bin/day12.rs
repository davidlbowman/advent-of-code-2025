use std::collections::HashSet;
use std::fs::read_to_string;

type Shape = Vec<(i32, i32)>;
type Region = (usize, usize, Vec<usize>);

fn parse_shape(lines: &[&str]) -> Shape {
    let mut cells = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                cells.push((row as i32, col as i32));
            }
        }
    }
    normalize(&cells)
}

fn normalize(shape: &Shape) -> Shape {
    if shape.is_empty() {
        return Vec::new();
    }
    let min_row = shape.iter().map(|&(r, _)| r).min().unwrap();
    let min_col = shape.iter().map(|&(_, c)| c).min().unwrap();
    let mut normalized: Shape = shape
        .iter()
        .map(|&(r, c)| (r - min_row, c - min_col))
        .collect();
    normalized.sort();
    normalized
}

fn rotate_90(shape: &Shape) -> Shape {
    let rotated: Shape = shape.iter().map(|&(r, c)| (c, -r)).collect();
    normalize(&rotated)
}

fn flip_horizontal(shape: &Shape) -> Shape {
    let flipped: Shape = shape.iter().map(|&(r, c)| (r, -c)).collect();
    normalize(&flipped)
}

fn get_all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations = HashSet::new();
    let mut current = shape.clone();

    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }

    current = flip_horizontal(shape);
    for _ in 0..4 {
        orientations.insert(current.clone());
        current = rotate_90(&current);
    }

    orientations.into_iter().collect()
}

fn is_region_line(line: &str) -> bool {
    if let Some(colon_pos) = line.find(':') {
        let dims_part = &line[..colon_pos];
        dims_part.contains('x') && dims_part.chars().all(|c| c.is_ascii_digit() || c == 'x')
    } else {
        false
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<Region>) {
    let mut shapes: Vec<Vec<Shape>> = Vec::new();
    let mut regions = Vec::new();
    let mut current_lines: Vec<&str> = Vec::new();
    let mut in_shape = false;
    let mut parsing_regions = false;

    for line in input.lines() {
        if is_region_line(line) {
            if in_shape && !current_lines.is_empty() {
                let shape = parse_shape(&current_lines);
                shapes.push(get_all_orientations(&shape));
                current_lines.clear();
                in_shape = false;
            }
            parsing_regions = true;

            let colon_pos = line.find(':').unwrap();
            let dims_part = &line[..colon_pos];
            let counts_part = &line[colon_pos + 1..];

            let dims: Vec<usize> = dims_part.split('x').map(|s| s.parse().unwrap()).collect();
            let width = dims[0];
            let height = dims[1];

            let counts: Vec<usize> = counts_part
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            regions.push((width, height, counts));
        } else if !parsing_regions {
            if line.contains(':') && !line.contains('x') {
                if in_shape && !current_lines.is_empty() {
                    let shape = parse_shape(&current_lines);
                    shapes.push(get_all_orientations(&shape));
                    current_lines.clear();
                }
                in_shape = true;
            } else if in_shape && !line.is_empty() {
                current_lines.push(line);
            }
        }
    }

    if !current_lines.is_empty() {
        let shape = parse_shape(&current_lines);
        shapes.push(get_all_orientations(&shape));
    }

    (shapes, regions)
}

const EMPTY: u8 = 0;
const FILLED: u8 = 1;
const PERMANENT_EMPTY: u8 = 2;

fn can_place(
    grid: &[Vec<u8>],
    shape: &Shape,
    row: i32,
    col: i32,
    height: usize,
    width: usize,
) -> bool {
    for &(dr, dc) in shape {
        let nr = row + dr;
        let nc = col + dc;
        if nr < 0 || nc < 0 || nr >= height as i32 || nc >= width as i32 {
            return false;
        }
        if grid[nr as usize][nc as usize] != EMPTY {
            return false;
        }
    }
    true
}

fn place(grid: &mut [Vec<u8>], shape: &Shape, row: i32, col: i32) {
    for &(dr, dc) in shape {
        grid[(row + dr) as usize][(col + dc) as usize] = FILLED;
    }
}

fn unplace(grid: &mut [Vec<u8>], shape: &Shape, row: i32, col: i32) {
    for &(dr, dc) in shape {
        grid[(row + dr) as usize][(col + dc) as usize] = EMPTY;
    }
}

fn solve(
    grid: &mut Vec<Vec<u8>>,
    shapes: &[Vec<Shape>],
    remaining: &mut Vec<usize>,
    width: usize,
    height: usize,
) -> bool {
    if remaining.iter().all(|&c| c == 0) {
        return true;
    }

    let mut found = None;
    'outer: for (row, grid_row) in grid.iter().enumerate() {
        for (col, &cell) in grid_row.iter().enumerate() {
            if cell == EMPTY {
                found = Some((row, col));
                break 'outer;
            }
        }
    }

    let Some((row, col)) = found else {
        return false;
    };

    let mut any_placement = false;
    for (shape_idx, orientations) in shapes.iter().enumerate() {
        if remaining[shape_idx] == 0 {
            continue;
        }

        for orientation in orientations {
            for &(sr, sc) in orientation {
                let place_row = row as i32 - sr;
                let place_col = col as i32 - sc;

                if place_row < 0 || place_col < 0 {
                    continue;
                }

                if can_place(grid, orientation, place_row, place_col, height, width) {
                    place(grid, orientation, place_row, place_col);
                    remaining[shape_idx] -= 1;

                    if solve(grid, shapes, remaining, width, height) {
                        return true;
                    }

                    remaining[shape_idx] += 1;
                    unplace(grid, orientation, place_row, place_col);
                    any_placement = true;
                }
            }
        }
    }

    if !any_placement {
        grid[row][col] = PERMANENT_EMPTY;
        let result = solve(grid, shapes, remaining, width, height);
        grid[row][col] = EMPTY;
        return result;
    }

    false
}

fn can_fit_region(shapes: &[Vec<Shape>], width: usize, height: usize, counts: &[usize]) -> bool {
    let mut total_cells = 0;
    for (shape_idx, &count) in counts.iter().enumerate() {
        if count > 0 && shape_idx < shapes.len() {
            total_cells += shapes[shape_idx][0].len() * count;
        }
    }

    if total_cells > width * height {
        return false;
    }

    let mut grid = vec![vec![EMPTY; width]; height];
    let mut remaining: Vec<usize> = counts.to_vec();

    while remaining.len() < shapes.len() {
        remaining.push(0);
    }

    solve(&mut grid, shapes, &mut remaining, width, height)
}

fn main() {
    let input = read_to_string("src/input/day12.txt").expect("Failed to read input");
    let (shapes, regions) = parse_input(&input);

    let count = regions
        .iter()
        .filter(|(width, height, counts)| can_fit_region(&shapes, *width, *height, counts))
        .count();

    println!("{}", count);
}
