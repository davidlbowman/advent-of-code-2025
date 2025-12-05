use std::fs::read_to_string;

fn line_to_bits(line: &str) -> Vec<u64> {
    let num_chunks = line.len().div_ceil(64);
    let mut chunks = vec![0u64; num_chunks];
    for (i, ch) in line.chars().enumerate() {
        if ch == '@' {
            let chunk_idx = i / 64;
            let bit_idx = i % 64;
            chunks[chunk_idx] |= 1u64 << bit_idx;
        }
    }
    chunks
}

fn get_bit(chunks: &[u64], i: usize) -> u64 {
    let chunk_idx = i / 64;
    let bit_idx = i % 64;
    if chunk_idx < chunks.len() {
        (chunks[chunk_idx] >> bit_idx) & 1
    } else {
        0
    }
}

fn shift_right(chunks: &[u64]) -> Vec<u64> {
    let mut result = vec![0u64; chunks.len()];
    for i in 0..chunks.len() {
        result[i] = chunks[i] >> 1;
        if i + 1 < chunks.len() {
            result[i] |= (chunks[i + 1] & 1) << 63;
        }
    }
    result
}

fn shift_left(chunks: &[u64], width: usize) -> Vec<u64> {
    let mut result = vec![0u64; chunks.len()];
    for i in 0..chunks.len() {
        result[i] = chunks[i] << 1;
        if i > 0 {
            result[i] |= chunks[i - 1] >> 63;
        }
    }
    let last_chunk = (width - 1) / 64;
    let last_bit = (width - 1) % 64;
    if last_chunk < result.len() {
        result[last_chunk] &= (1u64 << (last_bit + 1)) - 1;
    }
    result
}

fn expand_to_4bit(chunks: &[u64], width: usize) -> Vec<u64> {
    let num_u64s = (width * 4).div_ceil(64);
    let mut result = vec![0u64; num_u64s];

    for i in 0..width {
        let bit = get_bit(chunks, i);
        let dest_bit = i * 4;
        let dest_idx = dest_bit / 64;
        let dest_offset = dest_bit % 64;
        result[dest_idx] |= bit << dest_offset;
    }
    result
}

fn add_packed_4bit(a: &[u64], b: &[u64]) -> Vec<u64> {
    const MASK_LOW: u64 = 0x0F0F_0F0F_0F0F_0F0F;

    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let low = (x & MASK_LOW) + (y & MASK_LOW);
            let high = ((x >> 4) & MASK_LOW) + ((y >> 4) & MASK_LOW);
            (low & MASK_LOW) | ((high & MASK_LOW) << 4)
        })
        .collect()
}

fn count_accessible(counts: &[u64], original: &[u64], width: usize) -> i32 {
    let mut total = 0;
    for i in 0..width {
        if get_bit(original, i) == 0 {
            continue;
        }
        let bit_pos = i * 4;
        let idx = bit_pos / 64;
        let offset = bit_pos % 64;
        let count = (counts[idx] >> offset) & 0xF;
        if count < 4 {
            total += 1;
        }
    }
    total
}

fn main() {
    let input = read_to_string("src/input/day04.txt").expect("Failed to read input");
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let num_chunks = width.div_ceil(64);

    let rows: Vec<Vec<u64>> = lines.iter().map(|l| line_to_bits(l)).collect();
    let empty_row = vec![0u64; num_chunks];

    let mut total_count: i32 = 0;

    for row_idx in 0..height {
        let current = &rows[row_idx];
        let above = if row_idx > 0 {
            &rows[row_idx - 1]
        } else {
            &empty_row
        };
        let below = if row_idx < height - 1 {
            &rows[row_idx + 1]
        } else {
            &empty_row
        };

        let neighbors: [Vec<u64>; 8] = [
            shift_right(above),
            above.clone(),
            shift_left(above, width),
            shift_right(current),
            shift_left(current, width),
            shift_right(below),
            below.clone(),
            shift_left(below, width),
        ];

        let mut sum = vec![0u64; (width * 4).div_ceil(64)];
        for neighbor_mask in &neighbors {
            let expanded = expand_to_4bit(neighbor_mask, width);
            sum = add_packed_4bit(&sum, &expanded);
        }

        total_count += count_accessible(&sum, current, width);
    }

    println!("{}", total_count);
}
