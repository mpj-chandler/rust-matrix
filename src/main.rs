use matrix_algebra::{Entry, Matrix};

fn main() {
    let test_matrix = Matrix::new_constant_value(4, 4, Entry::Integer32(5));

    println!("{test_matrix}");

    let test_matrix_a = Matrix::new(
        3,
        4,
        [
            Entry::Integer32(1),
            Entry::Integer32(2),
            Entry::Integer32(3),
            Entry::Integer32(4),
            Entry::Integer32(5),
            Entry::Integer32(6),
            Entry::Integer32(7),
            Entry::Integer32(8),
            Entry::Integer32(9),
            Entry::Integer32(10),
            Entry::Integer32(11),
            Entry::Integer32(12),
        ]
        .to_vec(),
    );
    let test_matrix_b = Matrix::new(
        4,
        3,
        [
            Entry::Float64(12.0),
            Entry::Float64(11.0),
            Entry::Float64(10.0),
            Entry::Float64(9.0),
            Entry::Float64(8.0),
            Entry::Float64(7.0),
            Entry::Float64(6.0),
            Entry::Float64(5.0),
            Entry::Float64(4.0),
            Entry::Float64(3.0),
            Entry::Float64(2.0),
            Entry::Float64(1.0),
        ]
        .to_vec(),
    );

    let c = test_matrix_a * test_matrix_b;

    println!("{}", c);
}
