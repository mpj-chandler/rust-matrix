pub mod complex_number {
    use std::{
        fmt::Display,
        ops::{Add, AddAssign, Mul},
    };

    #[derive(PartialEq, Debug)]
    pub struct ComplexNumber<T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized> {
        real: T,
        complex: T,
    }

    impl<
            T: Add<Output = T> + Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized,
        > ComplexNumber<T>
    {
        pub fn new(real: T, complex: T) -> ComplexNumber<T> {
            ComplexNumber { real, complex }
        }
    }

    fn complex_number_add<
        T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized,
    >(
        lhs: &ComplexNumber<T>,
        rhs: &ComplexNumber<T>,
    ) -> ComplexNumber<T> {
        ComplexNumber {
            real: lhs.real + rhs.real,
            complex: lhs.complex + rhs.complex,
        }
    }

    impl<T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized> Add
        for ComplexNumber<T>
    {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            complex_number_add(&self, &other)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::complex_number::complex_number::ComplexNumber;
    use rand::prelude::*;

    #[test]
    fn test_complex_number_add() {
        let mut rng = rand::thread_rng();
        let real_one = rng.gen_range(1..=100);
        let complex_one = rng.gen_range(1..=100);
        let real_two = rng.gen_range(1..=100);
        let complex_two = rng.gen_range(1..=100);

        let lhs = ComplexNumber::new(real_one, complex_one);
        let rhs = ComplexNumber::new(real_two, complex_two);

        assert_eq!(lhs + rhs, ComplexNumber::new(real_one + real_two, complex_one + complex_two));
    }
}
