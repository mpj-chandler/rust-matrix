pub mod matrix_algebra {
    use std::fmt;
    use std::fmt::Display;
    use std::ops::{Add, AddAssign, Mul};

    #[derive(Clone)]
    pub struct Matrix<T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized> {
        pub n: usize,
        pub m: usize,
        pub entries: Vec<T>,
    }

    impl<
            T: Add<Output = T> + Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized,
        > Matrix<T>
    {
        pub fn new(m: usize, n: usize, entries: Vec<T>) -> Matrix<T> {
            Matrix { m, n, entries }
        }

        pub fn new_constant_value(m: usize, n: usize, value: T) -> Matrix<T> {
            if m == 0 || n == 0 {
                panic!("Matrix dimensions must each be greater than zero!");
            }

            Matrix {
                n,
                m,
                entries: vec![value; n * m],
            }
        }

        pub fn get_entry_ij(&self, i: usize, j: usize) -> &T {
            &self.entries[(i * self.n) + j]
        }

        pub fn set_entry_ij(&mut self, i: usize, j: usize, new_value: &T) {
            self.entries[(i * self.n) + j] = new_value.clone();
        }

        pub fn rows(&self) -> Vec<Vec<T>> {
            let mut rows = Vec::new();

            for i in (0..(self.m * self.n)).step_by(self.n) {
                rows.push(self.entries[i..(i + self.n)].to_vec());
            }

            rows
        }

        pub fn columns(&self) -> Vec<Vec<T>> {
            let mut columns = Vec::new();

            for i in 0..self.n {
                let mut new_column: Vec<T> = Vec::new();
                for j in 0..self.m {
                    new_column.push(self.entries[(self.n * j) + i].clone());
                }
                columns.push(new_column.to_vec());
            }

            columns
        }

        pub fn transpose(&self) -> Matrix<T> {
            let columns = self.columns();
            let mut transposed_entries = Vec::new();

            for column in columns {
                for entry in column {
                    println!("{}", entry);
                    transposed_entries.push(entry);
                }
                println!(".");
            }

            Matrix::<T> {
                m: self.n,
                n: self.m,
                entries: transposed_entries,
            }
        }

        pub fn submatrix(&self, row_indices: &[usize], column_indices: &[usize]) -> Matrix<T> {
            let rows = self.rows();
            let mut new_entries: Vec<T> = Vec::new();

            for row_index in row_indices {
                for column_index in column_indices {
                    let new_entry = &rows[*row_index][*column_index];

                    new_entries.push(*new_entry);
                }
            }

            Matrix {
                n: column_indices.len(),
                m: row_indices.len(),
                entries: new_entries,
            }
        }

        pub fn partition(
            &self,
            row_partitioning: &[usize],
            column_partitioning: &[usize],
        ) -> Vec<Matrix<T>> {
            [self.clone()].to_vec()
        }
    }

    impl<T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul<Output = T> + ?Sized>
        fmt::Display for Matrix<T>
    {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            let rows = self.rows();
            for (i, row) in rows.clone().into_iter().enumerate() {
                let printable_row: String = row
                    .iter()
                    .map(|&entry| {
                        let mut entry_as_string = entry.to_string();

                        while entry_as_string.len() < 4 {
                            entry_as_string += " ";
                        }

                        entry_as_string + " "
                    })
                    .collect();
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

    pub fn matrix_add<T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized>(
        a: &Matrix<T>,
        b: &Matrix<T>,
    ) -> Matrix<T>
    where
        for<'a> &'a T: Add<Output = T>,
    {
        if !is_additively_conformable(a, b) {
            panic!("Matrices are not additively conformable!")
        }
        let mut entries: Vec<T> = Vec::new();

        for i in 0..a.m {
            for j in 0..a.n {
                let new_value = a.get_entry_ij(i, j) + b.get_entry_ij(i, j);
                entries.push(new_value);
            }
        }

        Matrix::<T> {
            n: a.m,
            m: a.n,
            entries,
        }
    }

    impl<T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized> Add for Matrix<T>
    where
        for<'a> &'a T: Add<Output = T>,
    {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            matrix_add(&self, &other)
        }
    }

    impl<
            T: Add<Output = T> + Mul<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized,
        > Mul for Matrix<T>
    {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            matrix_multiply::<T>(&self, &rhs)
        }
    }

    fn is_multiplicatively_conformable<
        T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized,
    >(
        a: &Matrix<T>,
        b: &Matrix<T>,
    ) -> bool {
        a.n == b.m
    }

    fn is_additively_conformable<
        T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized,
    >(
        a: &Matrix<T>,
        b: &Matrix<T>,
    ) -> bool {
        a.n == b.n && a.m == b.m
    }

    pub fn matrix_multiply<
        T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul<Output = T> + ?Sized,
    >(
        a: &Matrix<T>,
        b: &Matrix<T>,
    ) -> Matrix<T> {
        if !is_multiplicatively_conformable(&a, &b) {
            panic!("Matrices are not multiplicatively conformable!");
        }
        let mut entries: Vec<T> = Vec::new();
        let a_rows = a.rows();
        let b_columns = b.columns();

        for i in 0..a_rows.len() {
            for j in 0..b_columns.len() {
                if a_rows[i].len() != b_columns[j].len() {
                    panic!(
                        "Row[{}]: Column[{}] length mismatch - cannot calculate matrix product!",
                        i, j
                    );
                }
                let mut new_entry: Option<T> = None;
                for k in 0..a_rows[i].len() {
                    match new_entry {
                        Some(value) => new_entry = Some(value + a_rows[i][k] * b_columns[j][k]),
                        None => new_entry = Some(a_rows[i][k] * b_columns[j][k]),
                    }
                }

                if entries.len() == 0 {
                    entries = vec![new_entry.expect("new_entry not initialised!")];
                } else {
                    entries.push(new_entry.expect("new_entry not initialised!"));
                }
            }
        }

        Matrix::<T> {
            m: a_rows.len(),
            n: b_columns.len(),
            entries,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix_algebra::{matrix_add, matrix_multiply, Matrix};

    use rand::prelude::*;
    use std::ops::Add;

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
        let test_matrix = Matrix::new_constant_value(m, n, 0);

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

        matrix_multiply(&test_matrix_a, &test_matrix_b);
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

        let matrix_sum = matrix_add(&test_matrix_a, &test_matrix_b);

        assert_eq!(matrix_sum.entries.len(), 12);
        assert_eq!(
            matrix_sum.entries,
            [13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13]
        );
    }

    #[test]
    fn test_matrix_add_operator() {
        let test_matrix_a = Matrix::new(3, 4, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec());
        let test_matrix_b = Matrix::new(3, 4, [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_vec());
        let matrix_sum = test_matrix_a + test_matrix_b;

        assert_eq!(matrix_sum.entries.len(), 12);
        assert_eq!(
            matrix_sum.entries,
            [13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,]
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

        assert_eq!(rows[0], vec![1, 2, 3, 4,]);
        assert_eq!(rows[1], vec![5, 6, 7, 8,]);
        assert_eq!(rows[2], vec![9, 10, 11, 12,]);
    }

    #[test]
    fn test_matrix_multiply() {
        let test_matrix_a = Matrix::new(3, 4, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec());
        let test_matrix_b = Matrix::new(4, 3, [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_vec());

        let matrix_product = matrix_multiply(&test_matrix_a, &test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [
                1 * 12 + 2 * 9 + 3 * 6 + 4 * 3,
                1 * 11 + 2 * 8 + 3 * 5 + 4 * 2,
                1 * 10 + 2 * 7 + 3 * 4 + 4 * 1,
                5 * 12 + 6 * 9 + 7 * 6 + 8 * 3,
                5 * 11 + 6 * 8 + 7 * 5 + 8 * 2,
                5 * 10 + 6 * 7 + 7 * 4 + 8 * 1,
                9 * 12 + 10 * 9 + 11 * 6 + 12 * 3,
                9 * 11 + 10 * 8 + 11 * 5 + 12 * 2,
                9 * 10 + 10 * 7 + 11 * 4 + 12 * 1
            ]
        );
    }

    #[test]
    fn test_matrix_multiply_operator() {
        let test_matrix_a = Matrix::new(3, 4, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].to_vec());
        let test_matrix_b = Matrix::new(4, 3, [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_vec());

        let matrix_product = test_matrix_a * test_matrix_b;

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [
                1 * 12 + 2 * 9 + 3 * 6 + 4 * 3,
                1 * 11 + 2 * 8 + 3 * 5 + 4 * 2,
                1 * 10 + 2 * 7 + 3 * 4 + 4 * 1,
                5 * 12 + 6 * 9 + 7 * 6 + 8 * 3,
                5 * 11 + 6 * 8 + 7 * 5 + 8 * 2,
                5 * 10 + 6 * 7 + 7 * 4 + 8 * 1,
                9 * 12 + 10 * 9 + 11 * 6 + 12 * 3,
                9 * 11 + 10 * 8 + 11 * 5 + 12 * 2,
                9 * 10 + 10 * 7 + 11 * 4 + 12 * 1
            ]
        );
    }

    #[test]
    fn test_matrix_multiply_constant_value_initialiser() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, 5);
        let test_matrix_b = Matrix::new_constant_value(4, 3, 4);

        let matrix_product = matrix_multiply(&test_matrix_a, &test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [80, 80, 80, 80, 80, 80, 80, 80, 80,]
        );
    }

    #[test]
    fn test_transpose() {
        let test_matrix = Matrix::new(2, 3, [1, 2, -1, 0, 3, 7].to_vec());

        let transpose_matrix = test_matrix.transpose();

        assert_eq!(transpose_matrix.m, test_matrix.n);
        assert_eq!(transpose_matrix.n, test_matrix.m);
        assert_eq!(transpose_matrix.entries, [1, 0, 2, 3, -1, 7]);
    }

    #[test]
    fn test_submatrix() {
        let test_matrix = Matrix::new(
            5,
            5,
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ]
            .to_vec(),
        );

        let submatrix = test_matrix.submatrix(&[0, 2, 4], &[1, 3]);

        assert_eq!(submatrix.n, 2);
        assert_eq!(submatrix.m, 3);
        assert_eq!(submatrix.entries, [2, 4, 12, 14, 22, 24]);
    }

    #[test]
    fn test_partition() {
        let test_matrix = Matrix::new(
            5,
            5,
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ]
            .to_vec(),
        );

        let submatrices = test_matrix.partition(&[3, 2], &[1, 2, 2]);

        assert_eq!(submatrices.len(), 6);
    }
}
