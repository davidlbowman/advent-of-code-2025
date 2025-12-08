use std::fs::read_to_string;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        true
    }
}

fn main() {
    let input = read_to_string("src/input/day08.txt").expect("Failed to read input");

    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .collect();

    let n = points.len();

    let mut distances: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[j].0 - points[i].0;
            let dy = points[j].1 - points[i].1;
            let dz = points[j].2 - points[i].2;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            distances.push((dist_sq, i, j));
        }
    }

    distances.sort_by_key(|&(dist, _, _)| dist);

    let mut uf = UnionFind::new(n);
    let mut edges_added = 0;
    let mut last_pair: (usize, usize) = (0, 0);

    for &(_, i, j) in &distances {
        if uf.union(i, j) {
            edges_added += 1;
            last_pair = (i, j);

            if edges_added == n - 1 {
                break;
            }
        }
    }

    let result = points[last_pair.0].0 * points[last_pair.1].0;
    println!("{}", result);
}
