

pub mod perimeter {

    /// Calculates the perimeter of a triangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_triangle(3.0, 4.0, 5.0);
    /// assert_eq!(result, 12.0);
    /// ```
    pub fn perimeter_triangle(side1: f64, side2: f64, side3: f64) -> f64 {
        let perimeter = side1 + side2 + side3;
        perimeter
    }

    /// Calculates the perimeter of an equilateral triangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_equilateral_triangle(5.0);
    /// assert_eq!(result, 15.0);
    /// ```
    pub fn perimeter_equilateral_triangle(side: f64) -> f64 {
        let perimeter = side * 3.0;
        perimeter
    }

    /// Calculates the perimeter of a rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_rectangle(4.0, 3.0);
    /// assert_eq!(result, 14.0);
    /// ```
    pub fn perimeter_rectangle(length: f64, width: f64) -> f64 {
        let perimeter = 2.0 * (length + width);
        perimeter
    }

    /// Calculates the perimeter of a square.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_square(5.0);
    /// assert_eq!(result, 20.0);
    /// ```
    pub fn perimeter_square(side: f64) -> f64 {
        let perimeter = 4.0 * side;
        perimeter
    }

    /// Calculates the perimeter of a parallelogram.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_parallelogram(6.0, 4.0);
    /// assert_eq!(result, 20.0);
    /// ```
    pub fn perimeter_parallelogram(side_a: f64, side_b: f64) -> f64 {
        let perimeter = 2.0 * (side_a + side_b);
        perimeter
    }

    /// Calculates the perimeter of a rhombus.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_rhombus(5.0);
    /// assert_eq!(result, 20.0);
    /// ```
    pub fn perimeter_rhombus(side: f64) -> f64 {
        let perimeter = 4.0 * side;
        perimeter
    }

    /// Calculates the perimeter of a trapezoid.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_trapezoid(3.0, 4.0, 5.0, 6.0);
    /// assert_eq!(result, 18.0);
    /// ```
    pub fn perimeter_trapezoid(side_a: f64, side_b: f64, side_c: f64, side_d: f64) -> f64 {
        let perimeter = side_a + side_b + side_c + side_d;
        perimeter
    }

    /// Calculates the perimeter of a kite.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_kite(3.0, 5.0);
    /// assert_eq!(result, 16.0);
    /// ```
    pub fn perimeter_kite(side_a: f64, side_b: f64) -> f64 {
        let perimeter = 2.0 * (side_a + side_b);
        perimeter
    }

    /// Calculates the perimeter of a regular pentagon.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_pentagon(4.0);
    /// assert_eq!(result, 20.0);
    /// ```
    pub fn perimeter_pentagon(side: f64) -> f64 {
        let perimeter = 5.0 * side;
        perimeter
    }

    /// Calculates the perimeter of a regular hexagon.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_hexagon(3.0);
    /// assert_eq!(result, 18.0);
    /// ```
    pub fn perimeter_hexagon(side: f64) -> f64 {
        let perimeter = 6.0 * side;
        perimeter
    }

    /// Calculates the circumference of a circle.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::perimeter::perimeter_circle(1.0);
    /// assert!((result - std::f64::consts::TAU).abs() < 1e-9);
    /// ```
    pub fn perimeter_circle(radius: f64) -> f64 {
        let perimeter = 2.0 * std::f64::consts::PI * radius;
        perimeter
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_perimeter_triangle() {
            assert!((perimeter_triangle(3.0, 4.0, 5.0) - 12.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_equilateral_triangle() {
            assert!((perimeter_equilateral_triangle(5.0) - 15.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_rectangle() {
            assert!((perimeter_rectangle(4.0, 3.0) - 14.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_square() {
            assert!((perimeter_square(5.0) - 20.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_parallelogram() {
            assert!((perimeter_parallelogram(6.0, 4.0) - 20.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_rhombus() {
            assert!((perimeter_rhombus(5.0) - 20.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_trapezoid() {
            assert!((perimeter_trapezoid(3.0, 4.0, 5.0, 6.0) - 18.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_kite() {
            assert!((perimeter_kite(3.0, 5.0) - 16.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_pentagon() {
            assert!((perimeter_pentagon(4.0) - 20.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_hexagon() {
            assert!((perimeter_hexagon(3.0) - 18.0).abs() < 1e-9);
        }

        #[test]
        fn test_perimeter_circle() {
            assert!((perimeter_circle(1.0) - std::f64::consts::TAU).abs() < 1e-9);
        }
    }
}

pub mod areas {

    /// Calculates the area of a triangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::areas::area_triangle(6.0, 4.0);
    /// assert_eq!(result, 12.0);
    /// ```
    pub fn area_triangle(base: f64, height: f64) -> f64 {
        let area = 0.5 * base * height;
        area
    }

    /// Calculates the area of a rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::areas::area_rectangle(5.0, 3.0);
    /// assert_eq!(result, 15.0);
    /// ```
    pub fn area_rectangle(width: f64, height: f64) -> f64 {
        let area = width * height;
        area
    }

    /// Calculates the area of a trapezoid.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::areas::area_trapezoid(4.0, 3.0, 5.0);
    /// assert_eq!(result, 16.0);
    /// ```
    pub fn area_trapezoid(height: f64, length_a: f64, length_b: f64) -> f64 {
        let area = (0.5 * (length_a + length_b)) * height;
        area
    }

    /// Calculates the area of a square.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::areas::area_square(4.0);
    /// assert_eq!(result, 16.0);
    /// ```
    pub fn area_square(side: f64) -> f64 {
        let area = side * side;
        area
    }

    /// Calculates the area of a parallelogram.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::areas::area_parallelogram(5.0, 3.0);
    /// assert_eq!(result, 15.0);
    /// ```
    pub fn area_parallelogram(base: f64, height: f64) -> f64 {
        let area = base * height;
        area
    }

    /// Calculates the area of a circle.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::areas::area_circle(1.0);
    /// assert!((result - std::f64::consts::PI).abs() < 1e-9);
    /// ```
    pub fn area_circle(radius: f64) -> f64 {
        let area = std::f64::consts::PI * (radius * radius);
        area
    }

    /// Calculates the area of a kite.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::areas::area_kite(6.0, 4.0);
    /// assert_eq!(result, 12.0);
    /// ```
    pub fn area_kite(d1: f64, d2: f64) -> f64 {
        let area = 0.5 * d1 * d2;
        area
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_area_triangle() {
            assert!((area_triangle(6.0, 4.0) - 12.0).abs() < 1e-9);
        }

        #[test]
        fn test_area_rectangle() {
            assert!((area_rectangle(5.0, 3.0) - 15.0).abs() < 1e-9);
        }

        #[test]
        fn test_area_trapezoid() {
            assert!((area_trapezoid(4.0, 3.0, 5.0) - 16.0).abs() < 1e-9);
        }

        #[test]
        fn test_area_square() {
            assert!((area_square(4.0) - 16.0).abs() < 1e-9);
        }

        #[test]
        fn test_area_parallelogram() {
            assert!((area_parallelogram(5.0, 3.0) - 15.0).abs() < 1e-9);
        }

        #[test]
        fn test_area_circle() {
            assert!((area_circle(1.0) - std::f64::consts::PI).abs() < 1e-9);
        }

        #[test]
        fn test_area_kite() {
            assert!((area_kite(6.0, 4.0) - 12.0).abs() < 1e-9);
        }
    }
}

pub mod volumes {

    /// Calculates the volume of a cube.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volumes::volume_cube(3.0);
    /// assert_eq!(result, 27.0);
    /// ```
    pub fn volume_cube(side: f64) -> f64 {
        let volume = side * side * side;
        volume
    }

    /// Calculates the volume of a rectangular prism.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volumes::volume_rectangular_prism(2.0, 3.0, 4.0);
    /// assert_eq!(result, 24.0);
    /// ```
    pub fn volume_rectangular_prism(length: f64, width: f64, height: f64) -> f64 {
        let volume = length * width * height;
        volume
    }

    // pub fn volume_prism() {

    // }

    // pub fn volume_pyramid() {

    // }

    /// Calculates the volume of a cone.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volumes::volume_cone(3.0, 4.0);
    /// assert!((result - 37.69911184307752).abs() < 1e-6);
    /// ```
    pub fn volume_cone(radius: f64, height: f64) -> f64 {
        let volume = (1.0 / 3.0) * std::f64::consts::PI * (radius * radius) * height;
        volume
    }

    /// Calculates the volume of a cylinder.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volumes::volume_cylinder(3.0, 4.0);
    /// assert!((result - 113.09733552923255).abs() < 1e-6);
    /// ```
    pub fn volume_cylinder(radius: f64, height: f64) -> f64 {
        let volume = std::f64::consts::PI * (radius * radius) * height;
        volume
    }

    /// Calculates the volume of a sphere.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volumes::volume_sphere(3.0);
    /// assert!((result - 113.09733552923255).abs() < 1e-6);
    /// ```
    pub fn volume_sphere(radius: f64) -> f64 {
        let volume = (4.0 / 3.0) * std::f64::consts::PI * (radius * radius * radius);
        volume
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_volume_cube() {
            assert!((volume_cube(3.0) - 27.0).abs() < 1e-9);
        }

        #[test]
        fn test_volume_rectangular_prism() {
            assert!((volume_rectangular_prism(2.0, 3.0, 4.0) - 24.0).abs() < 1e-9);
        }

        #[test]
        fn test_volume_cone() {
            let expected = (1.0 / 3.0) * std::f64::consts::PI * 9.0 * 4.0;
            assert!((volume_cone(3.0, 4.0) - expected).abs() < 1e-9);
        }

        #[test]
        fn test_volume_cylinder() {
            let expected = std::f64::consts::PI * 9.0 * 4.0;
            assert!((volume_cylinder(3.0, 4.0) - expected).abs() < 1e-9);
        }

        #[test]
        fn test_volume_sphere() {
            let expected = (4.0 / 3.0) * std::f64::consts::PI * 27.0;
            assert!((volume_sphere(3.0) - expected).abs() < 1e-9);
        }
    }
}
