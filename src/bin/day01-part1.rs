use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day01.txt").expect("Failed to read input");
    let mut counter: u32 = 0;
    let mut dial: i32 = 50;
    for line in input.lines() {
        let direction = line.chars().next();
        let amount: i32 = line[1..].parse().expect("not a number");

        match direction {
            Some('L') => dial -= amount,
            Some('R') => dial += amount,
            _ => println!("Failed to match"),
        };

        dial = dial.rem_euclid(100);

        if dial % 100 == 0 {
            counter += 1
        }
    }

    println!("The password is: {counter}");
}
