use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MatrixEntry {
    Integer32(i32),
    Float64(f64),
}

impl MatrixEntry {
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

    pub fn to_string(self) -> Option<String> {
        match self {
            Self::Integer32(i) => Some(i.to_string()),
            Self::Float64(i) => Some(i.to_string()),
        }
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
    use crate::matrix_entry::MatrixEntry;

    use rand::prelude::*;
}
