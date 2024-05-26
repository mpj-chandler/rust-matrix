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
			fmt.write_str(&printable_row)?;
			fmt.write_str("\n");
		}
		Ok(())
	}
}
