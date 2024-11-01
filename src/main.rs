use matrix::matrix_algebra::Matrix;

fn main() {
    let test_matrix = Matrix::new(2, 3, [1, 2, -1, 0, 3, 7].to_vec());

    println!("{test_matrix}");

    let test_matrix_a = Matrix::new(
        3,
        4,
        [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ]
        .to_vec(),
    );
    let test_matrix_b = Matrix::new(
        4,
        3,
        [
            12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ]
        .to_vec(),
    );

    let c = test_matrix_a * test_matrix_b;

    println!("{}", c);
}
