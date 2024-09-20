use matrix::matrix_algebra::Matrix;

fn main() {
    let test_matrix = Matrix::new_constant_value(4, 4, 5);

    println!("{test_matrix}");

    let test_matrix_a = Matrix::new(
        3,
        4,
        [
            1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 1.10, 1.11, 1.12,
        ]
        .to_vec(),
    );
    let test_matrix_b = Matrix::new(4, 3, [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_vec());

    let c = test_matrix_a * test_matrix_b;

    println!("{}", c);
}
