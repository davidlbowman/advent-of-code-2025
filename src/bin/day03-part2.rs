use std::fs::read_to_string;

fn main() {
    let mut total: u64 = 0;
    let input = read_to_string("src/input/day03.txt").expect("failed to read input");
    let banks = input.lines();
    let batteries = 12;

    for bank in banks {
        let mut digits: Vec<u64> = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        let mut result: u64 = 0;

        for remaining in (1..=batteries).rev() {
            let search_end = digits.len() - remaining;
            let search_slice = &digits[0..=search_end];
            let max_val = *search_slice.iter().max().unwrap();
            let index = digits.iter().position(|&x| x == max_val).unwrap();
            result = result * 10 + max_val;
            digits = digits[index + 1..].to_vec();
        }

        total += result;
    }
    println!("The total joltage is: {total}");
}
