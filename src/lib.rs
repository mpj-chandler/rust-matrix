pub mod matrix_algebra {
    use std::fmt;
    use std::ops::Add;

    #[derive(Clone)]
    pub struct Matrix {
        pub n: usize,
        pub m: usize,
        pub entries: Vec<i32>,
    }

    impl Matrix {
        pub fn new(m: usize, n: usize, entries: Vec<i32>) -> Matrix {
            Matrix { m, n, entries }
        }

        pub fn new_constant_value(m: usize, n: usize, value: i32) -> Matrix {
            if m == 0 || n == 0 {
                panic!("Matrix dimensions must each be greater than zero!");
            }
          
            Matrix {
                n: n,
                m: m,
                entries: vec![value; n * m],
            }
        }

        pub fn new_all_zeroes(m: usize, n: usize) -> Matrix {
            Matrix::new_constant_value(m, n, 0)
        }

        pub fn get_entry_ij(&self, i: usize, j: usize) -> i32 {
        	self.entries[(i * self.n) + j]
        }


        pub fn set_entry_ij(&mut self, i: usize, j: usize, new_value: i32) {
        	self.entries[(i * self.n) + j] = new_value;
        }

        pub fn rows(&self) -> Vec<Vec<i32>> {
        	// We want to split the vec into m equal parts of length n
        	let mut rows = Vec::new();

        	for i in (0..(self.m * self.n)).step_by(self.n) {
        		rows.push(self.entries[i * self.n..(i + 1) * self.n].to_vec());
        	}

        	rows
        }

        pub fn columns(&self) -> Vec<Vec<i32>> {
            let mut columns = Vec::new();
            
            for _i in 0..self.n {
            	let mut new_column: Vec<i32> = Vec::new();
                for j in (0..(self.n * self.m)).step_by(self.m) {
                    new_column.push(self.entries[j]);
                }
                columns.push(new_column);
            }

            columns
        }
    }

    impl fmt::Display for Matrix {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        	let rows = self.clone().rows();
            for row in rows {
                let printable_row: String =
                    row.iter().map(|&entry| entry.to_string() + ", ").collect();
                let _ = fmt.write_str(&printable_row)?;
                let _ = fmt.write_str("\n");
            }
            Ok(())
        }
    }

    impl Add for Matrix {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            if !is_additively_conformable(self.clone(), other.clone()) {
                panic!("Matrices are not additively conformable!")
            }

            let mut sum_matrix = Matrix::new_constant_value(self.m, self.n, 0);

            for i in 0..self.m {
                for j in 0..self.n {
                    sum_matrix.set_entry_ij(i, j, &self.get_entry_ij(i, j) + &other.get_entry_ij(i, j));
                }
            }
            sum_matrix
        }
    }

    fn is_multiplicatively_conformable(a: Matrix, b: Matrix) -> bool {
        a.n == b.m
    }

    fn is_additively_conformable(a: Matrix, b: Matrix) -> bool {
        a.n == b.n && a.m == b.m
    }

    pub fn matrix_multiply(a: Matrix, b: Matrix) -> Matrix {
        if !is_multiplicatively_conformable(a.clone(), b.clone()) {
            panic!("Matrices are not multiplicatively conformable!");
        }

        let mut c = Matrix::new_constant_value(b.n, a.m, 0);
        let a_rows = a.rows();
        let b_columns = b.columns();
        let mut i = 0;

        for a_row in a_rows {
        	for a_ik in a_row {
        		let mut j = 0;
        		for b_kj in &b_columns[i] {
                    c.set_entry_ij(i, j, a_ik * b_kj);
                    j += 1;
                }
            }
            i += 1;
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix_algebra::matrix_multiply;
    use crate::matrix_algebra::Matrix;
    use std::ops::Add;

    use rand::prelude::*;

    #[test]
    fn test_constant_value_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let value = rng.gen_range(1..=100);
        let test_matrix = Matrix::new_constant_value(m, n, value);

        assert_eq!(test_matrix.entries.len(), n * m);

        for entry in test_matrix.entries {
            assert_eq!(entry, value);
        }
    }

    #[test]
    fn test_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let mut entries: Vec<Vec<i32>> = vec![];

        for _i in 0..m {
            let mut row: Vec<i32> = vec![];
            for _j in 0..n {
                row.push(rng.gen_range(1..=100));
            }
            entries.push(row);
        }

        let test_matrix = Matrix { m, n, entries };

        assert_eq!(test_matrix.entries.len(), m);

        for row in test_matrix.entries {
            assert_eq!(row.len(), n);
        }
    }

    #[test]
    fn test_all_zeroes_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let test_matrix = Matrix::new_all_zeroes(m, n);

        assert_eq!(test_matrix.entries.len(), m);

        for row in test_matrix.entries {
            assert_eq!(row.len(), n);
            for entry in row {
                assert_eq!(entry, 0);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_zero_first_argument_to_initialiser() {
        let _test_matrix = Matrix::new_constant_value(0, 1, 1);
    }

    #[test]
    #[should_panic]
    fn test_zero_second_argument_to_initialiser() {
        let _test_matrix = Matrix::new_constant_value(1, 0, 1);
    }

    #[test]
    #[should_panic]
    fn test_panic_on_non_multiplicatively_conformable_matrices() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, 5);
        let test_matrix_b = Matrix::new_constant_value(5, 7, 4);

        matrix_multiply(test_matrix_a, test_matrix_b);
    }

    #[test]
    #[should_panic]
    fn test_panic_on_non_additively_conformable_matrices() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, 5);
        let test_matrix_b = Matrix::new_constant_value(5, 7, 4);

        let _ = test_matrix_a.add(test_matrix_b);
    }

    #[test]
    fn test_matrix_add() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, 5);
        let test_matrix_b = Matrix::new_constant_value(3, 4, 4);

        let matrix_sum = test_matrix_a.add(test_matrix_b);

        assert_eq!(matrix_sum.entries.len(), 3);

        assert_eq!(matrix_sum.entries[0], [9, 9, 9, 9]);
        assert_eq!(matrix_sum.entries[1], [9, 9, 9, 9]);
        assert_eq!(matrix_sum.entries[2], [9, 9, 9, 9]);
    }

    #[test]
    fn test_matrix_multiply() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, 5);
        let test_matrix_b = Matrix::new_constant_value(4, 3, 4);

        let matrix_product = matrix_multiply(test_matrix_a, test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 3);

        assert_eq!(matrix_product.entries[0], [20, 20, 20]);
        assert_eq!(matrix_product.entries[1], [20, 20, 20]);
        assert_eq!(matrix_product.entries[2], [20, 20, 20]);
    }
}
