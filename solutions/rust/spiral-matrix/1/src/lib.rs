pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut v = vec![vec![0u32; size as usize]; size as usize];

    if size == 0 {
        return v;
    }
    if size == 1 {
        return vec![vec![1]];
    }

    let last = size * size;
    let mut count = 1u32;
    let mut start = 0usize;
    let mut end = size as usize - 1;

    while count <= last {
        // Move right (do nothing to start or end)
        for col in start..=end {
            v[start][col] = count;
            count += 1;
        }

        // Move down (increase start by 1)
        start += 1;
        for row in start..=end {
            v[row][end] = count;
            count += 1;
        }

        // Move left (decrease start by 1 and decrease end by 1)
        start -= 1;
        end -= 1;
        for col in (start..=end).rev() {
            v[end + 1][col] = count;
            count += 1;
        }

        // Move up (increase start by 1)
        start += 1;
        for row in (start..=end).rev() {
            v[row][start - 1] = count;
            count += 1;
        }
    }

    v
}
