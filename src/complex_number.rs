use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub},
};

pub trait ComplexNumberRequiredTraits<T>:
    Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
    + AddAssign
    + MulAssign
    + Clone
    + Copy
    + Display
    + PartialOrd<T>
    + Neg<Output = T>
    + Default
    + From<u8>
{
}

impl<
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + AddAssign
            + MulAssign
            + Clone
            + Copy
            + Display
            + PartialOrd<T>
            + Neg<Output = T>
            + Default
            + From<u8>
            + ?Sized,
    > ComplexNumberRequiredTraits<T> for T
{
}

#[derive(PartialEq, Debug, PartialOrd, Copy, Clone)]
pub struct ComplexNumber<T: ComplexNumberRequiredTraits<T>> {
    real: T,
    complex: T,
}

impl<T: ComplexNumberRequiredTraits<T>> ComplexNumber<T> {
    pub fn new(real: T, complex: T) -> ComplexNumber<T> {
        ComplexNumber { real, complex }
    }

    pub fn complex_conjugate(&self) -> Self {
        ComplexNumber {
            real: self.real,
            complex: -self.complex,
        }
    }
}

impl<T: ComplexNumberRequiredTraits<T>> Neg for ComplexNumber<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ComplexNumber {
            real: -self.real,
            complex: -self.complex,
        }
    }
}

impl<T: ComplexNumberRequiredTraits<T>> Default for ComplexNumber<T> {
    fn default() -> Self {
        Self {
            real: Default::default(),
            complex: Default::default(),
        }
    }
}

fn complex_number_add<T: ComplexNumberRequiredTraits<T>>(
    lhs: &ComplexNumber<T>,
    rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    ComplexNumber {
        real: lhs.real + rhs.real,
        complex: lhs.complex + rhs.complex,
    }
}

impl<T: ComplexNumberRequiredTraits<T>> Add for ComplexNumber<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        complex_number_add(&self, &other)
    }
}

impl<T: ComplexNumberRequiredTraits<T>> From<u8> for ComplexNumber<T> {
    fn from(value: u8) -> Self {
        ComplexNumber {
            real: T::from(value),
            complex: T::from(0),
        }
    }
}

fn complex_number_sub<T: ComplexNumberRequiredTraits<T>>(
    lhs: &ComplexNumber<T>,
    rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    ComplexNumber {
        real: lhs.real - rhs.real,
        complex: lhs.complex - rhs.complex,
    }
}

impl<T: ComplexNumberRequiredTraits<T>> Sub for ComplexNumber<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        complex_number_sub(&self, &other)
    }
}

fn complex_number_multiply<T: ComplexNumberRequiredTraits<T>>(
    lhs: &ComplexNumber<T>,
    rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    ComplexNumber {
        real: (lhs.real * rhs.real) - (lhs.complex * rhs.complex),
        complex: (lhs.real * rhs.complex) + (rhs.real * lhs.complex),
    }
}

impl<T: ComplexNumberRequiredTraits<T>> Mul for ComplexNumber<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        complex_number_multiply::<T>(&self, &rhs)
    }
}

fn complex_number_divide<T: ComplexNumberRequiredTraits<T>>(
    lhs: &ComplexNumber<T>,
    rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    let rhs_complex_conjugate = rhs.complex_conjugate();
    let numerator = *lhs * rhs_complex_conjugate;
    let denominator = *rhs * rhs_complex_conjugate;

    if denominator.complex != T::default() {
        panic!(
            "Something went wrong. Denominator has complex element: {}",
            denominator
        );
    }

    ComplexNumber {
        real: numerator.real / denominator.real,
        complex: numerator.complex / denominator.real,
    }
}

impl<T: ComplexNumberRequiredTraits<T>> Div for ComplexNumber<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        complex_number_divide::<T>(&self, &rhs)
    }
}

fn complex_number_add_assign<T: ComplexNumberRequiredTraits<T>>(
    lhs: &ComplexNumber<T>,
    rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    complex_number_add(lhs, rhs)
}

impl<T: ComplexNumberRequiredTraits<T>> AddAssign for ComplexNumber<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = complex_number_add_assign::<T>(self, &rhs);
    }
}

fn complex_number_mul_assign<T: ComplexNumberRequiredTraits<T>>(
    lhs: &ComplexNumber<T>,
    rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    complex_number_multiply(lhs, rhs)
}

impl<T: ComplexNumberRequiredTraits<T>> MulAssign for ComplexNumber<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = complex_number_mul_assign::<T>(self, &rhs)
    }
}

fn format_complex_number_component<T: ComplexNumberRequiredTraits<T>>(
    input: &T,
    complex: bool,
) -> String {
    if *input == T::default() {
        return "".to_owned();
    } else if complex {
        if *input > T::default() {
            return "+".to_owned() + &input.to_string() + "i";
        } else {
            return input.to_string() + "i";
        }
    } else {
        input.to_string()
    }
}

impl<T: ComplexNumberRequiredTraits<T>> fmt::Display for ComplexNumber<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let display_string = format_complex_number_component(&self.real, false)
            + " "
            + &format_complex_number_component(&self.complex, true);
        let _ = fmt.write_str(&display_string);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::complex_number::ComplexNumber;
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

        assert_eq!(
            lhs + rhs,
            ComplexNumber::new(real_one + real_two, complex_one + complex_two)
        );
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

        assert_eq!(
            lhs - rhs,
            ComplexNumber::new(real_one - real_two, complex_one - complex_two)
        );
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

        assert_eq!(
            lhs * rhs,
            ComplexNumber::new(
                (real_one * real_two) - (complex_one * complex_two),
                (complex_one * real_two) + (complex_two * real_one)
            )
        );
    }

    #[test]
    fn test_complex_conjugate() {
        let mut rng = rand::thread_rng();
        let real = rng.gen_range(1..=100);
        let complex = rng.gen_range(1..=100);
        let complex_number = ComplexNumber::new(real, complex);

        assert_eq!(
            complex_number.complex_conjugate(),
            ComplexNumber::new(real, -complex,)
        );
    }

    #[test]
    fn test_complex_number_divide() {
        let mut rng = rand::thread_rng();
        let real_one = rng.gen_range(1.0..=100.0);
        let complex_one = rng.gen_range(1.0..=100.0);
        let real_two = rng.gen_range(1.0..=100.0);
        let complex_two = rng.gen_range(1.0..=100.0);

        let lhs = ComplexNumber::new(real_one, complex_one);
        let rhs = ComplexNumber::new(real_two, complex_two);

        assert_eq!(
            lhs / rhs,
            ComplexNumber::new(
                ((real_one * real_two) + (complex_one * complex_two))
                    / (real_two * real_two + complex_two * complex_two),
                ((complex_one * real_two) - (complex_two * real_one))
                    / (real_two * real_two + complex_two * complex_two),
            )
        );
    }
}
