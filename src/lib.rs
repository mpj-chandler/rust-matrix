use std::fmt;

pub struct Matrix {
	n: i32,
	m: i32,
	entries: Vec<i32>,
}

impl Matrix {
	pub fn new(n: i32, m: i32, entries: Vec<i32>) -> Matrix {
		Matrix {
			n,
			m,
			entries,
		}
	}
}

impl fmt::Display for Matrix {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		for row_index in 0..self.n {
			let start_index = (row_index * self.m) as usize;
			let end_index = (((row_index + 1) * self.m) - 1) as usize;
			// let _ = fmt.write_str(&start_index.to_string())?;
			// let _ = fmt.write_str(&end_index.to_string())?;

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
		let test_matrix = Matrix::new(4, 4, vec![1,16]);

    	assert_eq!(test_matrix.entries.len(), 16);
	}
}
