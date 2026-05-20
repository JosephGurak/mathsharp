

// possible alternative having a vector with enum to have different types 
pub mod m2x2 {
    // determinant
    // transpose
    // inverse
    // matrix operations of addition, subtraction, multiplication

    #[derive(Debug)]
    pub struct M2x2 {
        r1: [f64; 2], // using fixed arrays for memory efficiency and can use iterative methods
        r2: [f64; 2]
    }

    impl M2x2 {
        pub fn new(r1: [f64; 2], r2: [f64; 2] ) -> Self {
            Self {
                r1,
                r2
            }
        }


        pub fn determinant(&self) -> f64 {
            (self.r1[0] * self.r2[1]) - (self.r1[1] * self.r2[0])
        }

        pub fn transpose(&mut self) -> M2x2 {
            M2x2 { r1: [self.r1[0], self.r2[0]], r2: [self.r1[1], self.r2[1]] }
        }


        // 
        pub fn inverse(&mut self) -> M2x2 { // return a struct
            // adj = 1 / determ  
            // multiply adj by [d -b]  to get inverse matrix 
            //                 [-c a]

            let adj = 1.0/self.determinant(); 
            println!("{adj}"); // for testing purposes

            M2x2 
            {
                r1: [ adj *self.r2[1], adj*(-1.0 *self.r1[1])], 
                r2: [adj*(-1.0*self.r2[0]), adj*self.r1[0]]
            }          


        }

        pub fn matrix_addition(&mut self, m2: M2x2) -> M2x2 {
           M2x2 
            {   r1: [self.r1[0] + m2.r1[0], self.r1[1] + m2.r1[1]], 
                r2: [self.r2[0] + m2.r2[0], self.r2[1] + m2.r2[1]]        
            }
        }

        pub fn matrix_subtraction(&mut self, m2: M2x2) -> M2x2 {
           M2x2 
            {   r1: [self.r1[0] - m2.r1[0], self.r1[1] - m2.r1[1]], 
                r2: [self.r2[0] - m2.r2[0], self.r2[1] - m2.r2[1]]        
            }
        }

        pub fn matrix_multiplication(&mut self, m2: M2x2) -> M2x2 {
            M2x2 
            {
                 r1: [(self.r1[0] * m2.r1[0]) + (self.r1[1] * m2.r2[0]), (self.r1[0] * m2.r1[1]) + (self.r1[1] * m2.r2[1])],
                 r2: [(self.r2[0] * m2.r1[0]) + (self.r2[1] * m2.r2[0]), (self.r2[0] * m2.r1[1]) + (self.r2[1] * m2.r2[1])] 
            }
        }


    }



}


pub mod m3x3 {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct M3x3 {
        r1: [f64; 3],
        r2: [f64; 3],
        r3: [f64; 3],
    }

    impl M3x3 {
        pub fn new(r1: [f64; 3], r2: [f64; 3], r3: [f64; 3]) -> Self {
            Self { r1, r2, r3 }
        }

        pub fn determinant(&self) -> f64 {
            (self.r1[0] * self.r2[1] * self.r3[2])
                + (self.r1[1] * self.r2[2] * self.r3[0])
                + (self.r1[2] * self.r2[0] * self.r3[1])
                - (self.r1[2] * self.r2[1] * self.r3[0])
                - (self.r1[0] * self.r2[2] * self.r3[1])
                - (self.r1[1] * self.r2[0] * self.r3[2])
        }

        pub fn transpose(&self) -> M3x3 {
            M3x3 {
                r1: [self.r1[0], self.r2[0], self.r3[0]],
                r2: [self.r1[1], self.r2[1], self.r3[1]],
                r3: [self.r1[2], self.r2[2], self.r3[2]],
            }
        }

        /// Returns the inverse using the adjugate matrix divided by the determinant.
        /// Returns `None` when the determinant is zero.
        pub fn inverse(&self) -> Option<M3x3> {
            let det = self.determinant();
            if det == 0.0 {
                return None;
            }

            let a = self.r1[0];
            let b = self.r1[1];
            let c = self.r1[2];
            let d = self.r2[0];
            let e = self.r2[1];
            let f = self.r2[2];
            let g = self.r3[0];
            let h = self.r3[1];
            let i = self.r3[2];

            let c00 = e * i - f * h;
            let c01 = f * g - d * i;
            let c02 = d * h - e * g;
            let c10 = c * h - b * i;
            let c11 = a * i - c * g;
            let c12 = b * g - a * h;
            let c20 = b * f - c * e;
            let c21 = c * d - a * f;
            let c22 = a * e - b * d;

            let inv_det = 1.0 / det;
            Some(M3x3 {
                r1: [c00 * inv_det, c10 * inv_det, c20 * inv_det],
                r2: [c01 * inv_det, c11 * inv_det, c21 * inv_det],
                r3: [c02 * inv_det, c12 * inv_det, c22 * inv_det],
            })
        }

        pub fn matrix_addition(&self, other: M3x3) -> M3x3 {
            M3x3 {
                r1: [
                    self.r1[0] + other.r1[0],
                    self.r1[1] + other.r1[1],
                    self.r1[2] + other.r1[2],
                ],
                r2: [
                    self.r2[0] + other.r2[0],
                    self.r2[1] + other.r2[1],
                    self.r2[2] + other.r2[2],
                ],
                r3: [
                    self.r3[0] + other.r3[0],
                    self.r3[1] + other.r3[1],
                    self.r3[2] + other.r3[2],
                ],
            }
        }

        pub fn matrix_subtraction(&self, other: M3x3) -> M3x3 {
            M3x3 {
                r1: [
                    self.r1[0] - other.r1[0],
                    self.r1[1] - other.r1[1],
                    self.r1[2] - other.r1[2],
                ],
                r2: [
                    self.r2[0] - other.r2[0],
                    self.r2[1] - other.r2[1],
                    self.r2[2] - other.r2[2],
                ],
                r3: [
                    self.r3[0] - other.r3[0],
                    self.r3[1] - other.r3[1],
                    self.r3[2] - other.r3[2],
                ],
            }
        }

        pub fn matrix_multiplication(&self, other: M3x3) -> M3x3 {
            M3x3 {
                r1: [
                    (self.r1[0] * other.r1[0])
                        + (self.r1[1] * other.r2[0])
                        + (self.r1[2] * other.r3[0]),
                    (self.r1[0] * other.r1[1])
                        + (self.r1[1] * other.r2[1])
                        + (self.r1[2] * other.r3[1]),
                    (self.r1[0] * other.r1[2])
                        + (self.r1[1] * other.r2[2])
                        + (self.r1[2] * other.r3[2]),
                ],
                r2: [
                    (self.r2[0] * other.r1[0])
                        + (self.r2[1] * other.r2[0])
                        + (self.r2[2] * other.r3[0]),
                    (self.r2[0] * other.r1[1])
                        + (self.r2[1] * other.r2[1])
                        + (self.r2[2] * other.r3[1]),
                    (self.r2[0] * other.r1[2])
                        + (self.r2[1] * other.r2[2])
                        + (self.r2[2] * other.r3[2]),
                ],
                r3: [
                    (self.r3[0] * other.r1[0])
                        + (self.r3[1] * other.r2[0])
                        + (self.r3[2] * other.r3[0]),
                    (self.r3[0] * other.r1[1])
                        + (self.r3[1] * other.r2[1])
                        + (self.r3[2] * other.r3[1]),
                    (self.r3[0] * other.r1[2])
                        + (self.r3[1] * other.r2[2])
                        + (self.r3[2] * other.r3[2]),
                ],
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn identity() -> M3x3 {
            M3x3::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
        }

        #[test]
        fn determinant_of_identity() {
            assert!((identity().determinant() - 1.0).abs() < 1e-6);
        }

        #[test]
        fn transpose_swaps_rows_and_columns() {
            let m = M3x3::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]);
            let t = m.transpose();
            assert_eq!(t.r1[0], 1.0);
            assert_eq!(t.r1[1], 4.0);
            assert_eq!(t.r2[0], 2.0);
            assert_eq!(t.r3[2], 9.0);
        }

        #[test]
        fn inverse_times_matrix_is_identity() {
            let m = M3x3::new([2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]);
            let inv = m.inverse().unwrap();
            let product = m.matrix_multiplication(inv);
            assert!((product.r1[0] - 1.0).abs() < 1e-5);
            assert!(product.r1[1].abs() < 1e-5);
            assert!(product.r2[0].abs() < 1e-5);
            assert!((product.r2[1] - 1.0).abs() < 1e-5);
            assert!((product.r3[2] - 1.0).abs() < 1e-5);
        }

        #[test]
        fn singular_matrix_has_no_inverse() {
            let m = M3x3::new([1.0, 2.0, 3.0], [2.0, 4.0, 6.0], [1.0, 1.0, 1.0]);
            assert_eq!(m.determinant(), 0.0);
            assert!(m.inverse().is_none());
        }

        #[test]
        fn matrix_addition_and_subtraction() {
            let a = M3x3::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]);
            let b = identity();
            let sum = a.matrix_addition(b);
            assert_eq!(sum.r1, [2.0, 2.0, 3.0]);
            let diff = sum.matrix_subtraction(b);
            assert_eq!(diff, a);
        }

        #[test]
        fn matrix_multiplication() {
            let a = M3x3::new([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
            let b = M3x3::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
            let product = a.matrix_multiplication(b);
            assert_eq!(product, a);
        }
    }
}