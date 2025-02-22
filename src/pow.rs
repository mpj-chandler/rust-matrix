/// A trait created to represent raising an element
/// to a given power.
/// This explicitly relies on the `powf` trait
/// and thus will only work for types that implement
/// that trait
pub trait Pow {
    fn pow(&self, n: Self) -> Self;
}

macro_rules! impl_pow {
    ( $($ty:ty),* ) => {
        $(
            impl Pow for $ty {
                fn pow(&self, n: Self) -> Self {
                    self.powf(n)
                }
            }
        )*
    };
}

impl_pow!(f64, f32);
