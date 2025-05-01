

pub mod trig {  

    // add in generics to make type flexible 
    // look into how best to construct the impl, assicated functions, constructor fns for triangle
    // go back and refactor other parts of code for generics 
    enum TriangleDetails {
        TwoAngles(f32, f32),
        TwoSides(f32, f32),
        ThreeSides(f32, f32, f32),
    }

  
    pub struct Triangle {
        details: TriangleDetails,
    }

    impl Triangle {
        // pub fn new(angle1: f32, angle2: f32) -> Self {
        //     Self {
        //         angle1,
        //         angle2,
        //     }
        // }


        pub fn remaining_degrees(&self) -> f32 {
            let sides = self.angle1 + self.angle2;
            180.0 - sides 
        }
    }

}