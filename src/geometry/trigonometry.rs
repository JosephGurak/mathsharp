

pub mod trig {  

    // add in generics to make type flexible 
    // look into how best to construct the impl, assicated functions, constructor fns for triangle
    // go back and refactor other parts of code for generics 

    #[derive(Debug)]
    enum TriangleDetails {
        TwoAngles(f32, f32),
        ThreeAngles(f32, f32, f32),
        TwoSides(f32, f32),
        ThreeSides(f32, f32, f32),
    }

  
    #[derive(Debug)]
    pub struct RightTriangle {
        details: TriangleDetails,
    }

    impl RightTriangle {
       
        pub fn two_angles(angle1: f32, angle2: f32) -> Self {
            Self {
                details: TriangleDetails::TwoAngles(angle1, angle2)
            }
        }

        pub fn two_sides(side1: f32, side2: f32) -> Self {
            Self {
                details: TriangleDetails::TwoSides(side1, side2)
            }
        }

        pub fn remaining_degrees(&self) -> f32 {
            match self.details {
                TriangleDetails::TwoAngles(angle1, angle2 ) => 180.0 - angle1 - angle2,
                _ => panic!("Error this Associated fn for RightTriangle only accepts TriangleDetails::TwoAngles variant")
            }
        }


        // figure out good way to call this along with how to pass in different sides
        pub fn pythagorean_theorem(&self) -> f32 {
            868.0
        }
    }


    #[derive(Debug)]
    struct ScaleneTriangle {

    }

    #[derive(Debug)]
    struct AcuteTriangle {

    }

    #[derive(Debug)]
    struct IsoscelesTriangle {

    }

    #[derive(Debug)]
    struct EquilateralTriangle {

    }

    #[derive(Debug)]
    struct ObtuseTriangle {

    }

}