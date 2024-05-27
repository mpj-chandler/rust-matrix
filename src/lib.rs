use std::fmt;

pub struct Matrix {
	entries: Vec<Vec<i32>>,
}

impl Matrix {
	pub fn new(n: usize, m: usize) -> Matrix {
		let row_vec = vec![0; n];
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_initialiser() {
		let test_matrix = Matrix::new(4, 4);

    	assert_eq!(test_matrix.entries.len(), 4);

    	for row in test_matrix.entries {
    		assert_eq!(row.len(), 4);
    	}
	}
}
