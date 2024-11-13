pub mod complex_number {
    use std::{
        fmt::Display,
        ops::{Add, AddAssign, Mul, Sub},
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

    fn complex_number_sub<
        T: Add<Output = T> + Sub<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized,
    >(
        lhs: &ComplexNumber<T>,
        rhs: &ComplexNumber<T>,
    ) -> ComplexNumber<T> {
        ComplexNumber {
            real: lhs.real - rhs.real,
            complex: lhs.complex - rhs.complex,
        }
    }

    impl<T: Add<Output = T> + Sub<Output = T> + AddAssign + Clone + Copy + Display + Mul<Output = T> + ?Sized> Sub
        for ComplexNumber<T>
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            complex_number_sub(&self, &other)
        }
    }

    fn complex_number_multiply<T: Add<Output = T> + Sub<Output = T> + AddAssign + Clone + Copy + Display + Mul<Output = T> + ?Sized,
    >(
        lhs: &ComplexNumber<T>,
        rhs: &ComplexNumber<T>,
    ) -> ComplexNumber<T> {
        ComplexNumber {
            real: (lhs.real * rhs.real) - (lhs.complex * rhs.complex),
            complex: (lhs.real * rhs.complex) + (rhs.real * lhs.complex),
        }
    }

    impl<T: Add<Output = T> + Sub<Output = T> + AddAssign + Clone + Copy + Display + Mul<Output = T> + ?Sized> Mul
        for ComplexNumber<T>
    {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            complex_number_multiply::<T>(&self, &rhs)
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

    #[test]
    fn test_complex_number_sub() {
        let mut rng = rand::thread_rng();
        let real_one = rng.gen_range(1..=100);
        let complex_one = rng.gen_range(1..=100);
        let real_two = rng.gen_range(1..=100);
        let complex_two = rng.gen_range(1..=100);

        let lhs = ComplexNumber::new(real_one, complex_one);
        let rhs = ComplexNumber::new(real_two, complex_two);

        assert_eq!(lhs - rhs, ComplexNumber::new(real_one - real_two, complex_one - complex_two));
    }

    #[test]
    fn test_complex_number_multiply() {
        let mut rng = rand::thread_rng();
        let real_one = rng.gen_range(1..=100);
        let complex_one = rng.gen_range(1..=100);
        let real_two = rng.gen_range(1..=100);
        let complex_two = rng.gen_range(1..=100);

        let lhs = ComplexNumber::new(real_one, complex_one);
        let rhs = ComplexNumber::new(real_two, complex_two);

        assert_eq!(lhs * rhs, ComplexNumber::new((real_one * real_two) - (complex_one * complex_two), (complex_one * real_two) + (complex_two * real_one)));
    }
}
