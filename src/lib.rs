pub mod matrix_algebra {
    use std::fmt;

    #[derive(Clone)]
    pub struct Matrix {
        pub n: usize,
        pub m: usize,
        pub entries: Vec<Vec<i32>>,
    }

    impl Matrix {
        pub fn new(m: usize, n: usize, value: i32) -> Matrix {
            if m == 0 || n == 0 {
                panic!("Matrix dimensions must each be greater than zero!");
            }
            let row_vec = vec![value; n];
            Matrix {
                n: n,
                m: m,
                entries: vec![row_vec; m],
            }
        }

        pub fn columns(self) -> Vec<Vec<i32>> {
            let n = self.n;
            let m = self.m;

            let mut columns = vec![vec![0; m]; n];
            for row_index in 0..m {
                for column_index in 0..n {
                    columns[column_index][row_index] = self.entries[row_index][column_index];
                }
            }

            columns
        }
    }

    impl fmt::Display for Matrix {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            for row in &self.entries {
                let printable_row: String =
                    row.iter().map(|&entry| entry.to_string() + ", ").collect();
                let _ = fmt.write_str(&printable_row)?;
                let _ = fmt.write_str("\n");
            }
            Ok(())
        }
    }

    fn is_multiplicatively_conformable(a: Matrix, b: Matrix) -> bool {
        let n = a.entries[0].len();
        let n_prime = b.entries.len();

        n == n_prime
    }

    pub fn matrix_multiply(a: Matrix, b: Matrix) -> Matrix {
        if !is_multiplicatively_conformable(a.clone(), b.clone()) {
            panic!("Matrices are not multiplicatively conformable!");
        }

        let mut c = Matrix::new(b.entries[0].len(), a.entries.len(), 0);

        for (i, a_row) in a.entries.iter().enumerate() {
            for (k, a_ik) in a_row.iter().enumerate() {
                for (j, b_kj) in b.entries[k].iter().enumerate() {
                    println!("C[{}{}] = A[{}{}] * B[{}{}]", i, j, i, k, k, j);
                    println!("C[{}{}] = {} * {}", i, j, a_ik, b_kj);
                    c.entries[i][j] = a_ik * b_kj;
                }
            }
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix_algebra::matrix_multiply;
    use crate::matrix_algebra::Matrix;

    use rand::prelude::*;

    #[test]
    fn test_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let mut entries: Vec<Vec<i32>> = vec![];

        for _i in 0..m {
            let mut row: Vec<i32> = vec![];
            for _j in 0..n {
                row.push(rng.gen_range(1..=100));
            }
            entries.push(row);
        }

        let test_matrix = Matrix { m, n, entries };

        assert_eq!(test_matrix.entries.len(), m);

        for row in test_matrix.entries {
            assert_eq!(row.len(), n);
        }
    }

    #[test]
    fn test_convenience_initialiser() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1..=100);
        let m = rng.gen_range(1..=100);
        let test_matrix = Matrix::new(m, n, 0);

        assert_eq!(test_matrix.entries.len(), m);

        for row in test_matrix.entries {
            assert_eq!(row.len(), n);
        }
    }

    #[test]
    #[should_panic]
    fn test_zero_first_argument_to_initialiser() {
        let _test_matrix = Matrix::new(0, 1, 1);
    }

    #[test]
    #[should_panic]
    fn test_zero_second_argument_to_initialiser() {
        let _test_matrix = Matrix::new(1, 0, 1);
    }

    #[test]
    #[should_panic]
    fn test_panic_on_non_multiplicatively_conformable_matrices() {
        let test_matrix_a = Matrix::new(3, 4, 5);
        let test_matrix_b = Matrix::new(5, 7, 4);

        matrix_multiply(test_matrix_a, test_matrix_b);
    }

    #[test]
    fn test_matrix_multiply() {
        let test_matrix_a = Matrix::new(3, 4, 5);
        let test_matrix_b = Matrix::new(4, 3, 4);

        println!("{} \n{}", test_matrix_a, test_matrix_b);

        let matrix_product = matrix_multiply(test_matrix_a, test_matrix_b);

        assert_eq!(matrix_product.entries.len(), 3);

        assert_eq!(matrix_product.entries[0], [20, 20, 20]);
        assert_eq!(matrix_product.entries[1], [20, 20, 20]);
        assert_eq!(matrix_product.entries[2], [20, 20, 20]);
    }
}
