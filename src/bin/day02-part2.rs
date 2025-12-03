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
            let s = num.to_string();
            for i in 1..=s.len() / 2 {
                if s.len() % i == 0 {
                    let pattern = &s[..i];
                    if pattern.repeat(s.len() / i) == s {
                        total += num;
                        break;
                    }
                }
            }
        }
    }
    println!("The total number of invalid ids is: {total}")
}
