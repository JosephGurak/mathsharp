



pub mod perimeter {
    
    /// Calculates perimeter of Triangle.
    ///
    /// #Examples
    ///
    /// ```
    /// let side1 = 6.0;
    /// let side2 = 5.0;
    /// let side3 = 5.0;
    /// let answer = mathsharp::perimeter::perimeter_triangle(side1, side2, side3);
    ///
    /// assert_eq!(16.0, answer);
    /// ```

    pub fn perimeter_triangle(side1: f32, side2: f32, side3: f32) -> f32 {
        let perimeter = side1 + side2 + side3;
        perimeter
    }

    pub fn perimeter_equilateral_triangle(side: f32) -> f32 {
        let perimeter = side * 3.0;
        perimeter
    }

    pub fn perimeter_rectangle(length: f32, width: f32) -> f32 {
        let perimeter = 2.0 * (length + width);
        perimeter
    }

    pub fn perimeter_square(side: f32) -> f32 {
        let perimeter = 4.0 * side;
        perimeter
    }

    pub fn perimeter_parallelogram(side_a: f32, side_b: f32) -> f32 {
        let perimeter = 2.0 * (side_a + side_b);
        perimeter
    }

    pub fn perimeter_rhombus(side: f32) -> f32 {
        let perimeter = 4.0 * side;
        perimeter
    }

    pub fn perimeter_trapezoid(side_a: f32, side_b: f32, side_c: f32, side_d: f32) -> f32 {
        let perimeter = side_a + side_b + side_c +  side_d;
        perimeter
    }

    pub fn perimeter_kite(side_a: f32, side_b: f32) -> f32 {
        let perimeter = 2.0 * (side_a + side_b);
        perimeter
    }

    pub fn perimeter_pentagon(side: f32) -> f32 {
        let perimeter = 5.0 * side;
        perimeter
    }

    pub fn perimeter_hexagon(side: f32) -> f32 {
        let perimeter = 6.0 * side;
        perimeter
    }

    pub fn perimeter_circle(radius: f64) -> f64 {
        let perimeter = 2.0 * std::f64::consts::PI * radius;
        perimeter
    }


}

pub mod areas {
    pub fn area_triangle(base: f32, height: f32) -> f32 {
        let area = 0.5 * base * height;
        area
    }

    pub fn area_rectangle(width: f32, height: f32) -> f32 {
        let area = width * height;
        area
    }


    /// Calculates Area of Trapezoid.
    ///
    /// #Examples
    ///
    /// ```
    /// let height = 6.0;
    /// let length_a = 5.0;
    /// let length_b = 5.0;
    /// let answer = mathsharp::areas::area_trapezoid(height, length_a, length_b);
    ///
    /// assert_eq!(30.0, answer);
    /// ```

    pub fn area_trapezoid(height: f32, length_a: f32, length_b: f32) -> f32 {
        let area = (0.5 * (length_a + length_b)) * height;
        area
    }

    pub fn area_sqaure(side: f32) -> f32 {
        let area = side*side;
        area
    }

    pub fn area_parallelogram(base: f32, height: f32) -> f32 {
        let area = base * height;
        area
    }

    pub fn area_circle(radius: f64) -> f64 {
        let area = std::f64::consts::PI * (radius * radius);
        area
    }

    pub fn area_kite(d1: f32, d2: f32) -> f32 {
        let area = 0.5 * d1 * d2;
        area
    }
}

pub mod volumes {
    pub fn volume_cube(side: f32) -> f32 {
        let volume = side * side * side;
        volume
    }

    pub fn volume_rectangular_prism(length: f32, width: f32, height: f32) -> f32 {
        let volume = length * width * height;
        volume
    }

    // pub fn volume_prism() {
        
    // }

    // pub fn volume_pyramid() {
        
    // }

    pub fn volume_cone(radius: f64, height: f64) -> f64 {
        let volume = 0.33333 * std::f64::consts::PI * (radius * radius) * height;
        volume
    }

    pub fn volume_cylinder(radius: f64, height: f64) -> f64 {
        let volume = std::f64::consts::PI * (radius * radius) * height;
        volume
    }

    pub fn volume_sphere(radius: f64) -> f64 {
        let volume = 1.33333 * std::f64::consts::PI * (radius * radius * radius);
        volume
    }
}