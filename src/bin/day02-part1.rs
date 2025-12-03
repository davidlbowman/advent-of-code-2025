use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day02.txt").expect("Failed to read input");
    let parts = input.split(',');
    let mut total: u64 = 0;

    for part in parts {
        let range: Vec<&str> = part.split('-').collect();
        let start: u64 = range[0].trim().parse().expect("Not a number");
        let end: u64 = range[1].trim().parse().expect("Not a number");

        for num in start..=end {
            let str = num.to_string();

            if str.len() % 2 != 0 {
                continue;
            }

            let half = str.len() / 2;
            let first = &str[..half];
            let second = &str[half..];
            if first != second {
                continue;
            }

            total += num;
        }
    }
    println!("The total number of invalid ids is: {total}")
}
