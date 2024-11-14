use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Mul, Sub},
};

trait ComplexNumberRequiredTraits<T>: Add<Output = T>
+ Sub<Output = T>
+ AddAssign
+ Clone
+ Copy
+ Display
+ Mul<Output = T> {}

impl<T: Add<Output = T>
+ Sub<Output = T>
+ AddAssign
+ Clone
+ Copy
+ Display
+ Mul<Output = T>
+ ?Sized,> ComplexNumberRequiredTraits<T> for T {}

#[derive(PartialEq, Debug, PartialOrd, Copy, Clone)]
pub struct ComplexNumber<
    T: Add<Output = T>
        + Sub<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Display
        + Mul<Output = T>
        + ?Sized,
> {
    real: T,
    complex: T,
}

impl<
        T: Add<Output = T>
            + Sub<Output = T>
            + AddAssign
            + Clone
            + Copy
            + Display
            + Mul<Output = T>
            + ?Sized,
    > ComplexNumber<T>
{
    pub fn new(real: T, complex: T) -> ComplexNumber<T> {
        ComplexNumber { real, complex }
    }
}

fn complex_number_add<
    T: Add<Output = T>
        + Sub<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Display
        + Mul<Output = T>
        + ?Sized,
>(
    lhs: &ComplexNumber<T>,
    rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    ComplexNumber {
        real: lhs.real + rhs.real,
        complex: lhs.complex + rhs.complex,
    }
}

impl<
        T: Add<Output = T>
            + Sub<Output = T>
            + AddAssign
            + Clone
            + Copy
            + Display
            + Mul<Output = T>
            + ?Sized,
    > Add for ComplexNumber<T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        complex_number_add(&self, &other)
    }
}

fn complex_number_sub<
    T: Add<Output = T>
        + Sub<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Display
        + Mul<Output = T>
        + ?Sized,
>(
    lhs: &ComplexNumber<T>,
    rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    ComplexNumber {
        real: lhs.real - rhs.real,
        complex: lhs.complex - rhs.complex,
    }
}

impl<
        T: Add<Output = T>
            + Sub<Output = T>
            + AddAssign
            + Clone
            + Copy
            + Display
            + Mul<Output = T>
            + ?Sized,
    > Sub for ComplexNumber<T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        complex_number_sub(&self, &other)
    }
}

fn complex_number_multiply<
    T: Add<Output = T>
        + Sub<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Display
        + Mul<Output = T>
        + ?Sized,
>(
    lhs: &ComplexNumber<T>,
    rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    ComplexNumber {
        real: (lhs.real * rhs.real) - (lhs.complex * rhs.complex),
        complex: (lhs.real * rhs.complex) + (rhs.real * lhs.complex),
    }
}

impl<
        T: Add<Output = T>
            + Sub<Output = T>
            + AddAssign
            + Clone
            + Copy
            + Display
            + Mul<Output = T>
            + ?Sized,
    > Mul for ComplexNumber<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        complex_number_multiply::<T>(&self, &rhs)
    }
}

fn complex_number_add_assign<
T: Add<Output = T>
    + Sub<Output = T>
    + AddAssign
    + Clone
    + Copy
    + Display
    + Mul<Output = T>
    + ?Sized,
>(
lhs: &ComplexNumber<T>,
rhs: &ComplexNumber<T>,
) -> ComplexNumber<T> {
    complex_number_add(lhs, rhs)
}

impl<
        T: Add<Output = T>
            + Sub<Output = T>
            + AddAssign
            + Clone
            + Copy
            + Display
            + Mul<Output = T>
            + ?Sized,
    > AddAssign for ComplexNumber<T>
{
    fn add_assign(&mut self, rhs: Self) {
        *self = complex_number_add_assign::<T>(self, &rhs);
    }
}

fn format_complex_number_component<
    T: Add<Output = T>
        + Sub<Output = T>
        + AddAssign
        + Clone
        + Copy
        + Display
        + PartialOrd<i32>
        + Mul<Output = T>
        + ?Sized,
>(
    input: &T,
    prefix: bool,
) -> String {
    if *input == 0 {
        return "".to_owned();
    } else if prefix && *input > 0 {
        return "+".to_owned() + &input.to_string();
    } else {
        input.to_string()
    }
}

impl<
        T: Add<Output = T>
            + Sub<Output = T>
            + AddAssign
            + Clone
            + Copy
            + Display
            + PartialOrd<i32>
            + Mul<Output = T>
            + ?Sized,
    > fmt::Display for ComplexNumber<T>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let display_string = format_complex_number_component(&self.real, false)
            + " "
            + &format_complex_number_component(&self.complex, true)
            + "i";
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
}
