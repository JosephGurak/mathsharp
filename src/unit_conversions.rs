

pub mod length {

    /// Converts inches to feet.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::inches_feet(12.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn inches_feet(inches: f64) -> f64 {
        let feet = inches / 12.0;
        feet
    }

    /// Converts inches to miles.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::inches_mile(63360.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn inches_mile(inches: f64) -> f64 {
        let mile = inches / 63360.0;
        mile
    }

    /// Converts feet to inches.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::feet_inches(1.0);
    /// assert_eq!(result, 12.0);
    /// ```
    pub fn feet_inches(feet: f64) -> f64 {
        let inches = feet * 12.0;
        inches
    }

    /// Converts feet to yards.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::feet_yards(3.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn feet_yards(feet: f64) -> f64 {
        let yards = feet / 3.0;
        yards
    }

    /// Converts yards to feet.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::yards_feet(3.0);
    /// assert_eq!(result, 9.0);
    /// ```
    pub fn yards_feet(yards: f64) -> f64 {
        let feet = yards * 3.0;
        feet
    }

    /// Converts yards to inches.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::yards_inches(1.0);
    /// assert_eq!(result, 36.0);
    /// ```
    pub fn yards_inches(yards: f64) -> f64 {
        let inches = yards * 36.0;
        inches
    }

    /// Converts yards to miles.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::yards_mile(1760.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn yards_mile(yards: f64) -> f64 {
        let mile = yards / 1760.0;
        mile
    }

    /// Converts miles to yards.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::miles_yards(1.0);
    /// assert_eq!(result, 1760.0);
    /// ```
    pub fn miles_yards(miles: f64) -> f64 {
        let yards = miles * 1760.0;
        yards
    }

    /// Converts feet to miles.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::feet_mile(5280.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn feet_mile(feet: f64) -> f64 {
        let mile = feet / 5280.0;
        mile
    }

    /// Converts miles to feet.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::mile_feet(1.0);
    /// assert_eq!(result, 5280.0);
    /// ```
    pub fn mile_feet(mile: f64) -> f64 {
        let feet = mile * 5280.0;
        feet
    }

    /// Converts miles to inches.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::mile_inches(1.0);
    /// assert_eq!(result, 63360.0);
    /// ```
    pub fn mile_inches(mile: f64) -> f64 {
        let inches = mile * 63360.0;
        inches
    }

    // imperial to metric

    /// Converts inches to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::inches_millimeter(1.0);
    /// assert_eq!(result, 25.4);
    /// ```
    pub fn inches_millimeter(inches: f64) -> f64 {
        let millimeter = inches * 25.4;
        millimeter
    }

    /// Converts inches to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::inches_centimeter(1.0);
    /// assert_eq!(result, 2.54);
    /// ```
    pub fn inches_centimeter(inches: f64) -> f64 {
        let centimeter = inches * 2.54;
        centimeter
    }

    /// Converts inches to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::inches_meter(1.0);
    /// assert_eq!(result, 0.0254);
    /// ```
    pub fn inches_meter(inches: f64) -> f64 {
        let meter = inches * 0.0254;
        meter
    }

    /// Converts inches to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::inches_kilometer(1.0);
    /// assert_eq!(result, 0.0000254);
    /// ```
    pub fn inches_kilometer(inches: f64) -> f64 {
        let kilometer = inches * 0.0000254;
        kilometer
    }

    /// Converts feet to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::feet_millimeter(1.0);
    /// assert_eq!(result, 304.8);
    /// ```
    pub fn feet_millimeter(feet: f64) -> f64 {
        let millimeter = feet * 304.8;
        millimeter
    }

    /// Converts feet to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::feet_centimeter(1.0);
    /// assert_eq!(result, 30.48);
    /// ```
    pub fn feet_centimeter(feet: f64) -> f64 {
        let centimeter = feet * 30.48;
        centimeter
    }

    /// Converts feet to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::feet_meter(1.0);
    /// assert_eq!(result, 0.3048);
    /// ```
    pub fn feet_meter(feet: f64) -> f64 {
        let meter = feet * 0.3048;
        meter
    }

    /// Converts feet to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::feet_kilometer(1.0);
    /// assert_eq!(result, 0.0003048);
    /// ```
    pub fn feet_kilometer(feet: f64) -> f64 {
        let kilometer = feet * 0.0003048;
        kilometer
    }

    /// Converts yards to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::yards_millimeter(1.0);
    /// assert_eq!(result, 914.4);
    /// ```
    pub fn yards_millimeter(yards: f64) -> f64 {
        let millimeter = yards * 914.4;
        millimeter
    }

    /// Converts yards to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::yards_centimeter(1.0);
    /// assert_eq!(result, 91.44);
    /// ```
    pub fn yards_centimeter(yards: f64) -> f64 {
        let centimeter = yards * 91.44;
        centimeter
    }

    /// Converts yards to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::yards_meter(1.0);
    /// assert_eq!(result, 0.9144);
    /// ```
    pub fn yards_meter(yards: f64) -> f64 {
        let meter = yards * 0.9144;
        meter
    }

    /// Converts yards to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::yards_kilometer(1.0);
    /// assert_eq!(result, 0.0009144);
    /// ```
    pub fn yards_kilometer(yards: f64) -> f64 {
        let kilometer = yards * 0.0009144;
        kilometer
    }

    /// Converts miles to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::miles_millimeter(1.0);
    /// assert_eq!(result, 1_609_344.0);
    /// ```
    pub fn miles_millimeter(miles: f64) -> f64 {
        let millimeter = miles * 1_609_344.0;
        millimeter
    }

    /// Converts miles to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::miles_centimeter(1.0);
    /// assert_eq!(result, 160_934.4);
    /// ```
    pub fn miles_centimeter(miles: f64) -> f64 {
        let centimeter = miles * 160_934.4;
        centimeter
    }

    /// Converts miles to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::miles_meter(1.0);
    /// assert_eq!(result, 1609.344);
    /// ```
    pub fn miles_meter(miles: f64) -> f64 {
        let meter = miles * 1609.344;
        meter
    }

    /// Converts miles to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::miles_kilometer(1.0);
    /// assert_eq!(result, 1.609344);
    /// ```
    pub fn miles_kilometer(miles: f64) -> f64 {
        let kilometer = miles * 1.609344;
        kilometer
    }

    // metric to imperial

    /// Converts millimeters to inches.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::millimeter_inches(25.4);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn millimeter_inches(millimeter: f64) -> f64 {
        let inches = millimeter / 25.4;
        inches
    }

    /// Converts millimeters to feet.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::millimeter_feet(304.8);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn millimeter_feet(millimeter: f64) -> f64 {
        let feet = millimeter / 304.8;
        feet
    }

    /// Converts millimeters to yards.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::millimeter_yards(914.4);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn millimeter_yards(millimeter: f64) -> f64 {
        let yards = millimeter / 914.4;
        yards
    }

    /// Converts millimeters to miles.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::millimeter_miles(1_609_344.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn millimeter_miles(millimeter: f64) -> f64 {
        let miles = millimeter / 1_609_344.0;
        miles
    }

    /// Converts centimeters to inches.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::centimeter_inches(2.54);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn centimeter_inches(centimeter: f64) -> f64 {
        let inches = centimeter / 2.54;
        inches
    }

    /// Converts centimeters to feet.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::centimeter_feet(30.48);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn centimeter_feet(centimeter: f64) -> f64 {
        let feet = centimeter / 30.48;
        feet
    }

    /// Converts centimeters to yards.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::centimeter_yards(91.44);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn centimeter_yards(centimeter: f64) -> f64 {
        let yards = centimeter / 91.44;
        yards
    }

    /// Converts centimeters to miles.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::centimeter_miles(160_934.4);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn centimeter_miles(centimeter: f64) -> f64 {
        let miles = centimeter / 160_934.4;
        miles
    }

    /// Converts meters to inches.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::meter_inches(0.0254);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn meter_inches(meter: f64) -> f64 {
        let inches = meter / 0.0254;
        inches
    }

    /// Converts meters to feet.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::meter_feet(0.3048);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn meter_feet(meter: f64) -> f64 {
        let feet = meter / 0.3048;
        feet
    }

    /// Converts meters to yards.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::meter_yards(0.9144);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn meter_yards(meter: f64) -> f64 {
        let yards = meter / 0.9144;
        yards
    }

    /// Converts meters to miles.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::meter_miles(1609.344);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn meter_miles(meter: f64) -> f64 {
        let miles = meter / 1609.344;
        miles
    }

    /// Converts kilometers to inches.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::kilometer_inches(0.0000254);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn kilometer_inches(kilometer: f64) -> f64 {
        let inches = kilometer / 0.0000254;
        inches
    }

    /// Converts kilometers to feet.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::kilometer_feet(0.0003048);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn kilometer_feet(kilometer: f64) -> f64 {
        let feet = kilometer / 0.0003048;
        feet
    }

    /// Converts kilometers to yards.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::kilometer_yards(0.0009144);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn kilometer_yards(kilometer: f64) -> f64 {
        let yards = kilometer / 0.0009144;
        yards
    }

    /// Converts kilometers to miles.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::kilometer_miles(1.609344);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn kilometer_miles(kilometer: f64) -> f64 {
        let miles = kilometer / 1.609344;
        miles
    }

    // metric internal

    /// Converts millimeters to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::millimeter_centimeter(10.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn millimeter_centimeter(millimeter: f64) -> f64 {
        let centimeter = millimeter / 10.0;
        centimeter
    }

    /// Converts millimeters to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::millimeter_meter(1000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn millimeter_meter(millimeter: f64) -> f64 {
        let meter = millimeter / 1000.0;
        meter
    }

    /// Converts millimeters to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::millimeter_kilometer(1_000_000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn millimeter_kilometer(millimeter: f64) -> f64 {
        let kilometer = millimeter / 1_000_000.0;
        kilometer
    }

    /// Converts centimeters to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::centimeter_millimeter(1.0);
    /// assert_eq!(result, 10.0);
    /// ```
    pub fn centimeter_millimeter(centimeter: f64) -> f64 {
        let millimeter = centimeter * 10.0;
        millimeter
    }

    /// Converts centimeters to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::centimeter_meter(100.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn centimeter_meter(centimeter: f64) -> f64 {
        let meter = centimeter / 100.0;
        meter
    }

    /// Converts centimeters to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::centimeter_kilometer(100_000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn centimeter_kilometer(centimeter: f64) -> f64 {
        let kilometer = centimeter / 100_000.0;
        kilometer
    }

    /// Converts meters to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::meters_centimeter(1.0);
    /// assert_eq!(result, 100.0);
    /// ```
    pub fn meters_centimeter(meter: f64) -> f64 {
        let centimeter = meter * 100.0;
        centimeter
    }

    /// Converts meters to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::meters_millimeter(1.0);
    /// assert_eq!(result, 1000.0);
    /// ```
    pub fn meters_millimeter(meter: f64) -> f64 {
        let millimeter = meter * 1000.0;
        millimeter
    }

    /// Converts meters to kilometers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::meters_kilometer(1000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn meters_kilometer(meter: f64) -> f64 {
        let kilometer = meter / 1000.0;
        kilometer
    }

    /// Converts kilometers to meters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::kilometer_meters(1.0);
    /// assert_eq!(result, 1000.0);
    /// ```
    pub fn kilometer_meters(kilometer: f64) -> f64 {
        let meters = kilometer * 1000.0;
        meters
    }

    /// Converts kilometers to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::kilometer_centimeters(1.0);
    /// assert_eq!(result, 100_000.0);
    /// ```
    pub fn kilometer_centimeters(kilometer: f64) -> f64 {
        let centimeters = kilometer * 100_000.0;
        centimeters
    }

    /// Converts kilometers to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::length::kilometer_millimeters(1.0);
    /// assert_eq!(result, 1_000_000.0);
    /// ```
    pub fn kilometer_millimeters(kilometer: f64) -> f64 {
        let millimeters = kilometer * 1_000_000.0;
        millimeters
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_inches_feet() {
            assert!((inches_feet(12.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_inches_mile() {
            assert!((inches_mile(63360.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_feet_inches() {
            assert!((feet_inches(1.0) - 12.0).abs() < 1e-9);
        }

        #[test]
        fn test_feet_yards() {
            assert!((feet_yards(3.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_yards_feet() {
            assert!((yards_feet(1.0) - 3.0).abs() < 1e-9);
        }

        #[test]
        fn test_yards_inches() {
            assert!((yards_inches(1.0) - 36.0).abs() < 1e-9);
        }

        #[test]
        fn test_yards_mile() {
            assert!((yards_mile(1760.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_miles_yards() {
            assert!((miles_yards(1.0) - 1760.0).abs() < 1e-9);
        }

        #[test]
        fn test_feet_mile() {
            assert!((feet_mile(5280.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_mile_feet() {
            assert!((mile_feet(1.0) - 5280.0).abs() < 1e-9);
        }

        #[test]
        fn test_mile_inches() {
            assert!((mile_inches(1.0) - 63360.0).abs() < 1e-9);
        }

        #[test]
        fn test_inches_millimeter() {
            assert!((inches_millimeter(1.0) - 25.4).abs() < 1e-9);
        }

        #[test]
        fn test_inches_centimeter() {
            assert!((inches_centimeter(1.0) - 2.54).abs() < 1e-9);
        }

        #[test]
        fn test_inches_meter() {
            assert!((inches_meter(1.0) - 0.0254).abs() < 1e-9);
        }

        #[test]
        fn test_inches_kilometer() {
            assert!((inches_kilometer(1.0) - 0.0000254).abs() < 1e-12);
        }

        #[test]
        fn test_feet_millimeter() {
            assert!((feet_millimeter(1.0) - 304.8).abs() < 1e-9);
        }

        #[test]
        fn test_feet_centimeter() {
            assert!((feet_centimeter(1.0) - 30.48).abs() < 1e-9);
        }

        #[test]
        fn test_feet_meter() {
            assert!((feet_meter(1.0) - 0.3048).abs() < 1e-9);
        }

        #[test]
        fn test_feet_kilometer() {
            assert!((feet_kilometer(1.0) - 0.0003048).abs() < 1e-9);
        }

        #[test]
        fn test_yards_millimeter() {
            assert!((yards_millimeter(1.0) - 914.4).abs() < 1e-9);
        }

        #[test]
        fn test_yards_centimeter() {
            assert!((yards_centimeter(1.0) - 91.44).abs() < 1e-9);
        }

        #[test]
        fn test_yards_meter() {
            assert!((yards_meter(1.0) - 0.9144).abs() < 1e-9);
        }

        #[test]
        fn test_yards_kilometer() {
            assert!((yards_kilometer(1.0) - 0.0009144).abs() < 1e-9);
        }

        #[test]
        fn test_miles_millimeter() {
            assert!((miles_millimeter(1.0) - 1_609_344.0).abs() < 1e-6);
        }

        #[test]
        fn test_miles_centimeter() {
            assert!((miles_centimeter(1.0) - 160_934.4).abs() < 1e-6);
        }

        #[test]
        fn test_miles_meter() {
            assert!((miles_meter(1.0) - 1609.344).abs() < 1e-6);
        }

        #[test]
        fn test_miles_kilometer() {
            assert!((miles_kilometer(1.0) - 1.609344).abs() < 1e-9);
        }

        #[test]
        fn test_millimeter_inches() {
            assert!((millimeter_inches(25.4) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_millimeter_feet() {
            assert!((millimeter_feet(304.8) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_millimeter_yards() {
            assert!((millimeter_yards(914.4) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_millimeter_miles() {
            assert!((millimeter_miles(1_609_344.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_centimeter_inches() {
            assert!((centimeter_inches(2.54) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_centimeter_feet() {
            assert!((centimeter_feet(30.48) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_centimeter_yards() {
            assert!((centimeter_yards(91.44) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_centimeter_miles() {
            assert!((centimeter_miles(160_934.4) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_meter_inches() {
            assert!((meter_inches(0.0254) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_meter_feet() {
            assert!((meter_feet(0.3048) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_meter_yards() {
            assert!((meter_yards(0.9144) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_meter_miles() {
            assert!((meter_miles(1609.344) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilometer_inches() {
            assert!((kilometer_inches(0.0000254) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilometer_feet() {
            assert!((kilometer_feet(0.0003048) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilometer_yards() {
            assert!((kilometer_yards(0.0009144) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilometer_miles() {
            assert!((kilometer_miles(1.609344) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_millimeter_centimeter() {
            assert!((millimeter_centimeter(10.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_millimeter_meter() {
            assert!((millimeter_meter(1000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_millimeter_kilometer() {
            assert!((millimeter_kilometer(1_000_000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_centimeter_millimeter() {
            assert!((centimeter_millimeter(1.0) - 10.0).abs() < 1e-9);
        }

        #[test]
        fn test_centimeter_meter() {
            assert!((centimeter_meter(100.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_centimeter_kilometer() {
            assert!((centimeter_kilometer(100_000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_meters_centimeter() {
            assert!((meters_centimeter(1.0) - 100.0).abs() < 1e-9);
        }

        #[test]
        fn test_meters_millimeter() {
            assert!((meters_millimeter(1.0) - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn test_meters_kilometer() {
            assert!((meters_kilometer(1000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilometer_meters() {
            assert!((kilometer_meters(1.0) - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilometer_centimeters() {
            assert!((kilometer_centimeters(1.0) - 100_000.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilometer_millimeters() {
            assert!((kilometer_millimeters(1.0) - 1_000_000.0).abs() < 1e-9);
        }
    }
}

pub mod mass {

    /// Converts kilograms to grams.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::kilogram_gram(1.0);
    /// assert_eq!(result, 1000.0);
    /// ```
    pub fn kilogram_gram(kilogram: f64) -> f64 {
        let gram = kilogram * 1000.0;
        gram
    }

    /// Converts kilograms to milligrams.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::kilogram_milligram(1.0);
    /// assert_eq!(result, 1_000_000.0);
    /// ```
    pub fn kilogram_milligram(kilogram: f64) -> f64 {
        let milligram = kilogram * 1_000_000.0;
        milligram
    }

    /// Converts kilograms to pounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::kilogram_pounds(1.0);
    /// assert_eq!(result, 2.20462);
    /// ```
    pub fn kilogram_pounds(kilogram: f64) -> f64 {
        let pounds = kilogram * 2.20462;
        pounds
    }

    /// Converts grams to kilograms.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::gram_kilogram(1000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn gram_kilogram(gram: f64) -> f64 {
        let kilogram = gram / 1000.0;
        kilogram
    }

    /// Converts grams to milligrams.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::gram_milligram(1.0);
    /// assert_eq!(result, 1000.0);
    /// ```
    pub fn gram_milligram(gram: f64) -> f64 {
        let milligram = gram * 1000.0;
        milligram
    }

    /// Converts grams to pounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::gram_pounds(1.0);
    /// assert_eq!(result, 0.0022);
    /// ```
    pub fn gram_pounds(gram: f64) -> f64 {
        let pounds = gram * 0.0022;
        pounds
    }

    /// Converts milligrams to grams.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::milligram_gram(1000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn milligram_gram(milligram: f64) -> f64 {
        let gram = milligram * 0.001;
        gram
    }

    /// Converts milligrams to kilograms.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::milligram_kilogram(1_000_000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn milligram_kilogram(milligram: f64) -> f64 {
        let kilogram = milligram / 1_000_000.0;
        kilogram
    }

    /// Converts pounds to ounces.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::pounds_ounces(1.0);
    /// assert_eq!(result, 16.0);
    /// ```
    pub fn pounds_ounces(pounds: f64) -> f64 {
        let ounces = pounds * 16.0;
        ounces
    }

    /// Converts pounds to kilograms.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::pounds_kilograms(2.20462);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn pounds_kilograms(pounds: f64) -> f64 {
        let kilograms = pounds / 2.20462;
        kilograms
    }

    /// Converts pounds to grams.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::pounds_grams(1.0);
    /// assert_eq!(result, 453.59237);
    /// ```
    pub fn pounds_grams(pounds: f64) -> f64 {
        let grams = pounds * 453.59237;
        grams
    }

    /// Converts ounces to pounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::mass::ounces_pounds(16.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn ounces_pounds(ounces: f64) -> f64 {
        let pounds = ounces / 16.0;
        pounds
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_kilogram_gram() {
            assert!((kilogram_gram(1.0) - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilogram_milligram() {
            assert!((kilogram_milligram(1.0) - 1_000_000.0).abs() < 1e-6);
        }

        #[test]
        fn test_kilogram_pounds() {
            assert!((kilogram_pounds(1.0) - 2.20462).abs() < 1e-9);
        }

        #[test]
        fn test_gram_kilogram() {
            assert!((gram_kilogram(1000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_gram_milligram() {
            assert!((gram_milligram(1.0) - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn test_gram_pounds() {
            assert!((gram_pounds(1.0) - 0.0022).abs() < 1e-9);
        }

        #[test]
        fn test_milligram_gram() {
            assert!((milligram_gram(1000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_milligram_kilogram() {
            assert!((milligram_kilogram(1_000_000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_pounds_ounces() {
            assert!((pounds_ounces(1.0) - 16.0).abs() < 1e-9);
        }

        #[test]
        fn test_pounds_kilograms() {
            assert!((pounds_kilograms(2.20462) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_pounds_grams() {
            assert!((pounds_grams(1.0) - 453.59237).abs() < 1e-9);
        }

        #[test]
        fn test_ounces_pounds() {
            assert!((ounces_pounds(16.0) - 1.0).abs() < 1e-9);
        }
    }
}

pub mod volume {

    /// Converts milliliters to liters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volume::milliliter_liter(1000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn milliliter_liter(milliliter: f64) -> f64 {
        let liter = milliliter / 1000.0;
        liter
    }

    /// Converts liters to milliliters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volume::liter_milliliter(1.0);
    /// assert_eq!(result, 1000.0);
    /// ```
    pub fn liter_milliliter(liter: f64) -> f64 {
        let milliliter = liter * 1000.0;
        milliliter
    }

    /// Converts liters to US liquid gallons.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volume::liters_to_us_liquid_gallon(3.785412);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn liters_to_us_liquid_gallon(liters: f64) -> f64 {
        let gallons = liters / 3.785412;
        gallons
    }

    /// Converts liters to US dry gallons.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volume::liters_to_us_dry_gallon(4.404884);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn liters_to_us_dry_gallon(liters: f64) -> f64 {
        let gallons = liters / 4.404884;
        gallons
    }

    /// Converts US liquid gallons to liters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volume::us_liquid_gallon_to_liters(1.0);
    /// assert_eq!(result, 3.785412);
    /// ```
    pub fn us_liquid_gallon_to_liters(gallons: f64) -> f64 {
        let liters = gallons * 3.785412;
        liters
    }

    /// Converts US dry gallons to liters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volume::us_dry_gallon_to_liters(1.0);
    /// assert_eq!(result, 4.404884);
    /// ```
    pub fn us_dry_gallon_to_liters(gallons: f64) -> f64 {
        let liters = gallons * 4.404884;
        liters
    }

    /// Converts gallons to fluid ounces.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::volume::gallon_to_fluid_ounces(1.0);
    /// assert_eq!(result, 128.0);
    /// ```
    pub fn gallon_to_fluid_ounces(gallons: f64) -> f64 {
        let fluid_ounces = gallons * 128.0;
        fluid_ounces
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_milliliter_liter() {
            assert!((milliliter_liter(1000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_liter_milliliter() {
            assert!((liter_milliliter(1.0) - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn test_liters_to_us_liquid_gallon() {
            assert!((liters_to_us_liquid_gallon(3.785412) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_liters_to_us_dry_gallon() {
            assert!((liters_to_us_dry_gallon(4.404884) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_us_liquid_gallon_to_liters() {
            assert!((us_liquid_gallon_to_liters(1.0) - 3.785412).abs() < 1e-9);
        }

        #[test]
        fn test_us_dry_gallon_to_liters() {
            assert!((us_dry_gallon_to_liters(1.0) - 4.404884).abs() < 1e-9);
        }

        #[test]
        fn test_gallon_to_fluid_ounces() {
            assert!((gallon_to_fluid_ounces(1.0) - 128.0).abs() < 1e-9);
        }
    }
}


pub mod time {

    /// Converts seconds to minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::seconds_minutes(60.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn seconds_minutes(seconds: f64) -> f64 {
        let minutes = seconds / 60.0;
        minutes
    }

    /// Converts seconds to hours.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::seconds_hours(3600.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn seconds_hours(seconds: f64) -> f64 {
        let hours = seconds / 3600.0;
        hours
    }

    /// Converts seconds to days.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::seconds_days(86400.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn seconds_days(seconds: f64) -> f64 {
        let days = seconds / 86400.0;
        days
    }

    /// Converts minutes to seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::minutes_seconds(1.0);
    /// assert_eq!(result, 60.0);
    /// ```
    pub fn minutes_seconds(minutes: f64) -> f64 {
        let seconds = minutes * 60.0;
        seconds
    }

    /// Converts minutes to hours.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::minutes_hours(60.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn minutes_hours(minutes: f64) -> f64 {
        let hours = minutes / 60.0;
        hours
    }

    /// Converts minutes to days.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::minutes_days(1440.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn minutes_days(minutes: f64) -> f64 {
        let days = minutes / 1440.0;
        days
    }

    /// Converts hours to seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::hours_seconds(1.0);
    /// assert_eq!(result, 3600.0);
    /// ```
    pub fn hours_seconds(hours: f64) -> f64 {
        let seconds = hours * 3600.0;
        seconds
    }

    /// Converts hours to minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::hours_minutes(1.0);
    /// assert_eq!(result, 60.0);
    /// ```
    pub fn hours_minutes(hours: f64) -> f64 {
        let minutes = hours * 60.0;
        minutes
    }

    /// Converts hours to days.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::hours_days(24.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn hours_days(hours: f64) -> f64 {
        let days = hours / 24.0;
        days
    }

    /// Converts days to seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::days_seconds(1.0);
    /// assert_eq!(result, 86400.0);
    /// ```
    pub fn days_seconds(days: f64) -> f64 {
        let seconds = days * 86400.0;
        seconds
    }

    /// Converts days to minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::days_minutes(1.0);
    /// assert_eq!(result, 1440.0);
    /// ```
    pub fn days_minutes(days: f64) -> f64 {
        let minutes = days * 1440.0;
        minutes
    }

    /// Converts days to hours.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::time::days_hours(1.0);
    /// assert_eq!(result, 24.0);
    /// ```
    pub fn days_hours(days: f64) -> f64 {
        let hours = days * 24.0;
        hours
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_seconds_minutes() {
            assert!((seconds_minutes(60.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_seconds_hours() {
            assert!((seconds_hours(3600.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_seconds_days() {
            assert!((seconds_days(86400.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_minutes_seconds() {
            assert!((minutes_seconds(1.0) - 60.0).abs() < 1e-9);
        }

        #[test]
        fn test_minutes_hours() {
            assert!((minutes_hours(60.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_minutes_days() {
            assert!((minutes_days(1440.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_hours_seconds() {
            assert!((hours_seconds(1.0) - 3600.0).abs() < 1e-9);
        }

        #[test]
        fn test_hours_minutes() {
            assert!((hours_minutes(1.0) - 60.0).abs() < 1e-9);
        }

        #[test]
        fn test_hours_days() {
            assert!((hours_days(24.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_days_seconds() {
            assert!((days_seconds(1.0) - 86400.0).abs() < 1e-9);
        }

        #[test]
        fn test_days_minutes() {
            assert!((days_minutes(1.0) - 1440.0).abs() < 1e-9);
        }

        #[test]
        fn test_days_hours() {
            assert!((days_hours(1.0) - 24.0).abs() < 1e-9);
        }
    }
}


pub mod power {

    /// Converts horsepower to watts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::horsepower_watts(1.0);
    /// assert_eq!(result, 746.0);
    /// ```
    pub fn horsepower_watts(horsepower: f64) -> f64 {
        let watts = horsepower * 746.0;
        watts
    }

    /// Converts watts to horsepower.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::watts_horsepower(746.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn watts_horsepower(watts: f64) -> f64 {
        let horsepower = watts / 746.0;
        horsepower
    }

    /// Converts watts to kilowatts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::watts_kilowatt(1000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn watts_kilowatt(watts: f64) -> f64 {
        let kilowatt = watts / 1000.0;
        kilowatt
    }

    /// Converts watts to megawatts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::watts_megawatt(1_000_000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn watts_megawatt(watts: f64) -> f64 {
        let megawatt = watts / 1_000_000.0;
        megawatt
    }

    /// Converts watts to gigawatts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::watts_gigawatt(1_000_000_000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn watts_gigawatt(watts: f64) -> f64 {
        let gigawatt = watts / 1_000_000_000.0;
        gigawatt
    }

    /// Converts kilowatts to watts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::kilowatt_watts(1.0);
    /// assert_eq!(result, 1000.0);
    /// ```
    pub fn kilowatt_watts(kilowatt: f64) -> f64 {
        let watts = kilowatt * 1000.0;
        watts
    }

    /// Converts kilowatts to megawatts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::kilowatt_megawatts(1000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn kilowatt_megawatts(kilowatt: f64) -> f64 {
        let megawatts = kilowatt / 1000.0;
        megawatts
    }

    /// Converts kilowatts to gigawatts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::kilowatt_gigawatts(1_000_000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn kilowatt_gigawatts(kilowatt: f64) -> f64 {
        let gigawatts = kilowatt / 1_000_000.0;
        gigawatts
    }

    /// Converts megawatts to watts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::megawatts_watts(1.0);
    /// assert_eq!(result, 1_000_000.0);
    /// ```
    pub fn megawatts_watts(megawatts: f64) -> f64 {
        let watts = megawatts * 1_000_000.0;
        watts
    }

    /// Converts megawatts to kilowatts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::megawatts_kilowatts(1.0);
    /// assert_eq!(result, 1000.0);
    /// ```
    pub fn megawatts_kilowatts(megawatts: f64) -> f64 {
        let kilowatts = megawatts * 1000.0;
        kilowatts
    }

    /// Converts megawatts to gigawatts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::megawatts_gigawatts(1000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn megawatts_gigawatts(megawatts: f64) -> f64 {
        let gigawatts = megawatts / 1000.0;
        gigawatts
    }

    /// Converts gigawatts to watts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::gigawatts_watts(1.0);
    /// assert_eq!(result, 1_000_000_000.0);
    /// ```
    pub fn gigawatts_watts(gigawatts: f64) -> f64 {
        let watts = gigawatts * 1_000_000_000.0;
        watts
    }

    /// Converts gigawatts to kilowatts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::gigawatts_kilowatts(1.0);
    /// assert_eq!(result, 1_000_000.0);
    /// ```
    pub fn gigawatts_kilowatts(gigawatts: f64) -> f64 {
        let kilowatts = gigawatts * 1_000_000.0;
        kilowatts
    }

    /// Converts gigawatts to megawatts.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::power::gigawatts_megawatts(1.0);
    /// assert_eq!(result, 1000.0);
    /// ```
    pub fn gigawatts_megawatts(gigawatts: f64) -> f64 {
        let megawatts = gigawatts * 1000.0;
        megawatts
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_horsepower_watts() {
            assert!((horsepower_watts(1.0) - 746.0).abs() < 1e-9);
        }

        #[test]
        fn test_watts_horsepower() {
            assert!((watts_horsepower(746.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_watts_kilowatt() {
            assert!((watts_kilowatt(1000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_watts_megawatt() {
            assert!((watts_megawatt(1_000_000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_watts_gigawatt() {
            assert!((watts_gigawatt(1_000_000_000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilowatt_watts() {
            assert!((kilowatt_watts(1.0) - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilowatt_megawatts() {
            assert!((kilowatt_megawatts(1000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilowatt_gigawatts() {
            assert!((kilowatt_gigawatts(1_000_000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_megawatts_watts() {
            assert!((megawatts_watts(1.0) - 1_000_000.0).abs() < 1e-9);
        }

        #[test]
        fn test_megawatts_kilowatts() {
            assert!((megawatts_kilowatts(1.0) - 1000.0).abs() < 1e-9);
        }

        #[test]
        fn test_megawatts_gigawatts() {
            assert!((megawatts_gigawatts(1000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_gigawatts_watts() {
            assert!((gigawatts_watts(1.0) - 1_000_000_000.0).abs() < 1e-3);
        }

        #[test]
        fn test_gigawatts_kilowatts() {
            assert!((gigawatts_kilowatts(1.0) - 1_000_000.0).abs() < 1e-6);
        }

        #[test]
        fn test_gigawatts_megawatts() {
            assert!((gigawatts_megawatts(1.0) - 1000.0).abs() < 1e-9);
        }
    }
}


pub mod energy {

    /// Converts joules to kilojoules.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::energy::joule_kilojoules(1000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn joule_kilojoules(joules: f64) -> f64 {
        let kilojoules = joules / 1000.0;
        kilojoules
    }

    /// Converts kilojoules to joules.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::energy::kilojoules_joules(1.0);
    /// assert_eq!(result, 1000.0);
    /// ```
    pub fn kilojoules_joules(kilojoules: f64) -> f64 {
        let joules = kilojoules * 1000.0;
        joules
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_joule_kilojoules() {
            assert!((joule_kilojoules(1000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_kilojoules_joules() {
            assert!((kilojoules_joules(1.0) - 1000.0).abs() < 1e-9);
        }
    }
}

pub mod pressure {

    /// Converts pascals to atmospheres.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::pressure::pascal_atm(101325.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn pascal_atm(pascal: f64) -> f64 {
        let atm = pascal / 101325.0;
        atm
    }

    /// Converts pascals to bar.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::pressure::pascal_bar(100000.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn pascal_bar(pascal: f64) -> f64 {
        let bar = pascal / 100000.0;
        bar
    }

    /// Converts atmospheres to pascals.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::pressure::atm_pascal(1.0);
    /// assert_eq!(result, 101325.0);
    /// ```
    pub fn atm_pascal(atm: f64) -> f64 {
        let pascal = atm * 101325.0;
        pascal
    }

    /// Converts atmospheres to bar.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::pressure::atm_bar(1.0);
    /// assert_eq!(result, 1.01325);
    /// ```
    pub fn atm_bar(atm: f64) -> f64 {
        let bar = atm * 1.01325;
        bar
    }

    /// Converts atmospheres to PSI.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::pressure::atm_psi(1.0);
    /// assert_eq!(result, 14.69594);
    /// ```
    pub fn atm_psi(atm: f64) -> f64 {
        let psi = atm * 14.69594;
        psi
    }

    /// Converts PSI to atmospheres.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::pressure::psi_atm(14.69594);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn psi_atm(psi: f64) -> f64 {
        let atm = psi / 14.69594;
        atm
    }

    /// Converts PSI to bar.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::pressure::psi_bar(1.0);
    /// assert_eq!(result, 0.0689476);
    /// ```
    pub fn psi_bar(psi: f64) -> f64 {
        let bar = psi * 0.0689476;
        bar
    }

    /// Converts bar to atmospheres.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::pressure::bar_atm(1.0);
    /// assert_eq!(result, 0.98692327);
    /// ```
    pub fn bar_atm(bar: f64) -> f64 {
        let atm = bar * 0.98692327;
        atm
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_pascal_atm() {
            assert!((pascal_atm(101325.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_pascal_bar() {
            assert!((pascal_bar(100000.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_atm_pascal() {
            assert!((atm_pascal(1.0) - 101325.0).abs() < 1e-6);
        }

        #[test]
        fn test_atm_bar() {
            assert!((atm_bar(1.0) - 1.01325).abs() < 1e-9);
        }

        #[test]
        fn test_atm_psi() {
            assert!((atm_psi(1.0) - 14.69594).abs() < 1e-9);
        }

        #[test]
        fn test_psi_atm() {
            assert!((psi_atm(14.69594) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_psi_bar() {
            assert!((psi_bar(1.0) - 0.0689476).abs() < 1e-9);
        }

        #[test]
        fn test_bar_atm() {
            assert!((bar_atm(1.0) - 0.98692327).abs() < 1e-9);
        }
    }
}

pub mod area {

    /// Converts square inches to square feet.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::area::sq_inches_sq_feet(144.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn sq_inches_sq_feet(sq_inches: f64) -> f64 {
        let sq_feet = sq_inches / 144.0;
        sq_feet
    }

    /// Converts square millimeters to square centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::area::sq_millimeters_sq_centimeters(100.0);
    /// assert_eq!(result, 1.0);
    /// ```
    pub fn sq_millimeters_sq_centimeters(sq_millimeters: f64) -> f64 {
        let sq_centimeters = sq_millimeters / 100.0;
        sq_centimeters
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_sq_inches_sq_feet() {
            assert!((sq_inches_sq_feet(144.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn test_sq_millimeters_sq_centimeters() {
            assert!((sq_millimeters_sq_centimeters(100.0) - 1.0).abs() < 1e-9);
        }
    }
}



pub mod temperature {

    /// Converts Fahrenheit to Celsius.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::temperature::fahrenheit_celsius(32.0);
    /// assert_eq!(result, 0.0);
    /// ```
    pub fn fahrenheit_celsius(fahrenheit: f64) -> f64 {
        let temp_final = ((fahrenheit - 32.0) * 5.0) / 9.0;
        temp_final
    }

    /// Converts Celsius to Fahrenheit.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::temperature::celsius_fahrenheit(0.0);
    /// assert_eq!(result, 32.0);
    /// ```
    pub fn celsius_fahrenheit(celsius: f64) -> f64 {
        let temp_final = (celsius * 1.8) + 32.0;
        temp_final
    }

    /// Converts Celsius to Kelvin.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::temperature::celsius_kelvin(0.0);
    /// assert_eq!(result, 273.15);
    /// ```
    pub fn celsius_kelvin(celsius: f64) -> f64 {
        let temp_final = celsius + 273.15;
        temp_final
    }

    /// Converts Fahrenheit to Kelvin.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::temperature::fahrenheit_kelvin(32.0);
    /// assert_eq!(result, 273.15);
    /// ```
    pub fn fahrenheit_kelvin(fahrenheit: f64) -> f64 {
        let temp_final = (fahrenheit - 32.0) / 1.8 + 273.15;
        temp_final
    }

    /// Converts Kelvin to Fahrenheit.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::temperature::kelvin_fahrenheit(273.15);
    /// assert_eq!(result, 32.0);
    /// ```
    pub fn kelvin_fahrenheit(kelvin: f64) -> f64 {
        let temp_final = ((kelvin - 273.15) * 9.0) / 5.0 + 32.0;
        temp_final
    }

    /// Converts Kelvin to Celsius.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = mathsharp::temperature::kelvin_celsius(273.15);
    /// assert_eq!(result, 0.0);
    /// ```
    pub fn kelvin_celsius(kelvin: f64) -> f64 {
        let temp_final = kelvin - 273.15;
        temp_final
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_fahrenheit_celsius() {
            assert!((fahrenheit_celsius(32.0) - 0.0).abs() < 1e-9);
            assert!((fahrenheit_celsius(212.0) - 100.0).abs() < 1e-9);
        }

        #[test]
        fn test_celsius_fahrenheit() {
            assert!((celsius_fahrenheit(0.0) - 32.0).abs() < 1e-9);
            assert!((celsius_fahrenheit(100.0) - 212.0).abs() < 1e-9);
        }

        #[test]
        fn test_celsius_kelvin() {
            assert!((celsius_kelvin(0.0) - 273.15).abs() < 1e-9);
        }

        #[test]
        fn test_fahrenheit_kelvin() {
            assert!((fahrenheit_kelvin(32.0) - 273.15).abs() < 1e-9);
        }

        #[test]
        fn test_kelvin_fahrenheit() {
            assert!((kelvin_fahrenheit(273.15) - 32.0).abs() < 1e-9);
        }

        #[test]
        fn test_kelvin_celsius() {
            assert!((kelvin_celsius(273.15) - 0.0).abs() < 1e-9);
        }
    }
}
