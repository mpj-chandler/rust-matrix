pub trait Pow {
    fn powf(&self, n: Self) -> Self;
}

macro_rules! impl_pow {
    ( $($ty:ty),* ) => {
        $(
            impl Pow for $ty {
                fn powf(&self, n: Self) -> Self {
                    self.powf(n)
                }
            }
        )*
    };
}

impl_pow!(f64, f32);
