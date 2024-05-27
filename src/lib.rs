pub mod matrix_algebra {
	use std::fmt;

	pub struct Matrix {
		pub entries: Vec<Vec<i32>>,
	}

	impl Matrix {
		pub fn new(n: usize, m: usize, value: i32) -> Matrix {
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

	pub fn matrix_multiply(_a: Matrix, _b: Matrix) -> Matrix {
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
		let test_matrix = Matrix::new(n, m, 0);

    	assert_eq!(test_matrix.entries.len(), m);

    	for row in test_matrix.entries {
    		assert_eq!(row.len(),n);
    	}
	}

	#[test]
	fn test_matrix_multiply() {
		let test_matrix_a = Matrix::new(3, 4, 5);
		let test_matrix_b = Matrix::new(5, 3, 4);

		let matrix_product = matrix_multiply(test_matrix_a, test_matrix_b);

		assert_eq!(matrix_product.entries.len(), 7);
	}
}
