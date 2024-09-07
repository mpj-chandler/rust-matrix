use matrix::matrix_algebra::Matrix;

fn main() {
    let test_matrix = Matrix::new_constant_value(4, 4, 5);

    println!("{test_matrix}");
}
