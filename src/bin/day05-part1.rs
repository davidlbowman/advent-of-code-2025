use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day05.txt").expect("Failed to read input");
    let (fresh_ingredient_ranges, active_ingredient_ids) = input.split_once("\n\n").unwrap();
    let mut total_available_ingredients: i32 = 0;

    let ids: Vec<i64> = active_ingredient_ids
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let ranges: Vec<(i64, i64)> = fresh_ingredient_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect();

    for id in ids {
        for &(range_start, range_end) in &ranges {
            if id >= range_start && id <= range_end {
                total_available_ingredients += 1;
                break;
            }
        }
    }

    println!("There are {total_available_ingredients} available ingredients");
}
