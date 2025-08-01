#[cfg(test)]
mod tests;

pub type MatrixOfDoubles = Vec<Vec<f64>>;

pub fn init_matrix(matrix: &mut MatrixOfDoubles) {
    let size = matrix.len();

    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = ((i + j) % 1024) as f64
        }
    }
}

pub fn solution(matrix_in: &MatrixOfDoubles, matrix_out: &mut MatrixOfDoubles) {
    let size = matrix_in.len();
    let tile_size = 16;
    for ii in (0..size).step_by(tile_size){
        for jj in (0..size).step_by(tile_size){
            for i in ii..std::cmp::min(ii + tile_size, size) {
                for j in jj..std::cmp::min(jj + tile_size, size){
                    matrix_out[i][j] = matrix_in[j][i];
                }
            }
        }
    }

}
