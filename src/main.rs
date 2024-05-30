use matrix::Matrix;

fn main() {
    // let test_matrix = Matrix::new(4, 4, vec![1; 16]);
    let test_matrix = Matrix::new_identity_matrix(4);
    println!("{test_matrix}");
}
