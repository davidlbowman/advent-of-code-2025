use std::fs::read_to_string;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            self.rank[root_x] += 1;
        }
    }

    fn set_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
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

    for &(_, i, j) in distances.iter().take(1000) {
        uf.union(i, j);
    }

    let mut circuit_sizes: Vec<usize> = Vec::new();
    let mut seen_roots: Vec<bool> = vec![false; n];

    for i in 0..n {
        let root = uf.find(i);
        if !seen_roots[root] {
            seen_roots[root] = true;
            circuit_sizes.push(uf.set_size(i));
        }
    }

    circuit_sizes.sort_by(|a, b| b.cmp(a));

    let result: usize = circuit_sizes.iter().take(3).product();
    println!("{}", result);
}
