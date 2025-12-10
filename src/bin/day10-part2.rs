use std::fs::read_to_string;

fn parse_line(line: &str) -> (Vec<i64>, Vec<Vec<usize>>) {
    let bracket_end = line.find(']').unwrap();
    let rest = &line[bracket_end + 1..];
    let brace_pos = rest.find('{').unwrap();
    let buttons_str = &rest[..brace_pos];

    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = buttons_str.chars().collect();

    while i < chars.len() {
        if chars[i] == '(' {
            let start = i + 1;
            while i < chars.len() && chars[i] != ')' {
                i += 1;
            }
            let content: String = chars[start..i].iter().collect();
            let button_indices: Vec<usize> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            buttons.push(button_indices);
        }
        i += 1;
    }

    let joltage_str = &rest[brace_pos + 1..rest.len() - 1];
    let targets: Vec<i64> = joltage_str
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    (targets, buttons)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 { 0 } else { (a / gcd(a, b)) * b }
}

fn solve_gauss(targets: &[i64], buttons: &[Vec<usize>]) -> u64 {
    let n_counters = targets.len();
    let n_buttons = buttons.len();

    if targets.iter().all(|&t| t == 0) {
        return 0;
    }

    let mut aug: Vec<Vec<i64>> = (0..n_counters)
        .map(|i| {
            let mut row = vec![0i64; n_buttons + 1];
            for (j, btn) in buttons.iter().enumerate() {
                if btn.contains(&i) {
                    row[j] = 1;
                }
            }
            row[n_buttons] = targets[i];
            row
        })
        .collect();

    let mut pivot_row = 0;
    let mut pivot_cols: Vec<usize> = Vec::new();

    for col in 0..n_buttons {
        let mut found_pivot = None;
        for row in pivot_row..n_counters {
            if aug[row][col] != 0 {
                found_pivot = Some(row);
                break;
            }
        }

        let Some(pivot_r) = found_pivot else {
            continue;
        };

        aug.swap(pivot_row, pivot_r);

        let pivot_val = aug[pivot_row][col];
        for row in 0..n_counters {
            if row != pivot_row && aug[row][col] != 0 {
                let row_val = aug[row][col];
                let l = lcm(pivot_val.abs(), row_val.abs());
                let mult_pivot = l / pivot_val.abs();
                let mult_row = l / row_val.abs();
                let sign = if (pivot_val > 0) == (row_val > 0) { -1 } else { 1 };

                for c in 0..=n_buttons {
                    aug[row][c] = aug[row][c] * mult_row + sign * aug[pivot_row][c] * mult_pivot;
                }

                let g = aug[row].iter().fold(0i64, |acc, &x| gcd(acc, x));
                if g > 1 {
                    for c in 0..=n_buttons {
                        aug[row][c] /= g;
                    }
                }
            }
        }

        pivot_cols.push(col);
        pivot_row += 1;

        if pivot_row >= n_counters {
            break;
        }
    }

    for row in pivot_row..n_counters {
        if aug[row][n_buttons] != 0 {
            return u64::MAX;
        }
    }

    let free_cols: Vec<usize> = (0..n_buttons)
        .filter(|c| !pivot_cols.contains(c))
        .collect();

    let max_free: Vec<i64> = free_cols
        .iter()
        .map(|&c| {
            buttons[c]
                .iter()
                .map(|&i| targets[i])
                .min()
                .unwrap_or(0)
        })
        .collect();

    let mut best = u64::MAX;

    fn search(
        aug: &[Vec<i64>],
        pivot_cols: &[usize],
        free_cols: &[usize],
        max_free: &[i64],
        n_buttons: usize,
        free_idx: usize,
        free_vals: &mut Vec<i64>,
        best: &mut u64,
    ) {
        if free_idx >= free_cols.len() {
            let mut x = vec![0i64; n_buttons];
            for (i, &c) in free_cols.iter().enumerate() {
                x[c] = free_vals[i];
            }

            for (r, &pc) in pivot_cols.iter().enumerate() {
                let pivot_val = aug[r][pc];
                let mut rhs = aug[r][n_buttons];
                for c in 0..n_buttons {
                    if c != pc {
                        rhs -= aug[r][c] * x[c];
                    }
                }
                if rhs % pivot_val != 0 {
                    return;
                }
                x[pc] = rhs / pivot_val;
            }

            if x.iter().any(|&v| v < 0) {
                return;
            }

            let sum: u64 = x.iter().map(|&v| v as u64).sum();
            if sum < *best {
                *best = sum;
            }
            return;
        }

        let max_val = if *best == u64::MAX {
            max_free[free_idx]
        } else {
            max_free[free_idx].min(*best as i64)
        };
        for val in 0..=max_val {
            free_vals.push(val);
            search(
                aug,
                pivot_cols,
                free_cols,
                max_free,
                n_buttons,
                free_idx + 1,
                free_vals,
                best,
            );
            free_vals.pop();
        }
    }

    let mut free_vals = Vec::new();
    search(
        &aug,
        &pivot_cols,
        &free_cols,
        &max_free,
        n_buttons,
        0,
        &mut free_vals,
        &mut best,
    );

    best
}

fn main() {
    let input = read_to_string("src/input/day10.txt").expect("Failed to read input");

    let total: u64 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (targets, buttons) = parse_line(line);
            solve_gauss(&targets, &buttons)
        })
        .sum();

    println!("{}", total);
}
