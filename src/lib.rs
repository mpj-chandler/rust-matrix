mod matrix_entry;

pub mod matrix_algebra {
    use crate::matrix_entry::MatrixEntry;
    use std::fmt;
    use std::ops::{Add, Mul};

    #[derive(Clone)]
    pub struct Matrix {
        pub n: usize,
        pub m: usize,
        pub entries: Vec<MatrixEntry>,
    }

    impl Matrix {
        pub fn new(m: usize, n: usize, entries: Vec<MatrixEntry>) -> Matrix {
            Matrix { m, n, entries }
        }

        pub fn new_constant_value(m: usize, n: usize, value: MatrixEntry) -> Matrix {
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
            Matrix::new_constant_value(m, n, MatrixEntry::Integer32(0))
        }

        pub fn get_entry_ij(&self, i: usize, j: usize) -> MatrixEntry {
            self.entries[(i * self.n) + j]
        }

        pub fn set_entry_ij(&mut self, i: usize, j: usize, new_value: MatrixEntry) {
            self.entries[(i * self.n) + j] = new_value;
        }

        pub fn rows(&self) -> Vec<Vec<MatrixEntry>> {
            let mut rows = Vec::new();

            for i in (0..(self.m * self.n)).step_by(self.n) {
                rows.push(self.entries[i..(i + self.n)].to_vec());
            }

            rows
        }

        pub fn columns(&self) -> Vec<Vec<MatrixEntry>> {
            let mut columns = Vec::new();

            for i in 0..self.n {
                let mut new_column: Vec<MatrixEntry> = Vec::new();
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
                let printable_row: String = row
                    .iter()
                    .map(|&entry| {
                        let mut entry_as_string =
                            entry.to_string().expect("Unable to parse entry to string");

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

    pub fn matrix_add(a: &Matrix, b: &Matrix) -> Matrix {
        if !is_additively_conformable(a, b) {
            panic!("Matrices are not additively conformable!")
        }

        let mut sum_matrix = Matrix::new_constant_value(a.m, a.n, MatrixEntry::Integer32(0));

        for i in 0..a.m {
            for j in 0..a.n {
                sum_matrix.set_entry_ij(i, j, a.get_entry_ij(i, j) + b.get_entry_ij(i, j));
            }
        }
        sum_matrix
    }

    impl Add for Matrix {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            matrix_add(&self, &other)
        }
    }

    impl Mul for Matrix {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            matrix_multiply(&self, &rhs)
        }
    }

    fn is_multiplicatively_conformable(a: &Matrix, b: &Matrix) -> bool {
        a.n == b.m
    }

    fn is_additively_conformable(a: &Matrix, b: &Matrix) -> bool {
        a.n == b.n && a.m == b.m
    }

    pub fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
        if !is_multiplicatively_conformable(&a, &b) {
            panic!("Matrices are not multiplicatively conformable!");
        }
        let mut entries: Vec<MatrixEntry> = Vec::new();
        let a_rows = a.rows();
        let b_columns = b.columns();
        let mut mul_matrix =
            Matrix::new_constant_value(a_rows.len(), b_columns.len(), MatrixEntry::Integer32(0));

        for i in 0..a_rows.len() {
            for j in 0..b_columns.len() {
                if a_rows[i].len() != b_columns[j].len() {
                    panic!(
                        "Row[{}]: Column[{}] length mismatch - cannot calculate matrix product!",
                        i, j
                    );
                }
                let mut new_entry = MatrixEntry::Integer32(0);
                for k in 0..a_rows[i].len() {
                    new_entry += a_rows[i][k] * b_columns[j][k];
                }
                mul_matrix.set_entry_ij(i, j, new_entry);
                entries.push(new_entry);
            }
        }

        mul_matrix
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix_algebra::Matrix;
    use crate::matrix_algebra::{matrix_add, matrix_multiply};
    use crate::matrix_entry::MatrixEntry;
    use std::ops::Add;

    use rand::prelude::*;

    #[test]
    fn test_constant_value_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let value = rng.gen_range(1..=100);
        let test_matrix = Matrix::new_constant_value(m, n, MatrixEntry::Integer32(value));

        assert_eq!(test_matrix.entries.len(), n * m);

        for entry in test_matrix.entries {
            assert_eq!(entry, MatrixEntry::Integer32(value));
        }
    }

    #[test]
    fn test_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let mut entries: Vec<MatrixEntry> = Vec::new();

        for _i in 0..m {
            for _j in 0..n {
                entries.push(MatrixEntry::Integer32(rng.gen_range(1..=100)));
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
            assert_eq!(entry, MatrixEntry::Integer32(0));
        }
    }

    #[test]
    #[should_panic]
    fn test_zero_first_argument_to_initialiser() {
        let _test_matrix = Matrix::new_constant_value(0, 1, MatrixEntry::Integer32(1));
    }

    #[test]
    #[should_panic]
    fn test_zero_second_argument_to_initialiser() {
        let _test_matrix = Matrix::new_constant_value(1, 0, MatrixEntry::Integer32(1));
    }

    #[test]
    #[should_panic]
    fn test_panic_on_non_multiplicatively_conformable_matrices() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, MatrixEntry::Integer32(5));
        let test_matrix_b = Matrix::new_constant_value(5, 7, MatrixEntry::Integer32(4));

        matrix_multiply(&test_matrix_a, &test_matrix_b);
    }

    #[test]
    #[should_panic]
    fn test_panic_on_non_additively_conformable_matrices() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, MatrixEntry::Integer32(5));
        let test_matrix_b = Matrix::new_constant_value(5, 7, MatrixEntry::Integer32(4));

        let _ = test_matrix_a.add(test_matrix_b);
    }

    #[test]
    fn test_matrix_add() {
        let test_matrix_a = Matrix::new(
            3,
            4,
            [
                MatrixEntry::Integer32(1),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(12),
            ]
            .to_vec(),
        );
        let test_matrix_b = Matrix::new(
            3,
            4,
            [
                MatrixEntry::Integer32(12),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(1),
            ]
            .to_vec(),
        );

        let matrix_sum = matrix_add(&test_matrix_a, &test_matrix_b);

        assert_eq!(matrix_sum.entries.len(), 12);
        assert_eq!(
            matrix_sum.entries,
            [
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13)
            ]
        );
    }

    #[test]
    fn test_matrix_add_operator() {
        let test_matrix_a = Matrix::new(
            3,
            4,
            [
                MatrixEntry::Integer32(1),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(12),
            ]
            .to_vec(),
        );
        let test_matrix_b = Matrix::new(
            3,
            4,
            [
                MatrixEntry::Integer32(12),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(1),
            ]
            .to_vec(),
        );
        let matrix_sum = test_matrix_a + test_matrix_b;

        assert_eq!(matrix_sum.entries.len(), 12);
        assert_eq!(
            matrix_sum.entries,
            [
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13),
                MatrixEntry::Integer32(13)
            ]
        );
    }

    #[test]
    fn test_columns() {
        let test_matrix = Matrix::new(
            3,
            4,
            [
                MatrixEntry::Integer32(1),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(12),
            ]
            .to_vec(),
        );

        let columns = test_matrix.columns();

        assert_eq!(columns.len(), 4);

        assert_eq!(
            columns[0],
            vec![
                MatrixEntry::Integer32(1),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(9)
            ]
        );
        assert_eq!(
            columns[1],
            vec![
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(10)
            ]
        );
        assert_eq!(
            columns[2],
            vec![
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(11)
            ]
        );
        assert_eq!(
            columns[3],
            vec![
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(12)
            ]
        );
    }

    #[test]
    fn test_rows() {
        let test_matrix = Matrix::new(
            3,
            4,
            [
                MatrixEntry::Integer32(1),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(12),
            ]
            .to_vec(),
        );

        let rows = test_matrix.rows();

        assert_eq!(rows.len(), 3);

        assert_eq!(
            rows[0],
            vec![
                MatrixEntry::Integer32(1),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(4)
            ]
        );
        assert_eq!(
            rows[1],
            vec![
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(8)
            ]
        );
        assert_eq!(
            rows[2],
            vec![
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(12)
            ]
        );
    }

    #[test]
    fn test_matrix_multiply() {
        let test_matrix_a = Matrix::new(
            3,
            4,
            [
                MatrixEntry::Integer32(1),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(12),
            ]
            .to_vec(),
        );
        let test_matrix_b = Matrix::new(
            4,
            3,
            [
                MatrixEntry::Integer32(12),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(1),
            ]
            .to_vec(),
        );

        let matrix_product = matrix_multiply(&test_matrix_a, &test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [
                MatrixEntry::Integer32(1 * 12)
                    + MatrixEntry::Integer32(2 * 9)
                    + MatrixEntry::Integer32(3 * 6)
                    + MatrixEntry::Integer32(4 * 3),
                MatrixEntry::Integer32(1 * 11)
                    + MatrixEntry::Integer32(2 * 8)
                    + MatrixEntry::Integer32(3 * 5)
                    + MatrixEntry::Integer32(4 * 2),
                MatrixEntry::Integer32(1 * 10)
                    + MatrixEntry::Integer32(2 * 7)
                    + MatrixEntry::Integer32(3 * 4)
                    + MatrixEntry::Integer32(4 * 1),
                MatrixEntry::Integer32(5 * 12)
                    + MatrixEntry::Integer32(6 * 9)
                    + MatrixEntry::Integer32(7 * 6)
                    + MatrixEntry::Integer32(8 * 3),
                MatrixEntry::Integer32(5 * 11)
                    + MatrixEntry::Integer32(6 * 8)
                    + MatrixEntry::Integer32(7 * 5)
                    + MatrixEntry::Integer32(8 * 2),
                MatrixEntry::Integer32(5 * 10)
                    + MatrixEntry::Integer32(6 * 7)
                    + MatrixEntry::Integer32(7 * 4)
                    + MatrixEntry::Integer32(8 * 1),
                MatrixEntry::Integer32(9 * 12)
                    + MatrixEntry::Integer32(10 * 9)
                    + MatrixEntry::Integer32(11 * 6)
                    + MatrixEntry::Integer32(12 * 3),
                MatrixEntry::Integer32(9 * 11)
                    + MatrixEntry::Integer32(10 * 8)
                    + MatrixEntry::Integer32(11 * 5)
                    + MatrixEntry::Integer32(12 * 2),
                MatrixEntry::Integer32(9 * 10)
                    + MatrixEntry::Integer32(10 * 7)
                    + MatrixEntry::Integer32(11 * 4)
                    + MatrixEntry::Integer32(12 * 1)
            ]
        );
    }

    #[test]
    fn test_matrix_multiply_operator() {
        let test_matrix_a = Matrix::new(
            3,
            4,
            [
                MatrixEntry::Integer32(1),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(12),
            ]
            .to_vec(),
        );
        let test_matrix_b = Matrix::new(
            4,
            3,
            [
                MatrixEntry::Integer32(12),
                MatrixEntry::Integer32(11),
                MatrixEntry::Integer32(10),
                MatrixEntry::Integer32(9),
                MatrixEntry::Integer32(8),
                MatrixEntry::Integer32(7),
                MatrixEntry::Integer32(6),
                MatrixEntry::Integer32(5),
                MatrixEntry::Integer32(4),
                MatrixEntry::Integer32(3),
                MatrixEntry::Integer32(2),
                MatrixEntry::Integer32(1),
            ]
            .to_vec(),
        );

        let matrix_product = test_matrix_a * test_matrix_b;

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [
                MatrixEntry::Integer32(1 * 12)
                    + MatrixEntry::Integer32(2 * 9)
                    + MatrixEntry::Integer32(3 * 6)
                    + MatrixEntry::Integer32(4 * 3),
                MatrixEntry::Integer32(1 * 11)
                    + MatrixEntry::Integer32(2 * 8)
                    + MatrixEntry::Integer32(3 * 5)
                    + MatrixEntry::Integer32(4 * 2),
                MatrixEntry::Integer32(1 * 10)
                    + MatrixEntry::Integer32(2 * 7)
                    + MatrixEntry::Integer32(3 * 4)
                    + MatrixEntry::Integer32(4 * 1),
                MatrixEntry::Integer32(5 * 12)
                    + MatrixEntry::Integer32(6 * 9)
                    + MatrixEntry::Integer32(7 * 6)
                    + MatrixEntry::Integer32(8 * 3),
                MatrixEntry::Integer32(5 * 11)
                    + MatrixEntry::Integer32(6 * 8)
                    + MatrixEntry::Integer32(7 * 5)
                    + MatrixEntry::Integer32(8 * 2),
                MatrixEntry::Integer32(5 * 10)
                    + MatrixEntry::Integer32(6 * 7)
                    + MatrixEntry::Integer32(7 * 4)
                    + MatrixEntry::Integer32(8 * 1),
                MatrixEntry::Integer32(9 * 12)
                    + MatrixEntry::Integer32(10 * 9)
                    + MatrixEntry::Integer32(11 * 6)
                    + MatrixEntry::Integer32(12 * 3),
                MatrixEntry::Integer32(9 * 11)
                    + MatrixEntry::Integer32(10 * 8)
                    + MatrixEntry::Integer32(11 * 5)
                    + MatrixEntry::Integer32(12 * 2),
                MatrixEntry::Integer32(9 * 10)
                    + MatrixEntry::Integer32(10 * 7)
                    + MatrixEntry::Integer32(11 * 4)
                    + MatrixEntry::Integer32(12 * 1)
            ]
        );
    }

    #[test]
    fn test_matrix_multiply_constant_value_initialiser() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, MatrixEntry::Integer32(5));
        let test_matrix_b = Matrix::new_constant_value(4, 3, MatrixEntry::Integer32(4));

        let matrix_product = matrix_multiply(&test_matrix_a, &test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [
                MatrixEntry::Integer32(80),
                MatrixEntry::Integer32(80),
                MatrixEntry::Integer32(80),
                MatrixEntry::Integer32(80),
                MatrixEntry::Integer32(80),
                MatrixEntry::Integer32(80),
                MatrixEntry::Integer32(80),
                MatrixEntry::Integer32(80),
                MatrixEntry::Integer32(80)
            ]
        );
    }
}
