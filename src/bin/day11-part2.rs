use std::collections::HashMap;
use std::fs::read_to_string;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        let device = parts[0];
        let outputs: Vec<&str> = parts[1].split_whitespace().collect();

        graph.insert(device, outputs);
    }

    graph
}

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    visited_dac: bool,
    visited_fft: bool,
    memo: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    let visited_dac = visited_dac || current == "dac";
    let visited_fft = visited_fft || current == "fft";

    if current == "out" {
        return if visited_dac && visited_fft { 1 } else { 0 };
    }

    let key = (current, visited_dac, visited_fft);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let Some(outputs) = graph.get(current) else {
        return 0;
    };

    let mut total = 0;
    for &next in outputs {
        total += count_paths(graph, next, visited_dac, visited_fft, memo);
    }

    memo.insert(key, total);
    total
}

fn main() {
    let input = read_to_string("src/input/day11.txt").expect("Failed to read input");
    let graph = parse_input(&input);

    let mut memo = HashMap::new();
    let path_count = count_paths(&graph, "svr", false, false, &mut memo);
    println!("{}", path_count);
}
