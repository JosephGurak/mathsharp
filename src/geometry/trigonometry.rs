pub mod trig {
    use std::fmt;

    /// Errors from invalid right-triangle inputs.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TrigError {
        /// A length or angle must be positive and finite.
        NonPositiveValue(&'static str),
        /// An acute angle in degrees must satisfy `0 < angle < 90`.
        AngleOutOfRange,
        /// The two given angles sum to `180°` or more.
        AnglesSumTooLarge,
        /// The hypotenuse must be strictly longer than the given leg.
        HypotenuseNotGreaterThanLeg,
    }

    impl fmt::Display for TrigError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TrigError::NonPositiveValue(name) => {
                    write!(f, "{name} must be positive and finite")
                }
                TrigError::AngleOutOfRange => {
                    write!(f, "each acute angle must satisfy 0 < angle < 90 degrees")
                }
                TrigError::AnglesSumTooLarge => {
                    write!(f, "the sum of the two angles must be less than 180 degrees")
                }
                TrigError::HypotenuseNotGreaterThanLeg => {
                    write!(f, "hypotenuse must be greater than the given leg")
                }
            }
        }
    }

    impl std::error::Error for TrigError {}

    #[derive(Debug, Clone, PartialEq)]
    enum RightTriangleData {
        TwoAnglesDeg { a: f64, b: f64 },
        TwoLegs { a: f64, b: f64 },
        LegAndHypotenuse { leg: f64, hypotenuse: f64 },
    }

    /// A right triangle described by one of three known partial configurations.
    #[derive(Debug, Clone, PartialEq)]
    pub struct RightTriangle {
        data: RightTriangleData,
    }

    impl fmt::Display for RightTriangle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.data {
                RightTriangleData::TwoAnglesDeg { a, b } => {
                    write!(f, "right triangle (angles {a}°, {b}°)")
                }
                RightTriangleData::TwoLegs { a, b } => {
                    write!(f, "right triangle (legs {a}, {b})")
                }
                RightTriangleData::LegAndHypotenuse { leg, hypotenuse } => {
                    write!(f, "right triangle (leg {leg}, hypotenuse {hypotenuse})")
                }
            }
        }
    }

    fn validate_positive(value: f64, name: &'static str) -> Result<(), TrigError> {
        if value <= 0.0 || !value.is_finite() {
            return Err(TrigError::NonPositiveValue(name));
        }
        Ok(())
    }

    fn validate_acute_angle_deg(angle: f64) -> Result<(), TrigError> {
        if angle <= 0.0 || angle >= 90.0 || !angle.is_finite() {
            return Err(TrigError::AngleOutOfRange);
        }
        Ok(())
    }

    impl RightTriangle {
        pub fn from_two_angles_deg(a: f64, b: f64) -> Result<Self, TrigError> {
            if !a.is_finite() || !b.is_finite() {
                return Err(TrigError::AngleOutOfRange);
            }
            if a + b >= 180.0 {
                return Err(TrigError::AnglesSumTooLarge);
            }
            validate_acute_angle_deg(a)?;
            validate_acute_angle_deg(b)?;
            Ok(Self {
                data: RightTriangleData::TwoAnglesDeg { a, b },
            })
        }

        pub fn from_two_legs(a: f64, b: f64) -> Result<Self, TrigError> {
            validate_positive(a, "leg a")?;
            validate_positive(b, "leg b")?;
            Ok(Self {
                data: RightTriangleData::TwoLegs { a, b },
            })
        }

        pub fn from_leg_hypotenuse(leg: f64, hypotenuse: f64) -> Result<Self, TrigError> {
            validate_positive(leg, "leg")?;
            validate_positive(hypotenuse, "hypotenuse")?;
            if hypotenuse <= leg {
                return Err(TrigError::HypotenuseNotGreaterThanLeg);
            }
            Ok(Self {
                data: RightTriangleData::LegAndHypotenuse { leg, hypotenuse },
            })
        }

        pub fn third_angle_deg(&self) -> Option<f64> {
            match self.data {
                RightTriangleData::TwoAnglesDeg { a, b } => Some(180.0 - a - b),
                _ => None,
            }
        }

        pub fn hypotenuse(&self) -> Option<f64> {
            match self.data {
                RightTriangleData::TwoLegs { a, b } => Some((a * a + b * b).sqrt()),
                RightTriangleData::LegAndHypotenuse { hypotenuse, .. } => Some(hypotenuse),
                _ => None,
            }
        }

        pub fn missing_leg(&self) -> Option<f64> {
            match self.data {
                RightTriangleData::LegAndHypotenuse { leg, hypotenuse } => {
                    Some((hypotenuse * hypotenuse - leg * leg).sqrt())
                }
                _ => None,
            }
        }
    }

    pub fn deg_to_rad(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    pub fn rad_to_deg(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn third_angle_from_two_acute_angles() {
            let triangle = RightTriangle::from_two_angles_deg(30.0, 60.0).unwrap();
            assert_eq!(triangle.third_angle_deg(), Some(90.0));
        }

        #[test]
        fn hypotenuse_from_two_legs_3_4_5() {
            let triangle = RightTriangle::from_two_legs(3.0, 4.0).unwrap();
            assert!((triangle.hypotenuse().unwrap() - 5.0).abs() < 1e-9);
        }

        #[test]
        fn missing_leg_from_leg_and_hypotenuse() {
            let triangle = RightTriangle::from_leg_hypotenuse(3.0, 5.0).unwrap();
            assert!((triangle.missing_leg().unwrap() - 4.0).abs() < 1e-9);
        }

        #[test]
        fn rejects_non_positive_legs() {
            assert!(RightTriangle::from_two_legs(0.0, 4.0).is_err());
        }
    }
}
