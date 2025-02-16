use matrix::{complex_number::ComplexNumber, matrix_algebra::Matrix};

fn main() {
    let mut test_matrix = Matrix::new(2, 3, [1.0, 2.0, -1.0, 0.0, 3.0, 7.0].to_vec());
    let entry = test_matrix.get_entry_ij(1, 1);
    println!("{entry}");
    test_matrix.set_entry_ij(1, 1, &4.0);
    let entry = test_matrix.get_entry_ij(1, 1);
    println!("{entry}");

    let test_matrix_a = Matrix::new(
        3,
        4,
        [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ]
        .to_vec(),
    );
    println!("{}", test_matrix_a);

    let complex_number = ComplexNumber::new(5.4, -10.0);

    println!("{}", complex_number * complex_number);

    let test_matrix_d = Matrix::new(
        4,
        2,
        [
            complex_number,
            complex_number,
            complex_number,
            complex_number,
            complex_number,
            complex_number,
            complex_number,
            complex_number,
        ]
        .to_vec(),
    );

    let test_matrix_e = Matrix::new(
        2,
        4,
        [
            complex_number,
            complex_number,
            complex_number,
            complex_number,
            complex_number,
            complex_number,
            complex_number,
            complex_number,
        ]
        .to_vec(),
    );

    println!("{}", test_matrix_d * test_matrix_e);
}
