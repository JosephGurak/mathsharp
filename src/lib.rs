//! #MathSharp
//! 
//! `mathsharp` is a collection of mathematical functions and resources to assist in solving a wide range of topics.

mod unit_conversions;

pub use self::unit_conversions::area;
pub use self::unit_conversions::energy;
pub use self::unit_conversions::mass;
pub use self::unit_conversions::power;
pub use self::unit_conversions::pressure;
pub use self::unit_conversions::time;
pub use self::unit_conversions::volume;
pub use self::unit_conversions::length;

// methods declared this way as part of legacy before new organizaion structure. at major version convert this but for now leave as it would break
pub use self::unit_conversions::temperature;
pub use self::unit_conversions::temperature::celsius_fahrenheit;
pub use self::unit_conversions::temperature::celsius_kelvin;
pub use self::unit_conversions::temperature::fahrenheit_celsius;
pub use self::unit_conversions::temperature::fahrenheit_kelvin;
pub use self::unit_conversions::temperature::kelvin_celsius;
pub use self::unit_conversions::temperature::kelvin_fahrenheit;

/////////////////////////////////////////////////////
pub mod geometry {
    pub mod geometric_calculations;
    
}

pub use self::geometry::geometric_calculations::perimeter;
pub use self::geometry::geometric_calculations::areas;
pub use self::geometry::geometric_calculations::volumes;




/////////////////////////////////////////////////////

pub mod linear_algebra {
    pub mod matrices;
}

pub use self::linear_algebra::matrices::m2x2;
pub use self::linear_algebra::matrices::m3x3;


/////////////////////////////////////////////////////


