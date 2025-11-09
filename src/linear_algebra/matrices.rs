

// possible alternative having a vector with enum to have different types 
pub mod m2x2 {
    // determinant
    // transpose
    // inverse
    // matrix operations of addition, subtraction, multiplication

    #[derive(Debug)]
    pub struct M2x2 {
        r1: [f32; 2], // using fixed arrays for memory efficiency and can use iterative methods
        r2: [f32; 2]
    }

    impl M2x2 {
        pub fn new(r1: [f32; 2], r2: [f32; 2] ) -> Self {
            Self {
                r1,
                r2
            }
        }


        pub fn determinant(&self) -> f32 {
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
    // determinant
    // transpose
    // inverse
    // matrix operations of addition, subtraction, multiplication

    #[derive(Debug)] 
    struct M3x3 {
        r1: [f32; 3],
        r2: [f32; 3],
        r3: [f32; 3],
    }


    impl M3x3 {
        pub fn new(r1: [f32; 3], r2: [f32; 3], r3: [f32; 3] ) -> Self {
            Self {
                r1,
                r2,
                r3
            }
        }

        // using shortcut method here
        pub fn determinant(&self) -> f32 {
            (
                (self.r1[0] * self.r2[1] * self.r3[2]) + (self.r1[1]*self.r2[2]*self.r3[0]) + (self.r1[2]*self.r2[0]*self.r3[1])
            ) - (
                (self.r1[2]*self.r2[1]*self.r3[0])+(self.r1[0]*self.r2[2]*self.r3[1])+(self.r1[1]*self.r2[0]*self.r3[2])
            )
        }




    }

    

}