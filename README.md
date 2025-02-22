# matrix

This crate is a Rust implementation of some foundational matrix algebra
functionality.

While technically generic, the matrix library is only implemented for
the \<f32\> and \<f64\> data types, as well as complex numbers (with real and
imaginary components of the same \<f32\> / \<f64\> types), for which I have
written a dedicated module.

Users could, for instance make this library work for {integer} types,
although there's not a huge amount of point as you would soon get into
rounding issues, particular with required traits such as `Sqrt`.

This library is very early in its development. All implementations are
naive with no optimisations in the form of quicker algorithms or multi-
threading, which will be added in due course.

Any and all feedback is welcomed.