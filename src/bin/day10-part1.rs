use std::fs::read_to_string;

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let pattern = &line[bracket_start + 1..bracket_end];

    let mut target: u64 = 0;
    for (i, c) in pattern.chars().enumerate() {
        if c == '#' {
            target |= 1 << i;
        }
    }

    let rest = &line[bracket_end + 1..];
    let brace_pos = rest.find('{').unwrap_or(rest.len());
    let buttons_str = &rest[..brace_pos];

    let mut buttons: Vec<u64> = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = buttons_str.chars().collect();

    while i < chars.len() {
        if chars[i] == '(' {
            let start = i + 1;
            while i < chars.len() && chars[i] != ')' {
                i += 1;
            }
            let content: String = chars[start..i].iter().collect();

            let mut button_mask: u64 = 0;
            for part in content.split(',') {
                let idx: usize = part.trim().parse().unwrap();
                button_mask |= 1 << idx;
            }
            buttons.push(button_mask);
        }
        i += 1;
    }

    (target, buttons)
}

fn min_presses(target: u64, buttons: &[u64]) -> u32 {
    let n = buttons.len();
    let mut min_count = u32::MAX;

    for mask in 0u64..(1 << n) {
        let mut state: u64 = 0;
        let mut count = 0;

        for (i, &button) in buttons.iter().enumerate() {
            if mask & (1 << i) != 0 {
                state ^= button;
                count += 1;
            }
        }

        if state == target && count < min_count {
            min_count = count;
        }
    }

    min_count
}

fn main() {
    let input = read_to_string("src/input/day10.txt").expect("Failed to read input");

    let total: u32 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (target, buttons) = parse_line(line);
            min_presses(target, buttons.as_slice())
        })
        .sum();

    println!("{}", total);
}
