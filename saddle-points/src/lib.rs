pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points = vec![];
    for (row, row_vec) in input.iter().enumerate() {
        for (column, value) in row_vec.iter().enumerate() {
            if row_vec.iter().all(|x| x <= value) && input.iter().all(|x| x[column] >= *value) {
                points.push((row, column));
            }
        }
    }
    points
}
