/// A trait created to represent types that expose a
/// square root method. Relies explicitly on `sqrt`
pub trait Sqrt {
    fn square_root(&self) -> Self;
}

macro_rules! impl_sqrt {
    ( $($ty:ty),* ) => {
        $(
            impl Sqrt for $ty {
                fn square_root(&self) -> Self {
                    self.sqrt()
                }
            }
        )*
    };
}

impl_sqrt!(f64, f32);
