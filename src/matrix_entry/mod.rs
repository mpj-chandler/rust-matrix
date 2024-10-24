use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MatrixEntry {
    Integer32(i32),
    Float64(f64),
}

impl MatrixEntry {
    pub fn to_string(self) -> Option<String> {
        match self {
            Self::Integer32(i) => Some(i.to_string()),
            Self::Float64(i) => Some(i.to_string()),
        }
    }
}

impl Add for MatrixEntry {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        entry_add(&self, &other)
    }
}

impl AddAssign for MatrixEntry {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

pub fn entry_add(a: &MatrixEntry, b: &MatrixEntry) -> MatrixEntry {
    match (a, b) {
        (MatrixEntry::Integer32(x), MatrixEntry::Integer32(y)) => MatrixEntry::Integer32(x + y),
        (MatrixEntry::Float64(x), MatrixEntry::Float64(y)) => MatrixEntry::Float64(x + y),
        (MatrixEntry::Float64(x), MatrixEntry::Integer32(y)) => {
            MatrixEntry::Float64(x * (*y as f64))
        }
        (MatrixEntry::Integer32(x), MatrixEntry::Float64(y)) => {
            MatrixEntry::Float64((*x as f64) * y)
        }
    }
}

pub fn entry_multiply(a: &MatrixEntry, b: &MatrixEntry) -> MatrixEntry {
    match (a, b) {
        (MatrixEntry::Integer32(x), MatrixEntry::Integer32(y)) => MatrixEntry::Integer32(x * y),
        (MatrixEntry::Float64(x), MatrixEntry::Float64(y)) => MatrixEntry::Float64(x * y),
        (MatrixEntry::Float64(x), MatrixEntry::Integer32(y)) => {
            MatrixEntry::Float64(x * (*y as f64))
        }
        (MatrixEntry::Integer32(x), MatrixEntry::Float64(y)) => {
            MatrixEntry::Float64((*x as f64) * y)
        }
    }
}

impl Mul for MatrixEntry {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        entry_multiply(&self, &rhs)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::PartialOrd;
    use std::ops::RangeInclusive;

    use crate::{matrix_algebra::Matrix, matrix_entry::MatrixEntry};

    use rand::{
        distributions::uniform::{SampleRange, SampleUniform},
        prelude::*,
    };

    fn generate_random_entries<T>() -> (T, T)
    where
        T: SampleUniform + PartialOrd,
        RangeInclusive<i32>: SampleRange<T>,
    {
        let mut rng = rand::thread_rng();
        let test_value_one: T = rng.gen_range::<T, RangeInclusive<i32>>(1..=100);
        let test_value_two: T = rng.gen_range::<T, RangeInclusive<i32>>(1..=100);

        (test_value_one as T, test_value_two as T)
    }

    #[test]
    fn test_matrix_entry_add() {
        let (test_value_one, test_value_two) = generate_random_entries::<i32>();
        let test_entry_one = MatrixEntry::Integer32(test_value_one);
        let test_entry_two = MatrixEntry::Integer32(test_value_two);
        let result = test_entry_one + test_entry_two;

        assert_eq!(
            result,
            MatrixEntry::Integer32(test_value_one + test_value_two)
        );

        let (test_value_one, test_value_two) = generate_random_entries::<f64>();
        let test_entry_one = MatrixEntry::Float64(test_value_one);
        let test_entry_two = MatrixEntry::Float64(test_value_two);
        let result = test_entry_one + test_entry_two;

        assert_eq!(
            result,
            MatrixEntry::Float64(test_value_one + test_value_two)
        );
    }

    #[test]
    fn test_matrix_entry_multiply() {
        let (test_value_one, test_value_two) = generate_random_entries::<i32>();
        let test_entry_one = MatrixEntry::Integer32(test_value_one);
        let test_entry_two = MatrixEntry::Integer32(test_value_two);
        let result = test_entry_one * test_entry_two;

        assert_eq!(
            result,
            MatrixEntry::Integer32(test_value_one * test_value_two)
        );
    }
}
