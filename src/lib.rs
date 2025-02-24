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
pub use self::unit_conversions::temperature;

/////////////////////////////////////////////////////
mod geometric_calculations;

pub use geometric_calculations::perimeter;





