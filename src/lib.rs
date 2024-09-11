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
            let mut rows = Vec::new();

            for i in (0..(self.m * self.n)).step_by(self.n) {
                rows.push(self.entries[i..(i + self.n)].to_vec());
            }

            rows
        }

        pub fn columns(&self) -> Vec<Vec<i32>> {
            let mut columns = Vec::new();

            for i in 0..self.n {
                let mut new_column: Vec<i32> = Vec::new();
                for j in 0..self.m {
                    new_column.push(self.entries[(self.n * j) + i]);
                }
                columns.push(new_column.to_vec());
            }

            columns
        }
    }

    impl fmt::Display for Matrix {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            let rows = self.rows();
            for (i, row) in rows.clone().into_iter().enumerate() {
                let printable_row: String =
                    row.iter().map(|&entry| entry.to_string() + " ").collect();
                if i == 0 {
                    let _ = fmt.write_str("⌜");
                } else if i == &rows.len() - 1 {
                    let _ = fmt.write_str("⌞");
                } else {
                    let _ = fmt.write_str("⎸");
                }

                let _ = fmt.write_str(&printable_row)?;
                if i == 0 {
                    let _ = fmt.write_str("⌝");
                } else if i == rows.len() - 1 {
                    let _ = fmt.write_str("⌟");
                } else {
                    let _ = fmt.write_str("⎹");
                }
                let _ = fmt.write_str("\n");
            }
            Ok(())
        }
    }

    impl Add for Matrix {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            if !is_additively_conformable(&self, &other) {
                panic!("Matrices are not additively conformable!")
            }

            let mut sum_matrix = Matrix::new_constant_value(self.m, self.n, 0);

            for i in 0..self.m {
                for j in 0..self.n {
                    sum_matrix.set_entry_ij(
                        i,
                        j,
                        &self.get_entry_ij(i, j) + &other.get_entry_ij(i, j),
                    );
                }
            }
            sum_matrix
        }
    }

    fn is_multiplicatively_conformable(a: &Matrix, b: &Matrix) -> bool {
        a.n == b.m
    }

    fn is_additively_conformable(a: &Matrix, b: &Matrix) -> bool {
        a.n == b.n && a.m == b.m
    }

    pub fn matrix_multiply(a: Matrix, b: Matrix) -> Matrix {
        if !is_multiplicatively_conformable(&a, &b) {
            panic!("Matrices are not multiplicatively conformable!");
        }

        let mut entries: Vec<i32> = Vec::new();
        let a_rows = a.rows();
        let b_columns = b.columns();
        let _i = 0;

        for i in 0..a_rows.len() {
            for j in 0..b_columns.len() {
                if a_rows[i].len() != b_columns[j].len() {
                    panic!("Row / Column length mismatch - cannot calculate matrix product!")
                }
                let mut new_entry: i32 = 0;
                for k in 0..a_rows[i].len() {
                    new_entry += a_rows[i][k] * b_columns[j][k];
                }
                entries.push(new_entry);
            }
        }

        Matrix {
            n: a.n,
            m: b.m,
            entries,
        }
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
        let mut entries: Vec<i32> = Vec::new();

        for _i in 0..m {
            for _j in 0..n {
                entries.push(rng.gen_range(1..=100));
            }
        }

        let test_matrix = Matrix { m, n, entries };

        assert_eq!(test_matrix.entries.len(), n * m);
    }

    #[test]
    fn test_all_zeroes_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let test_matrix = Matrix::new_all_zeroes(m, n);

        assert_eq!(test_matrix.entries.len(), n * m);

        for entry in test_matrix.entries {
            assert_eq!(entry, 0);
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
        let test_matrix_a = Matrix::new(3, 4, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec());
        let test_matrix_b = Matrix::new(3, 4, [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_vec());

        let matrix_sum = test_matrix_a.add(test_matrix_b);

        assert_eq!(matrix_sum.entries.len(), 12);
        assert_eq!(
            matrix_sum.entries,
            [13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13]
        );
    }

    #[test]
    fn test_columns() {
        let test_matrix = Matrix::new(3, 4, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec());

        let columns = test_matrix.columns();

        assert_eq!(columns.len(), 4);

        assert_eq!(columns[0], vec![1, 5, 9]);
        assert_eq!(columns[1], vec![2, 6, 10]);
        assert_eq!(columns[2], vec![3, 7, 11]);
        assert_eq!(columns[3], vec![4, 8, 12]);
    }

    #[test]
    fn test_rows() {
        let test_matrix = Matrix::new(3, 4, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec());

        let rows = test_matrix.rows();

        assert_eq!(rows.len(), 3);

        assert_eq!(rows[0], vec![1, 2, 3, 4]);
        assert_eq!(rows[1], vec![5, 6, 7, 8]);
        assert_eq!(rows[2], vec![9, 10, 11, 12]);
    }

    #[test]
    fn test_matrix_multiply() {
        let test_matrix_a = Matrix::new(3, 4, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec());
        let test_matrix_b = Matrix::new(4, 3, [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_vec());

        let matrix_product = matrix_multiply(test_matrix_a, test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [
                (1 * 12) + (2 * 9) + (3 * 6) + (4 * 3),
                (1 * 11) + (2 * 8) + (3 * 5) + (4 * 2),
                (1 * 10) + (2 * 7) + (3 * 4) + (4 * 1),
                (5 * 12) + (6 * 9) + (7 * 6) + (8 * 3),
                (5 * 11) + (6 * 8) + (7 * 5) + (8 * 2),
                (5 * 10) + (6 * 7) + (7 * 4) + (8 * 1),
                (9 * 12) + (10 * 9) + (11 * 6) + (12 * 3),
                (9 * 11) + (10 * 8) + (11 * 5) + (12 * 2),
                (9 * 10) + (10 * 7) + (11 * 4) + (12 * 1)
            ]
        );
    }

    #[test]
    fn test_matrix_multiply_constant_value_initialiser() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, 5);
        let test_matrix_b = Matrix::new_constant_value(4, 3, 4);

        let matrix_product = matrix_multiply(test_matrix_a, test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(matrix_product.entries, [80, 80, 80, 80, 80, 80, 80, 80, 80]);
    }
}
