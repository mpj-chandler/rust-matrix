//! This crate is a Rust implementation of some foundational matrix algebra
//! functionality.
//!
//! While technically generic, the matrix library is only implemented for
//! the \<f32\> and \<f64\> data types, as well as complex numbers (with real and
//! imaginary components of the same \<f32\> / \<f64\> types), for which I have
//! written a dedicated module.
//!
//! Users could, for instance make this library work for {integer} types,
//! although there's not a huge amount of point as you would soon get into
//! rounding issues, particular with required traits such as `Sqrt`
//!

/// An implementation of `MatrixElementRequiredTraits<T>` for complex numbers
pub mod complex_number;
/// The core of the crate, contains the Matrix struct and all Matrix operations
pub mod matrix_algebra;
pub mod pow;
pub mod sqrt;
