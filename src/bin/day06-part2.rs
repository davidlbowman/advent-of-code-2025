use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day06.txt").expect("Failed to read input");
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let operator_line = lines.last().unwrap();
    let num_lines: Vec<&str> = lines[..lines.len() - 1].to_vec();

    let padded_lines: Vec<String> = num_lines
        .iter()
        .map(|l| format!("{:width$}", l, width = width))
        .collect();
    let padded_op: String = format!("{:width$}", operator_line, width = width);

    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut col = 0;

    while col < width {
        let mut is_separator = true;
        for line in &padded_lines {
            let ch = line.chars().nth(col).unwrap_or(' ');
            if ch != ' ' {
                is_separator = false;
                break;
            }
        }
        let op_ch = padded_op.chars().nth(col).unwrap_or(' ');
        if op_ch != ' ' {
            is_separator = false;
        }

        if is_separator {
            col += 1;
            continue;
        }

        let block_start = col;
        let mut block_end = col;
        while block_end < width {
            let mut all_space = true;
            for line in &padded_lines {
                let ch = line.chars().nth(block_end).unwrap_or(' ');
                if ch != ' ' {
                    all_space = false;
                    break;
                }
            }
            let op_ch = padded_op.chars().nth(block_end).unwrap_or(' ');
            if op_ch != ' ' {
                all_space = false;
            }
            if all_space {
                break;
            }
            block_end += 1;
        }

        blocks.push((block_start, block_end));
        col = block_end;
    }

    blocks.reverse();

    let mut grand_total: i64 = 0;

    for (block_start, block_end) in blocks {
        let mut operator = ' ';
        for i in block_start..block_end {
            let ch = padded_op.chars().nth(i).unwrap_or(' ');
            if ch == '+' || ch == '*' {
                operator = ch;
                break;
            }
        }

        let mut numbers: Vec<i64> = Vec::new();

        for c in (block_start..block_end).rev() {
            let mut digits = String::new();
            for line in &padded_lines {
                let ch = line.chars().nth(c).unwrap_or(' ');
                if ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }
            if !digits.is_empty()
                && let Ok(num) = digits.parse::<i64>()
            {
                numbers.push(num);
            }
        }

        if !numbers.is_empty() && operator != ' ' {
            let result: i64 = if operator == '+' {
                numbers.iter().sum()
            } else {
                numbers.iter().product()
            };
            grand_total += result;
        }
    }

    println!("Grand total: {grand_total}");
}
