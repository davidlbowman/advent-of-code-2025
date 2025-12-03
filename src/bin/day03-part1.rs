use std::fs::read_to_string;

fn main() {
    let mut total = 0;
    let input = read_to_string("src/input/day03.txt").expect("failed to read input");
    let banks = input.lines();

    for bank in banks {
        if bank.is_empty() {
            continue;
        }

        let chars: Vec<char> = bank.chars().collect();

        let mut sorted = chars.clone();
        sorted.sort();
        sorted.reverse();

        let largest = sorted[0];
        let second_largest = sorted[1];

        let joltage = if largest == second_largest {
            let first_digit = largest.to_digit(10).unwrap();
            let second_digit = second_largest.to_digit(10).unwrap();
            first_digit * 10 + second_digit
        } else {
            let pos = chars.iter().position(|&c| c == largest).unwrap();

            if pos == chars.len() - 1 {
                let first_digit = second_largest.to_digit(10).unwrap();
                let second_digit = largest.to_digit(10).unwrap();
                first_digit * 10 + second_digit
            } else {
                let rest = &chars[pos + 1..];
                let next_largest = rest.iter().max().unwrap();
                let first_digit = largest.to_digit(10).unwrap();
                let second_digit = next_largest.to_digit(10).unwrap();
                first_digit * 10 + second_digit
            }
        };

        total += joltage;
    }
    println!("The total joltage is: {total}");
}
