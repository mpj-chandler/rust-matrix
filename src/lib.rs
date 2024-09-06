use std::fmt;

pub struct Matrix {
	n: i32,
	m: i32,
	entries: Vec<i32>,
}

impl Matrix {
	pub fn new(n: i32, m: i32, entries: Vec<i32>) -> Matrix {
		if entries.len() != (n * m).try_into().unwrap() {
			panic!("Matrix incorrectly specified! Number of entries should equal n * m");
		}

		Matrix {
			n,
			m,
			entries,
		}
	}

	pub fn new_identity_matrix(n: i32) -> Matrix {
		let mut entries: Vec<i32> = Vec::new();
		for row_index in 0..n {
			let mut new_row = vec![0; n as usize];
			new_row[row_index as usize] = 1;
			for entry in new_row {
				entries.push(entry);
			}
		}

		Matrix {
			n,
			m: n,
			entries,
		}
	}
}

impl fmt::Display for Matrix {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		for row_index in 0..self.n {
			let start_index: usize = (row_index * self.m).try_into().unwrap();
			let end_index: usize = (((row_index + 1) * self.m) - 1).try_into().unwrap();
			let printable_row: String = self.entries[start_index..=end_index].iter().map( |&entry| entry.to_string() + ", ").collect();
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
		let test_matrix = Matrix::new(4, 4, vec![1; 16]);

    	assert_eq!(test_matrix.entries.len(), 16);
	}

	#[test]
	#[should_panic]
	fn test_initialiser_panic() {
		let _ = Matrix::new(4, 6, vec![1; 16]);
	}
}
