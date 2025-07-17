


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

        pub fn inverse() {
            
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





    }

    

}