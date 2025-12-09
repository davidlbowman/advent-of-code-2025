use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input/day09.txt").expect("Failed to read input");

    let points: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();

    let n = points.len();
    let mut max_area: i64 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let width = (points[j].0 - points[i].0).abs() + 1;
            let height = (points[j].1 - points[i].1).abs() + 1;
            let area = width * height;
            max_area = max_area.max(area);
        }
    }

    println!("{}", max_area);
}
