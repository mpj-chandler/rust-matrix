pub mod matrix_algebra {
    use std::fmt;
    use std::mem::size_of;
    use std::ops::{Add, AddAssign, Mul};

    #[derive(Clone, Debug, PartialEq, Copy)]
    pub enum Entry {
        Integer32(i32),
        Float64(f64),
    }

    pub trait EntryFromRaw {
        fn entry_from_raw(&self) -> Entry;
    }

    impl EntryFromRaw for i32 {
        fn entry_from_raw(&self) -> Entry {
            Entry::Integer32(*self)
        }
    }

    impl EntryFromRaw for f64 {
        fn entry_from_raw(&self) -> Entry {
            Entry::Float64(*self)
        }
    }

    impl Entry {
        fn from_raw(value: &dyn EntryFromRaw) -> Entry {
            value.entry_from_raw()
        }

        fn to_int32(self) -> Option<i32> {
            match self {
                Self::Integer32(i) => Some(i),
                Self::Float64(i) => Some(i as i32),
            }
        }

        fn to_float64(self) -> Option<f64> {
            match self {
                Self::Integer32(i) => Some(i as f64),
                Self::Float64(i) => Some(i),
            }
        }

        fn to_string(self) -> Option<String> {
            match self {
                Self::Integer32(i) => Some(i.to_string()),
                Self::Float64(i) => Some(i.to_string()),
            }
        }
    }

    pub fn entry_add(a: &Entry, b: &Entry) -> Entry {
        match (a, b) {
            (Entry::Integer32(x), Entry::Integer32(y)) => Entry::Integer32(x + y),
            (Entry::Float64(x), Entry::Float64(y)) => Entry::Float64(x + y),
            (Entry::Float64(x), Entry::Integer32(y)) => Entry::Float64(x * (*y as f64)),
            (Entry::Integer32(x), Entry::Float64(y)) => Entry::Float64((*x as f64) * y),
        }
    }

    impl Add for Entry {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            entry_add(&self, &other)
        }
    }

    impl AddAssign for Entry {
        fn add_assign(&mut self, other: Self) {
            *self = *self + other;
        }
    }

    pub fn entry_multiply(a: &Entry, b: &Entry) -> Entry {
        match (a, b) {
            (Entry::Integer32(x), Entry::Integer32(y)) => Entry::Integer32(x * y),
            (Entry::Float64(x), Entry::Float64(y)) => Entry::Float64(x * y),
            (Entry::Float64(x), Entry::Integer32(y)) => Entry::Float64(x * (*y as f64)),
            (Entry::Integer32(x), Entry::Float64(y)) => Entry::Float64((*x as f64) * y),
        }
    }

    impl Mul for Entry {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            entry_multiply(&self, &rhs)
        }
    }

    #[derive(Clone)]
    pub struct Matrix {
        pub n: usize,
        pub m: usize,
        pub entries: Vec<Entry>,
    }

    impl Matrix {
        pub fn new(m: usize, n: usize, entries: Vec<Entry>) -> Matrix {
            Matrix { m, n, entries }
        }

        pub fn new_constant_value(m: usize, n: usize, value: Entry) -> Matrix {
            if m == 0 || n == 0 {
                panic!("Matrix dimensions must each be greater than zero!");
            }

            Matrix {
                n: n,
                m: m,
                entries: vec![value; n * m],
            }
        }

        pub fn new_from_raw<T>(m: usize, n: usize, values: Vec<T>) -> Matrix
        where
            T: EntryFromRaw,
        {
            let mut entries: Vec<Entry> = Vec::with_capacity(n * m * (size_of::<Entry>()));

            for value in values {
                entries.push(value.entry_from_raw());
            }

            Matrix { m, n, entries }
        }

        pub fn new_all_zeroes(m: usize, n: usize) -> Matrix {
            Matrix::new_constant_value(m, n, Entry::Integer32(0))
        }

        pub fn get_entry_ij(&self, i: usize, j: usize) -> Entry {
            self.entries[(i * self.n) + j]
        }

        pub fn set_entry_ij(&mut self, i: usize, j: usize, new_value: Entry) {
            self.entries[(i * self.n) + j] = new_value;
        }

        pub fn rows(&self) -> Vec<Vec<Entry>> {
            let mut rows = Vec::new();

            for i in (0..(self.m * self.n)).step_by(self.n) {
                rows.push(self.entries[i..(i + self.n)].to_vec());
            }

            rows
        }

        pub fn columns(&self) -> Vec<Vec<Entry>> {
            let mut columns = Vec::new();

            for i in 0..self.n {
                let mut new_column: Vec<Entry> = Vec::new();
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

        let mut sum_matrix = Matrix::new_constant_value(a.m, a.n, Entry::Integer32(0));

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
        let mut entries: Vec<Entry> = Vec::new();
        let a_rows = a.rows();
        let b_columns = b.columns();
        let mut mul_matrix =
            Matrix::new_constant_value(a_rows.len(), b_columns.len(), Entry::Integer32(0));

        for i in 0..a_rows.len() {
            for j in 0..b_columns.len() {
                if a_rows[i].len() != b_columns[j].len() {
                    panic!(
                        "Row[{}]: Column[{}] length mismatch - cannot calculate matrix product!",
                        i, j
                    );
                }
                let mut new_entry = Entry::Integer32(0);
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
    use crate::matrix_algebra::{matrix_add, matrix_multiply};
    use crate::matrix_algebra::{Entry, EntryFromRaw, Matrix};
    use std::mem::size_of;
    use std::ops::{Add, AddAssign, Mul};

    use rand::prelude::*;

    #[test]
    fn test_constant_value_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let value = rng.gen_range(1..=100);
        let test_matrix = Matrix::new_constant_value(m, n, Entry::Integer32(value));

        assert_eq!(test_matrix.entries.len(), n * m);

        for entry in test_matrix.entries {
            assert_eq!(entry, Entry::Integer32(value));
        }
    }

    #[test]
    fn test_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let mut entries: Vec<Entry> = Vec::new();

        for _i in 0..m {
            for _j in 0..n {
                entries.push(Entry::Integer32(rng.gen_range(1..=100)));
            }
        }

        let test_matrix = Matrix { m, n, entries };

        assert_eq!(test_matrix.entries.len(), n * m);
    }

    #[test]
    fn test_from_raw_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let mut values: &[dyn EntryFromRaw] =
            &Vec::with_capacity(n * m * size_of::<f64>()).into_boxed_slice();

        for i in 0..m {
            for j in 0..n {
                if j % 2 == 0 {
                    values.add(rng.gen_range(1..=100) as f64);
                } else {
                    values.add(rng.gen_range(1..=100) as i32);
                }
            }
        }

        let test_matrix = Matrix::new_from_raw(m, n, *values.to_vec());

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
            assert_eq!(entry, Entry::Integer32(0));
        }
    }

    #[test]
    #[should_panic]
    fn test_zero_first_argument_to_initialiser() {
        let _test_matrix = Matrix::new_constant_value(0, 1, Entry::Integer32(1));
    }

    #[test]
    #[should_panic]
    fn test_zero_second_argument_to_initialiser() {
        let _test_matrix = Matrix::new_constant_value(1, 0, Entry::Integer32(1));
    }

    #[test]
    #[should_panic]
    fn test_panic_on_non_multiplicatively_conformable_matrices() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, Entry::Integer32(5));
        let test_matrix_b = Matrix::new_constant_value(5, 7, Entry::Integer32(4));

        matrix_multiply(&test_matrix_a, &test_matrix_b);
    }

    #[test]
    #[should_panic]
    fn test_panic_on_non_additively_conformable_matrices() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, Entry::Integer32(5));
        let test_matrix_b = Matrix::new_constant_value(5, 7, Entry::Integer32(4));

        let _ = test_matrix_a.add(test_matrix_b);
    }

    #[test]
    fn test_entry_add() {
        let mut rng = rand::thread_rng();
        let test_value_one = rng.gen_range(1..=100);
        let test_value_two = rng.gen_range(1..=100);

        let test_entry_one = Entry::Integer32(test_value_one);
        let test_entry_two = Entry::Integer32(test_value_two);

        let result = test_entry_one + test_entry_two;

        assert_eq!(result, Entry::Integer32(test_value_one + test_value_two));
    }

    #[test]
    fn test_matrix_add() {
        let test_matrix_a = Matrix::new(
            3,
            4,
            [
                Entry::Integer32(1),
                Entry::Integer32(2),
                Entry::Integer32(3),
                Entry::Integer32(4),
                Entry::Integer32(5),
                Entry::Integer32(6),
                Entry::Integer32(7),
                Entry::Integer32(8),
                Entry::Integer32(9),
                Entry::Integer32(10),
                Entry::Integer32(11),
                Entry::Integer32(12),
            ]
            .to_vec(),
        );
        let test_matrix_b = Matrix::new(
            3,
            4,
            [
                Entry::Integer32(12),
                Entry::Integer32(11),
                Entry::Integer32(10),
                Entry::Integer32(9),
                Entry::Integer32(8),
                Entry::Integer32(7),
                Entry::Integer32(6),
                Entry::Integer32(5),
                Entry::Integer32(4),
                Entry::Integer32(3),
                Entry::Integer32(2),
                Entry::Integer32(1),
            ]
            .to_vec(),
        );

        let matrix_sum = matrix_add(&test_matrix_a, &test_matrix_b);

        assert_eq!(matrix_sum.entries.len(), 12);
        assert_eq!(
            matrix_sum.entries,
            [
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13)
            ]
        );
    }

    #[test]
    fn test_matrix_add_operator() {
        let test_matrix_a = Matrix::new(
            3,
            4,
            [
                Entry::Integer32(1),
                Entry::Integer32(2),
                Entry::Integer32(3),
                Entry::Integer32(4),
                Entry::Integer32(5),
                Entry::Integer32(6),
                Entry::Integer32(7),
                Entry::Integer32(8),
                Entry::Integer32(9),
                Entry::Integer32(10),
                Entry::Integer32(11),
                Entry::Integer32(12),
            ]
            .to_vec(),
        );
        let test_matrix_b = Matrix::new(
            3,
            4,
            [
                Entry::Integer32(12),
                Entry::Integer32(11),
                Entry::Integer32(10),
                Entry::Integer32(9),
                Entry::Integer32(8),
                Entry::Integer32(7),
                Entry::Integer32(6),
                Entry::Integer32(5),
                Entry::Integer32(4),
                Entry::Integer32(3),
                Entry::Integer32(2),
                Entry::Integer32(1),
            ]
            .to_vec(),
        );
        let matrix_sum = test_matrix_a + test_matrix_b;

        assert_eq!(matrix_sum.entries.len(), 12);
        assert_eq!(
            matrix_sum.entries,
            [
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13),
                Entry::Integer32(13)
            ]
        );
    }

    #[test]
    fn test_columns() {
        let test_matrix = Matrix::new(
            3,
            4,
            [
                Entry::Integer32(1),
                Entry::Integer32(2),
                Entry::Integer32(3),
                Entry::Integer32(4),
                Entry::Integer32(5),
                Entry::Integer32(6),
                Entry::Integer32(7),
                Entry::Integer32(8),
                Entry::Integer32(9),
                Entry::Integer32(10),
                Entry::Integer32(11),
                Entry::Integer32(12),
            ]
            .to_vec(),
        );

        let columns = test_matrix.columns();

        assert_eq!(columns.len(), 4);

        assert_eq!(
            columns[0],
            vec![
                Entry::Integer32(1),
                Entry::Integer32(5),
                Entry::Integer32(9)
            ]
        );
        assert_eq!(
            columns[1],
            vec![
                Entry::Integer32(2),
                Entry::Integer32(6),
                Entry::Integer32(10)
            ]
        );
        assert_eq!(
            columns[2],
            vec![
                Entry::Integer32(3),
                Entry::Integer32(7),
                Entry::Integer32(11)
            ]
        );
        assert_eq!(
            columns[3],
            vec![
                Entry::Integer32(4),
                Entry::Integer32(8),
                Entry::Integer32(12)
            ]
        );
    }

    #[test]
    fn test_rows() {
        let test_matrix = Matrix::new(
            3,
            4,
            [
                Entry::Integer32(1),
                Entry::Integer32(2),
                Entry::Integer32(3),
                Entry::Integer32(4),
                Entry::Integer32(5),
                Entry::Integer32(6),
                Entry::Integer32(7),
                Entry::Integer32(8),
                Entry::Integer32(9),
                Entry::Integer32(10),
                Entry::Integer32(11),
                Entry::Integer32(12),
            ]
            .to_vec(),
        );

        let rows = test_matrix.rows();

        assert_eq!(rows.len(), 3);

        assert_eq!(
            rows[0],
            vec![
                Entry::Integer32(1),
                Entry::Integer32(2),
                Entry::Integer32(3),
                Entry::Integer32(4)
            ]
        );
        assert_eq!(
            rows[1],
            vec![
                Entry::Integer32(5),
                Entry::Integer32(6),
                Entry::Integer32(7),
                Entry::Integer32(8)
            ]
        );
        assert_eq!(
            rows[2],
            vec![
                Entry::Integer32(9),
                Entry::Integer32(10),
                Entry::Integer32(11),
                Entry::Integer32(12)
            ]
        );
    }

    #[test]
    fn test_matrix_multiply() {
        let test_matrix_a = Matrix::new(
            3,
            4,
            [
                Entry::Integer32(1),
                Entry::Integer32(2),
                Entry::Integer32(3),
                Entry::Integer32(4),
                Entry::Integer32(5),
                Entry::Integer32(6),
                Entry::Integer32(7),
                Entry::Integer32(8),
                Entry::Integer32(9),
                Entry::Integer32(10),
                Entry::Integer32(11),
                Entry::Integer32(12),
            ]
            .to_vec(),
        );
        let test_matrix_b = Matrix::new(
            4,
            3,
            [
                Entry::Integer32(12),
                Entry::Integer32(11),
                Entry::Integer32(10),
                Entry::Integer32(9),
                Entry::Integer32(8),
                Entry::Integer32(7),
                Entry::Integer32(6),
                Entry::Integer32(5),
                Entry::Integer32(4),
                Entry::Integer32(3),
                Entry::Integer32(2),
                Entry::Integer32(1),
            ]
            .to_vec(),
        );

        let matrix_product = matrix_multiply(&test_matrix_a, &test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [
                Entry::Integer32(1 * 12)
                    + Entry::Integer32(2 * 9)
                    + Entry::Integer32(3 * 6)
                    + Entry::Integer32(4 * 3),
                Entry::Integer32(1 * 11)
                    + Entry::Integer32(2 * 8)
                    + Entry::Integer32(3 * 5)
                    + Entry::Integer32(4 * 2),
                Entry::Integer32(1 * 10)
                    + Entry::Integer32(2 * 7)
                    + Entry::Integer32(3 * 4)
                    + Entry::Integer32(4 * 1),
                Entry::Integer32(5 * 12)
                    + Entry::Integer32(6 * 9)
                    + Entry::Integer32(7 * 6)
                    + Entry::Integer32(8 * 3),
                Entry::Integer32(5 * 11)
                    + Entry::Integer32(6 * 8)
                    + Entry::Integer32(7 * 5)
                    + Entry::Integer32(8 * 2),
                Entry::Integer32(5 * 10)
                    + Entry::Integer32(6 * 7)
                    + Entry::Integer32(7 * 4)
                    + Entry::Integer32(8 * 1),
                Entry::Integer32(9 * 12)
                    + Entry::Integer32(10 * 9)
                    + Entry::Integer32(11 * 6)
                    + Entry::Integer32(12 * 3),
                Entry::Integer32(9 * 11)
                    + Entry::Integer32(10 * 8)
                    + Entry::Integer32(11 * 5)
                    + Entry::Integer32(12 * 2),
                Entry::Integer32(9 * 10)
                    + Entry::Integer32(10 * 7)
                    + Entry::Integer32(11 * 4)
                    + Entry::Integer32(12 * 1)
            ]
        );
    }

    #[test]
    fn test_matrix_multiply_operator() {
        let test_matrix_a = Matrix::new(
            3,
            4,
            [
                Entry::Integer32(1),
                Entry::Integer32(2),
                Entry::Integer32(3),
                Entry::Integer32(4),
                Entry::Integer32(5),
                Entry::Integer32(6),
                Entry::Integer32(7),
                Entry::Integer32(8),
                Entry::Integer32(9),
                Entry::Integer32(10),
                Entry::Integer32(11),
                Entry::Integer32(12),
            ]
            .to_vec(),
        );
        let test_matrix_b = Matrix::new(
            4,
            3,
            [
                Entry::Integer32(12),
                Entry::Integer32(11),
                Entry::Integer32(10),
                Entry::Integer32(9),
                Entry::Integer32(8),
                Entry::Integer32(7),
                Entry::Integer32(6),
                Entry::Integer32(5),
                Entry::Integer32(4),
                Entry::Integer32(3),
                Entry::Integer32(2),
                Entry::Integer32(1),
            ]
            .to_vec(),
        );

        let matrix_product = test_matrix_a * test_matrix_b;

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [
                Entry::Integer32(1 * 12)
                    + Entry::Integer32(2 * 9)
                    + Entry::Integer32(3 * 6)
                    + Entry::Integer32(4 * 3),
                Entry::Integer32(1 * 11)
                    + Entry::Integer32(2 * 8)
                    + Entry::Integer32(3 * 5)
                    + Entry::Integer32(4 * 2),
                Entry::Integer32(1 * 10)
                    + Entry::Integer32(2 * 7)
                    + Entry::Integer32(3 * 4)
                    + Entry::Integer32(4 * 1),
                Entry::Integer32(5 * 12)
                    + Entry::Integer32(6 * 9)
                    + Entry::Integer32(7 * 6)
                    + Entry::Integer32(8 * 3),
                Entry::Integer32(5 * 11)
                    + Entry::Integer32(6 * 8)
                    + Entry::Integer32(7 * 5)
                    + Entry::Integer32(8 * 2),
                Entry::Integer32(5 * 10)
                    + Entry::Integer32(6 * 7)
                    + Entry::Integer32(7 * 4)
                    + Entry::Integer32(8 * 1),
                Entry::Integer32(9 * 12)
                    + Entry::Integer32(10 * 9)
                    + Entry::Integer32(11 * 6)
                    + Entry::Integer32(12 * 3),
                Entry::Integer32(9 * 11)
                    + Entry::Integer32(10 * 8)
                    + Entry::Integer32(11 * 5)
                    + Entry::Integer32(12 * 2),
                Entry::Integer32(9 * 10)
                    + Entry::Integer32(10 * 7)
                    + Entry::Integer32(11 * 4)
                    + Entry::Integer32(12 * 1)
            ]
        );
    }

    #[test]
    fn test_matrix_multiply_constant_value_initialiser() {
        let test_matrix_a = Matrix::new_constant_value(3, 4, Entry::Integer32(5));
        let test_matrix_b = Matrix::new_constant_value(4, 3, Entry::Integer32(4));

        let matrix_product = matrix_multiply(&test_matrix_a, &test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 9);

        assert_eq!(
            matrix_product.entries,
            [
                Entry::Integer32(80),
                Entry::Integer32(80),
                Entry::Integer32(80),
                Entry::Integer32(80),
                Entry::Integer32(80),
                Entry::Integer32(80),
                Entry::Integer32(80),
                Entry::Integer32(80),
                Entry::Integer32(80)
            ]
        );
    }
}
