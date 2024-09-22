use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Entry {
    Integer32(i32),
    Float64(f64),
}

impl Entry {
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
