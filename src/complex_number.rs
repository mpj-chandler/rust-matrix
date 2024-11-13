pub struct ComplexNumber<T: Add<Output = T> + AddAssign + Clone + Copy + Display + Mul + ?Sized> {
    real: T,
    complex: T,
}

impl ComplexNumber {
    
}