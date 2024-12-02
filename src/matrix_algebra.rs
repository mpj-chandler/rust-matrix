use std::fmt;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

pub trait MatrixElementRequiredTraits<T>:
    Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
    + AddAssign
    + MulAssign
    + Clone
    + Copy
    + Display
    + PartialOrd<T>
    + Neg<Output = T>
    + Default
    + From<u8>
{
}

impl<
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + AddAssign
            + MulAssign
            + Clone
            + Copy
            + Display
            + PartialOrd<T>
            + Neg<Output = T>
            + Default
            + From<u8>
            + ?Sized,
    > MatrixElementRequiredTraits<T> for T
{
}

#[derive(PartialEq, Debug, PartialOrd, Clone)]
pub struct Matrix<T: MatrixElementRequiredTraits<T>> {
    pub n: usize,
    pub m: usize,
    pub entries: Vec<T>,
}

impl<T: MatrixElementRequiredTraits<T>> Matrix<T> {
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

    pub fn row_interchange(&self, first_row_index: usize, second_row_index: usize) -> Matrix<T> {
        let rows = self.rows();
        let first_row = &rows[first_row_index];
        let second_row = &rows[second_row_index];

        let mut new_entries: Vec<T> = Vec::new();

        for row_index in 0..rows.len() {
            if row_index == first_row_index {
                for entry in second_row {
                    new_entries.push(*entry);
                }
            } else if row_index == second_row_index {
                for entry in first_row {
                    new_entries.push(*entry);
                }
            } else {
                for entry in rows[row_index].clone() {
                    new_entries.push(entry);
                }
            }
        }

        Matrix {
            m: self.m,
            n: self.n,
            entries: new_entries,
        }
    }

    pub fn multiply_row_by_scalar(&self, row_index: usize, scalar: T) -> Matrix<T> {
        let rows = self.rows();
        let row_to_be_multiplied = &rows[row_index];

        let mut new_entries: Vec<T> = Vec::new();

        for i in 0..rows.len() {
            if i == row_index {
                for entry in row_to_be_multiplied {
                    new_entries.push(*entry * scalar);
                }
            } else {
                for entry in rows[i].clone() {
                    new_entries.push(entry);
                }
            }
        }

        Matrix {
            m: self.m,
            n: self.n,
            entries: new_entries,
        }
    }

    pub fn add_row_to_scalar_multiple_of_row(
        &self,
        target_index: usize,
        source_index: usize,
        scalar: T,
    ) -> Matrix<T> {
        let rows = self.rows();
        let row_to_be_added_to = rows[target_index].clone();
        let row_to_be_added: Vec<T> = rows[source_index]
            .clone()
            .into_iter()
            .map(|entry| -> T { scalar * entry })
            .collect();

        let mut new_entries: Vec<T> = Vec::new();

        for i in 0..rows.len() {
            if i == target_index {
                let new_row = row_to_be_added.iter().zip(row_to_be_added_to.iter());
                for (_, (lhs, rhs)) in new_row.enumerate() {
                    new_entries.push(*lhs + *rhs);
                }
            } else {
                for entry in rows[i].clone() {
                    new_entries.push(entry);
                }
            }
        }

        Matrix {
            m: self.m,
            n: self.n,
            entries: new_entries,
        }
    }

    fn all_zeroes(&self) -> bool {
        for i in 0..self.m {
            for j in 0..self.n {
                if *self.get_entry_ij(i, j) != T::default() {
                    return false;
                }
            }
        }

        true
    }

    fn first_row_with_non_zero_entry_in_column(&self, column_index: usize) -> usize {
        let rows = self.rows();
        let mut first_row_with_non_zero_entry = 0;

        for row in rows {
            if row[column_index] != T::default() {
                break;
            }
            first_row_with_non_zero_entry += 1;
        }

        first_row_with_non_zero_entry
    }

    fn reduce_rows_relative_to_row(&self, column_index: usize, row_index: usize) -> Matrix<T> {
        let mut reduced = self.clone();
        for j in 0..self.m {
            if j != row_index {
                let value_in_non_zero_column_of_row = self.get_entry_ij(j, column_index);
                if *value_in_non_zero_column_of_row != T::default() {
                    let scalar = -T::from(1) * *value_in_non_zero_column_of_row;
                    reduced = reduced.add_row_to_scalar_multiple_of_row(j, row_index, scalar);
                }
            }
        }

        reduced
    }

    fn reduce_first_row(&self, column_index: usize) -> (Matrix<T>, T) {
        let mut coefficient = T::from(1);
        let first_row_with_non_zero_entry =
            self.first_row_with_non_zero_entry_in_column(column_index);
        let a_k_interchanged = self.row_interchange(0, first_row_with_non_zero_entry);
        let scalar = T::from(1) / *a_k_interchanged.get_entry_ij(0, column_index);
        coefficient = coefficient / scalar;
        (
            a_k_interchanged.multiply_row_by_scalar(0, scalar),
            coefficient,
        )
    }

    fn row_echolon_form_recursive_with_coefficient(&self, k: usize) -> (Matrix<T>, T) {
        let mut coefficient = T::from(1);
        let mut partitioned: Vec<Matrix<T>> = Vec::new();

        if k != 0 {
            partitioned = self.partition(&[self.n], &[k, self.m - k]);
        }

        let a_k = if k == 0 { self } else { &partitioned[1] };

        if a_k.all_zeroes() {
            return (self.clone(), coefficient);
        }

        let columns = a_k.columns();
        let first_non_zero_column_index = first_non_zero_vec_index(&columns);
        let (mut a_k, new_coefficient) = a_k.reduce_first_row(first_non_zero_column_index);

        coefficient = coefficient * -new_coefficient;

        let combined_entries = if k == 0 {
            &a_k.entries
        } else {
            let _ = &partitioned[0].entries.append(&mut a_k.entries);
            &partitioned[0].entries
        };

        let combined = Matrix::new(self.m, self.n, combined_entries.to_vec());
        let reduced = combined.reduce_rows_relative_to_row(first_non_zero_column_index, k);

        if k != self.m - 1 {
            let (row_echolon, new_coefficient) =
                reduced.row_echolon_form_recursive_with_coefficient(k + 1);
            return (row_echolon, new_coefficient * coefficient);
        }

        (reduced, coefficient)
    }

    fn row_echolon_form_recursive(&self, k: usize) -> Matrix<T> {
        self.row_echolon_form_recursive_with_coefficient(k).0
    }

    pub fn row_echolon_form(&self) -> Matrix<T> {
        self.row_echolon_form_recursive(0)
    }

    pub fn column_echolon_form(&self) -> Matrix<T> {
        self.transpose().row_echolon_form().transpose()
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
                transposed_entries.push(entry);
            }
        }

        Matrix::<T> {
            m: self.n,
            n: self.m,
            entries: transposed_entries,
        }
    }

    pub fn determinant(&self) -> T {
        let (row_echolon_form, coefficient) = self.row_echolon_form_recursive_with_coefficient(0);

        println!("REF:\n{}", row_echolon_form);
        let mut determinant: Option<T> = None;

        for i in 0..row_echolon_form.n {
            for j in 0..row_echolon_form.m {
                if i == j {
                    let entry = row_echolon_form.get_entry_ij(i, j);
                    match determinant {
                        None => determinant = Some(*entry),
                        Some(det) => determinant = Some(det * *entry),
                    }
                }
            }
        }

        match determinant {
            Some(det) => return det * coefficient,
            None => panic!("Unable to calculate determinant!"),
        }
    }

    pub fn partition(
        &self,
        column_partitioning: &[usize],
        row_partitioning: &[usize],
    ) -> Vec<Matrix<T>> {
        let mut partitioned_matrices: Vec<Matrix<T>> = Vec::new();
        let mut row_offset = 0;

        for row_partition in row_partitioning {
            let mut column_offset = 0;
            for column_partition in column_partitioning {
                let mut new_entries: Vec<T> = Vec::new();

                for i in row_offset..(row_offset + *row_partition) {
                    for j in column_offset..(column_offset + *column_partition) {
                        new_entries.push(*self.get_entry_ij(i, j));
                    }
                }
                partitioned_matrices.push(Matrix {
                    m: *row_partition,
                    n: *column_partition,
                    entries: new_entries,
                });
                column_offset += *column_partition;
            }
            row_offset += *row_partition;
        }

        partitioned_matrices
    }

    pub fn submatrix(&self, column_indices: &[usize], row_indices: &[usize]) -> Matrix<T> {
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
}

impl<T: MatrixElementRequiredTraits<T>> fmt::Display for Matrix<T> {
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

pub fn new_all_default<T>(m: usize, n: usize) -> Matrix<T>
where
    T: MatrixElementRequiredTraits<T>,
{
    if m == 0 || n == 0 {
        panic!("Matrix dimensions must each be greater than zero!");
    }

    Matrix::<T> {
        n,
        m,
        entries: vec![T::default(); n * m],
    }
}

pub fn new_identity_matrix<T>(dimension: usize) -> Matrix<T>
where
    T: MatrixElementRequiredTraits<T>,
{
    let mut new_empty_matrix = new_all_default::<T>(dimension, dimension);

    for index in 0..dimension {
        new_empty_matrix.set_entry_ij(index, index, &T::from(1));
    }

    println!("{new_empty_matrix}");
    new_empty_matrix
}

fn matrix_add<T: MatrixElementRequiredTraits<T>>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
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

impl<T: MatrixElementRequiredTraits<T>> Add for Matrix<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        matrix_add(&self, &other)
    }
}

impl<T: MatrixElementRequiredTraits<T>> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        matrix_multiply::<T>(&self, &rhs)
    }
}

fn is_multiplicatively_conformable<T: MatrixElementRequiredTraits<T>>(
    a: &Matrix<T>,
    b: &Matrix<T>,
) -> bool {
    a.n == b.m
}

fn is_additively_conformable<T: MatrixElementRequiredTraits<T>>(
    a: &Matrix<T>,
    b: &Matrix<T>,
) -> bool {
    a.n == b.n && a.m == b.m
}

fn matrix_multiply<T: MatrixElementRequiredTraits<T>>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
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

fn first_non_zero_vec_index<T: MatrixElementRequiredTraits<T>>(input: &Vec<Vec<T>>) -> usize {
    let mut first_nonzero_vec_index = 0;

    for vector in input {
        for element in vector {
            if *element != T::default() {
                return first_nonzero_vec_index;
            }
        }
        first_nonzero_vec_index += 1;
    }

    first_nonzero_vec_index
}

fn index_of_first_non_zero_element_in_vec<T: MatrixElementRequiredTraits<T>>(
    input: &Vec<T>,
) -> usize {
    let mut index_of_first_non_zero_element = 0;
    for element in input {
        if *element != T::default() {
            return index_of_first_non_zero_element;
        }
        index_of_first_non_zero_element += 1;
    }

    index_of_first_non_zero_element
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use std::ops::Add;

    use crate::{
        complex_number::ComplexNumber,
        matrix_algebra::{first_non_zero_vec_index, index_of_first_non_zero_element_in_vec},
    };

    use super::{new_all_default, new_identity_matrix, Matrix};

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

        let _ = test_matrix_a * test_matrix_b;
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

        let matrix_sum = test_matrix_a + test_matrix_b;

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

        let matrix_product = test_matrix_a * test_matrix_b;

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
    fn test_new_all_default() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let test_matrix_f64 = new_all_default::<f64>(m, n);

        for entry in test_matrix_f64.entries {
            assert_eq!(entry, f64::default());
        }

        let test_matrix_i32 = new_all_default::<i32>(m, n);

        for entry in test_matrix_i32.entries {
            assert_eq!(entry, i32::default());
        }
    }

    #[test]
    fn test_new_identity_matrix() {
        let mut rng = rand::thread_rng();
        let dimension = rng.gen_range(1..=100);

        let test_matrix_i32 = new_identity_matrix::<i32>(dimension);

        for index in 0..dimension {
            assert_eq!(*test_matrix_i32.get_entry_ij(index, index), 1);
        }

        let test_matrix_f64 = new_identity_matrix::<f64>(dimension);

        for index in 0..dimension {
            assert_eq!(*test_matrix_f64.get_entry_ij(index, index), 1.0);
        }
    }

    #[test]
    fn test_get_entry() {
        let entries = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0,
        ]
        .to_vec();
        let test_matrix = Matrix::new(5, 5, entries);
        let mut index = 1.0;

        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(*test_matrix.get_entry_ij(i, j), index);
                index += 1.0;
            }
        }
    }

    #[test]
    fn test_set_entry() {
        let mut test_matrix = new_all_default::<f64>(5, 5);
        let mut index = 1.0;

        for i in 0..5 {
            for j in 0..5 {
                test_matrix.set_entry_ij(i, j, &index);
                assert_eq!(*test_matrix.get_entry_ij(i, j), index);
                index += 1.0;
            }
        }
    }

    #[test]
    fn test_partition() {
        let test_matrix = Matrix::new(
            5,
            5,
            [
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0,
            ]
            .to_vec(),
        );

        let submatrices = test_matrix.partition(&[3, 2], &[2, 3]);

        assert_eq!(
            submatrices,
            [
                Matrix::new(2, 3, [1.0, 2.0, 3.0, 6.0, 7.0, 8.0].to_vec()),
                Matrix::new(2, 2, [4.0, 5.0, 9.0, 10.0].to_vec()),
                Matrix::new(
                    3,
                    3,
                    [11.0, 12.0, 13.0, 16.0, 17.0, 18.0, 21.0, 22.0, 23.0].to_vec()
                ),
                Matrix::new(3, 2, [14.0, 15.0, 19.0, 20.0, 24.0, 25.0].to_vec())
            ]
        );

        let test_matrix = Matrix::new(
            3,
            4,
            [
                0.0, 1.0, -4.0, -7.0, 0.0, 0.0, 3.0, -1.0, 0.0, 0.0, 3.0, -1.0,
            ]
            .to_vec(),
        );

        let submatrices = test_matrix.partition(&[4], &[1, 2]);

        assert_eq!(
            submatrices,
            [
                Matrix::new(1, 4, [0.0, 1.0, -4.0, -7.0].to_vec()),
                Matrix::new(2, 4, [0.0, 0.0, 3.0, -1.0, 0.0, 0.0, 3.0, -1.0,].to_vec()),
            ]
        );
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

        let submatrix = test_matrix.submatrix(&[1, 3], &[0, 2, 4]);

        assert_eq!(submatrix.n, 2);
        assert_eq!(submatrix.m, 3);
        assert_eq!(submatrix.entries, [2, 4, 12, 14, 22, 24]);
    }

    #[test]
    fn test_row_interchange() {
        let test_matrix = Matrix::new(2, 3, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0].to_vec());

        let result = test_matrix.row_interchange(0, 1);

        assert_eq!(
            result,
            Matrix::new(2, 3, [4.0, 5.0, 6.0, 1.0, 2.0, 3.0].to_vec()),
        );
    }

    #[test]
    fn test_multiply_row_by_scalar() {
        let test_matrix = Matrix::new(3, 3, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].to_vec());

        let result = test_matrix.multiply_row_by_scalar(2, 2.0);

        assert_eq!(
            result,
            Matrix::new(
                3,
                3,
                [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 14.0, 16.0, 18.0].to_vec()
            ),
        );
    }

    #[test]
    fn test_add_row_to_scalar_multiple_of_row() {
        let test_matrix = Matrix::new(3, 3, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].to_vec());

        let result = test_matrix.add_row_to_scalar_multiple_of_row(0, 2, 5.0);

        assert_eq!(
            result,
            Matrix::new(
                3,
                3,
                [36.0, 42.0, 48.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].to_vec()
            ),
        );
    }

    #[test]
    fn test_all_zeroes() {
        let mut entries = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0].to_vec();
        let test_matrix = Matrix::new(3, 4, entries.clone());

        assert_eq!(true, test_matrix.all_zeroes());

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..12);
        entries[random_index] = 1.0;

        let test_matrix = Matrix::new(3, 4, entries.clone());

        assert_eq!(false, test_matrix.all_zeroes());
    }

    #[test]
    fn test_first_non_zero_vec_index() {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..12);
        let mut test_vec: Vec<Vec<i32>> = vec![];
        let zero_vec = [0, 0, 0].to_vec();
        let non_zero_vec = [1, 2, 3].to_vec();

        for i in 0..12 {
            if i == random_index {
                test_vec.push(non_zero_vec.clone());
            } else {
                test_vec.push(zero_vec.clone());
            }
        }

        assert_eq!(random_index, first_non_zero_vec_index(&test_vec));
    }

    #[test]
    fn test_index_of_first_non_zero_element_in_vec() {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..12);
        let mut test_vec: Vec<i32> = vec![];

        for i in 0..12 {
            if i == random_index {
                test_vec.push(1);
            } else {
                test_vec.push(0);
            }
        }

        assert_eq!(
            random_index,
            index_of_first_non_zero_element_in_vec(&test_vec)
        );
    }

    #[test]
    fn test_row_echolon_form() {
        let test_matrix = Matrix::new(
            3,
            4,
            [
                0.0, 0.0, 3.0, -1.0, 0.0, -1.0, 4.0, 7.0, 0.0, -1.0, 7.0, 6.0,
            ]
            .to_vec(),
        );

        let result = test_matrix.row_echolon_form();

        assert_eq!(
            result,
            Matrix::new(
                3,
                4,
                [
                    0.0,
                    1.0,
                    0.0,
                    -25.0 / 3.0,
                    0.0,
                    0.0,
                    1.0,
                    -1.0 / 3.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                ]
                .to_vec()
            ),
        );

        let test_matrix = Matrix::new(
            3,
            3,
            [4.0, -8.0, 16.0, 1.0, -3.0, 6.0, 2.0, 1.0, 1.0].to_vec(),
        );

        let result = test_matrix.row_echolon_form();

        assert_eq!(result, new_identity_matrix(3));

        let test_matrix = Matrix::new(
            3,
            4,
            [0.0, 2.0, 1.0, 4.0, 0.0, 0.0, 2.0, 6.0, 1.0, 0.0, -3.0, 2.0].to_vec(),
        );

        let result = test_matrix.row_echolon_form();

        assert_eq!(
            result,
            Matrix::new(
                3,
                4,
                [1.0, 0.0, 0.0, 11.0, 0.0, 1.0, 0.0, 0.5, 0.0, 0.0, 1.0, 3.0].to_vec()
            )
        );

        let test_matrix = Matrix::new(
            3,
            3,
            [
                ComplexNumber::new(2.0, 0.0),
                ComplexNumber::new(8.0, 2.0),
                ComplexNumber::new(6.0, -6.0),
                ComplexNumber::new(0.0, 1.0),
                ComplexNumber::new(0.0, 5.0),
                ComplexNumber::new(3.0, 3.0),
                ComplexNumber::new(1.0, 2.0),
                ComplexNumber::new(4.0, 11.0),
                ComplexNumber::new(9.0, 3.0),
            ]
            .to_vec(),
        );

        let result = test_matrix.row_echolon_form();

        assert_eq!(
            result,
            Matrix::new(
                3,
                3,
                [
                    ComplexNumber::new(1.0, 0.0),
                    ComplexNumber::new(0.0, 0.0),
                    ComplexNumber::new(3.0, -3.0),
                    ComplexNumber::new(0.0, 0.0),
                    ComplexNumber::new(1.0, 0.0),
                    ComplexNumber::new(0.0, 0.0),
                    ComplexNumber::new(0.0, 0.0),
                    ComplexNumber::new(0.0, 0.0),
                    ComplexNumber::new(0.0, 0.0),
                ]
                .to_vec()
            )
        );

        let test_matrix = Matrix::new(
            3,
            2,
            [
                ComplexNumber::new(1.0, 1.0),
                ComplexNumber::new(1.0, -1.0),
                ComplexNumber::new(2.0, 0.0),
                ComplexNumber::new(2.0, 0.0),
                ComplexNumber::new(1.0, 2.0),
                ComplexNumber::new(2.0, -1.0),
            ]
            .to_vec(),
        );

        let result = test_matrix.row_echolon_form();

        assert_eq!(
            result,
            Matrix::new(
                3,
                2,
                [
                    ComplexNumber::new(1.0, 0.0),
                    ComplexNumber::new(0.0, 0.0),
                    ComplexNumber::new(0.0, 0.0),
                    ComplexNumber::new(1.0, 0.0),
                    ComplexNumber::new(0.0, 0.0),
                    ComplexNumber::new(0.0, 0.0),
                ]
                .to_vec()
            )
        );
    }

    #[test]
    fn test_column_echolon_form() {
        let test_matrix = Matrix::new(2, 3, [1.0, 2.0, 3.0, 2.0, 3.0, 4.0].to_vec());

        let result = test_matrix.column_echolon_form();

        assert_eq!(
            result,
            Matrix::new(2, 3, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0,].to_vec())
        );

        let test_matrix = Matrix::new(3, 3, [1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0].to_vec());

        let result = test_matrix.column_echolon_form();

        assert_eq!(
            result,
            Matrix::new(3, 3, [1.0, 0.0, 0.0, 2.0, 0.0, 0.0, 3.0, 0.0, 0.0].to_vec())
        );
    }

    #[test]
    fn test_determinant() {
        let test_matrix = Matrix::new(
            4,
            4,
            [
                2.0, 2.0, -1.0, 9.0, 4.0, 2.0, 1.0, 17.0, 1.0, -1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 9.0,
            ]
            .to_vec(),
        );

        assert_eq!(test_matrix.determinant(), 0.0);

        let test_matrix = Matrix::new(
            3,
            3,
            [
                ComplexNumber::new(1.0, 0.0),
                ComplexNumber::new(1.0, 0.0),
                ComplexNumber::new(0.0, 1.0),
                ComplexNumber::new(1.0, 1.0),
                ComplexNumber::new(1.0, 1.0),
                ComplexNumber::new(1.0, 0.0),
                ComplexNumber::new(2.0, 3.0),
                ComplexNumber::new(0.0, -1.0),
                ComplexNumber::new(3.0, 0.0),
            ]
            .to_vec(),
        );

        let determinant = test_matrix.determinant();

        fn delta_real(determinant: ComplexNumber<f64>, expectation: f64) -> bool {
            determinant.real - expectation < (1.0 / 1000000000000000000000000000000.0)
        }

        fn delta_complex(determinant: ComplexNumber<f64>, expectation: f64) -> bool {
            determinant.complex - expectation < (1.0 / 1000000000000000000000000000000.0)
        }
        assert!(delta_real(determinant, 8.0));
        assert!(delta_complex(determinant, 6.0));
    }
}
