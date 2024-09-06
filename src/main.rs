use matrix::matrix_algebra::Matrix;

fn main() {
<<<<<<< HEAD
    let test_matrix = Matrix::new(4, 4, 5);

=======
    // let test_matrix = Matrix::new(4, 4, vec![1; 16]);
    let test_matrix = Matrix::new_identity_matrix(4);
>>>>>>> main
    println!("{test_matrix}");
}
