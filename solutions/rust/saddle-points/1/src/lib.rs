pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {

    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    if input[0].is_empty() { return saddle_points; }

    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut mins: Vec<u64> = Vec::new();
    for j in 0..num_cols {
        let mut min_cols: Vec<u64> = Vec::new();
        for i in 0..num_rows {
            min_cols.push(input[i][j]);
        }
        mins.push(*min_cols.iter().min().unwrap());
    }

    for (index, row) in input.iter().enumerate() {
        let max = row.iter().max().unwrap();

        let it = row.iter().enumerate()
                    .filter(|&x| x.1 == max && x.1 == &mins[x.0])
                    .map(|x| x.0);
                
        for col in it {
            saddle_points.push((index, col));
        }
    }

    saddle_points
}