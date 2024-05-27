pub mod matrix_algebra {
	use std::fmt;

	pub struct Matrix {
		pub entries: Vec<Vec<i32>>,
	}

	impl Matrix {
		pub fn new(m: usize, n: usize, value: i32) -> Matrix {
			let row_vec = vec![value; n];
			Matrix {
				entries: vec![row_vec; m],
			}
		}
	}

	impl fmt::Display for Matrix {
		fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
			for row in &self.entries {
				let printable_row: String = row.iter().map( |&entry| entry.to_string() + ", ").collect();
				let _ = fmt.write_str(&printable_row)?;
				let _ = fmt.write_str("\n");
			}
			Ok(())
		}
	}

	fn is_multiplicatively_conformable(a: Matrix, b: Matrix) -> bool {
		let n = a.entries[0].len();
		let n_prime = b.entries.len();

		n == n_prime
	}

	pub fn matrix_multiply(a: Matrix, b: Matrix) -> Matrix {
		if !is_multiplicatively_conformable(a, b) {
			panic!("Matrices are not multiplicatively conformable!");
		}
		Matrix::new(4, 4, 0)
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix_algebra::matrix_multiply;
	use crate::matrix_algebra::Matrix;

	use rand::prelude::*;

	#[test]
	fn test_initialiser() {
		let mut rng = rand::thread_rng();
		let n = rng.gen_range(1..=100);
		let m = rng.gen_range(1..=100);
		let test_matrix = Matrix::new(m, n, 0);

    	assert_eq!(test_matrix.entries.len(), m);

    	for row in test_matrix.entries {
    		assert_eq!(row.len(),n);
    	}
	}

	#[test]
	#[should_panic]
	fn test_panic_on_non_multiplicatively_conformable_matrices() {
		let test_matrix_a = Matrix::new(3, 4, 5);
		let test_matrix_b = Matrix::new(5, 7, 4);

		matrix_multiply(test_matrix_a, test_matrix_b);
	}

	#[test]
	fn test_matrix_multiply() {
		let test_matrix_a = Matrix::new(3, 4, 5);
		let test_matrix_b = Matrix::new(5, 3, 4);

		let matrix_product = matrix_multiply(test_matrix_a, test_matrix_b);

		assert_eq!(matrix_product.entries.len(), 7);
	}
}
