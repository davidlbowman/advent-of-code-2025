use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day01.txt").expect("Failed to read input");
    let mut counter: u32 = 0;
    let mut dial: i32 = 50;

    for line in input.lines() {
        let direction = line.chars().next();
        let amount: i32 = line[1..].parse().expect("not a number");

        let full_crossings = amount / 100;
        let remainder = amount % 100;

        let partial_crossing = match direction {
            Some('L') => {
                if dial > 0 && dial <= remainder {
                    1
                } else {
                    0
                }
            }
            Some('R') => {
                if dial + remainder >= 100 {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        };

        counter += (full_crossings + partial_crossing) as u32;

        match direction {
            Some('L') => dial -= amount,
            Some('R') => dial += amount,
            _ => println!("Failed to match"),
        };

        dial = dial.rem_euclid(100);
    }

    println!("The password is: {counter}");
}
