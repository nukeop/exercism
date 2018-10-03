pub fn is_greatest_in_row(x: usize, y: usize, matrix: &[Vec<u64>]) -> bool {
    let row = &matrix[y];
    
    let max = row.iter().max().unwrap();
    *max == matrix[y][x]
}

pub fn is_lowest_in_column(x: usize, y: usize, matrix: &[Vec<u64>]) -> bool {
    let mut col: Vec<u64> = Vec::new();

    for i in 0..matrix.len() {
        col.push(matrix[i][x]);
    }

    let min = col.iter().min().unwrap();
    *min == matrix[y][x]
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if is_greatest_in_row(x, y, input) &&
                is_lowest_in_column(x, y, input) {
                    result.push((y, x));
                }
        }
    }

    return result;
}
