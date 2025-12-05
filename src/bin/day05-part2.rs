use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day05.txt").expect("Failed to read input");
    let (fresh_ingredient_ranges, _) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(i64, i64)> = fresh_ingredient_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect();

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let total: i64 = merged.iter().map(|(start, end)| end - start + 1).sum();

    println!("Total fresh ingredient IDs: {total}");
}
