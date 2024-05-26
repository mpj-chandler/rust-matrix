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

	pub fn print(&self) {
		for row in &self.entries {
			for element in row {
				println!("{element}");
			}
		}
	}
}