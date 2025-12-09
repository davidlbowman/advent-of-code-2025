use std::collections::HashMap;
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

    let mut h_segments: Vec<(i64, i64, i64)> = Vec::new();
    let mut v_segments: Vec<(i64, i64, i64)> = Vec::new();

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        if y1 == y2 {
            let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            h_segments.push((y1, min_x, max_x));
        } else if x1 == x2 {
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            v_segments.push((x1, min_y, max_y));
        }
    }

    let mut h_by_y: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    for (y, x_min, x_max) in &h_segments {
        h_by_y.entry(*y).or_default().push((*x_min, *x_max));
    }

    let mut v_by_x: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    for (x, y_min, y_max) in &v_segments {
        v_by_x.entry(*x).or_default().push((*y_min, *y_max));
    }

    let is_rect_valid = |x_min: i64, x_max: i64, y_min: i64, y_max: i64| -> bool {
        let test_points = [
            (x_min, y_min),
            (x_min, y_max),
            (x_max, y_min),
            (x_max, y_max),
            ((x_min + x_max) / 2, (y_min + y_max) / 2),
            (x_min, (y_min + y_max) / 2),
            (x_max, (y_min + y_max) / 2),
            ((x_min + x_max) / 2, y_min),
            ((x_min + x_max) / 2, y_max),
        ];

        for (x, y) in test_points {
            let mut on_boundary = false;
            if let Some(segs) = h_by_y.get(&y) {
                for (sx_min, sx_max) in segs {
                    if x >= *sx_min && x <= *sx_max {
                        on_boundary = true;
                        break;
                    }
                }
            }
            if !on_boundary && let Some(segs) = v_by_x.get(&x) {
                for (sy_min, sy_max) in segs {
                    if y >= *sy_min && y <= *sy_max {
                        on_boundary = true;
                        break;
                    }
                }
            }

            if !on_boundary {
                let mut crossings = 0;
                for (vx, vy_min, vy_max) in &v_segments {
                    if *vx < x && y >= *vy_min && y < *vy_max {
                        crossings += 1;
                    }
                }
                if crossings % 2 == 0 {
                    return false;
                }
            }
        }

        for (vx, vy_min, vy_max) in &v_segments {
            if *vx > x_min && *vx < x_max && *vy_min < y_max && *vy_max > y_min {
                let test_y = (*vy_min.max(&y_min) + *vy_max.min(&y_max)) / 2;

                let mut left_crossings = 0;
                for (vx2, vy2_min, vy2_max) in &v_segments {
                    if *vx2 < vx - 1 && test_y >= *vy2_min && test_y < *vy2_max {
                        left_crossings += 1;
                    }
                }

                if left_crossings % 2 == 0 {
                    return false;
                }
            }
        }

        for (hy, hx_min, hx_max) in &h_segments {
            if *hy > y_min && *hy < y_max && *hx_min < x_max && *hx_max > x_min {
                let test_x = (*hx_min.max(&x_min) + *hx_max.min(&x_max)) / 2;

                let mut below_crossings = 0;
                for (vx, vy_min, vy_max) in &v_segments {
                    if *vx < test_x && (hy - 1) >= *vy_min && (hy - 1) < *vy_max {
                        below_crossings += 1;
                    }
                }

                if below_crossings % 2 == 0 {
                    return false;
                }
            }
        }

        true
    };

    let mut max_area: i64 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;
            let area = width * height;

            if area <= max_area {
                continue;
            }

            if is_rect_valid(min_x, max_x, min_y, max_y) {
                max_area = area;
            }
        }
    }

    println!("{}", max_area);
}
