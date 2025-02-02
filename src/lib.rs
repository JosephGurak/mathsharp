//! #MathSharp
//! 
//! `mathsharp` is a collection of mathematical functions and resources to assist in solving a wide range of topics.



pub use self::temperature::celsius_fahrenheit;
pub use self::temperature::fahrenheit_celsius;
pub use self::temperature::celsius_kelvin;
pub use self::temperature::fahrenheit_kelvin;
pub use self::temperature::kelvin_fahrenheit;
pub use self::temperature::kelvin_celsius;

pub use self::length::inches_feet;

pub mod temperature {
    /// Converts Fahrenheit to Celsius.
    /// 
    /// #Examples
    /// 
    /// ```
    /// let fahrenheit = 32.0;
    /// let answer = mathsharp::fahrenheit_celsius(fahrenheit);
    /// 
    /// assert_eq!(0.0, answer);
    /// ```
    pub fn fahrenheit_celsius(fahrenheit: f32) -> f32 {         
        let temp_final = ((fahrenheit - 32.0) * 5.0)/9.0;

        //println!("Celsius: {:?}", temp_final);

        temp_final
    }



    /// Converts Celsius to Fahrenheit.
    /// 
    /// #Examples
    /// 
    /// ```
    /// let celsius = 0.0;
    /// let answer = mathsharp::celsius_fahrenheit(celsius);
    /// 
    /// assert_eq!(32.0, answer);
    /// ```

    pub fn celsius_fahrenheit(celsius: f32) -> f32 {
        let temp_final = (celsius * 1.8) + 32.0;

        temp_final
        
    }

    /// Converts Celsius to Kelvin.
    /// 
    /// #Examples
    /// 
    /// ```
    /// let celsius = 0.0;
    /// let answer = mathsharp::celsius_kelvin(celsius);
    /// 
    /// assert_eq!(273.15, answer);
    /// ```

    pub fn celsius_kelvin(celsius: f32) -> f32 {
        let temp_final = celsius + 273.15;

        temp_final
        
    }

    /// Converts Fahrenheit to Kelvin.
    /// 
    /// #Examples
    /// 
    /// ```
    /// let fahrenheit = 32.0;
    /// let answer = mathsharp::fahrenheit_kelvin(fahrenheit);
    /// 
    /// assert_eq!(273.15, answer);
    /// ```

    pub fn fahrenheit_kelvin(fahrenheit: f32) -> f32 {
        let temp_final = (fahrenheit - 32.0)/1.8 + 273.15;

        temp_final
        
    }
    
    /// Converts Kelvin to Fahrenheit.
    /// 
    /// #Examples
    /// 
    /// ```
    /// let kelvin = 310.15;
    /// let answer = mathsharp::kelvin_fahrenheit(kelvin);
    /// 
    /// assert_eq!(98.6, answer);
    /// ```

    pub fn kelvin_fahrenheit(kelvin: f32) -> f32 {
        let temp_final = ((kelvin - 273.15)*9.0)/5.0 + 32.0;

        temp_final
        
    }

    /// Converts Kelvin to celsius.
    /// 
    /// #Examples
    /// 
    /// ```
    /// let kelvin = 273.15;
    /// let answer = mathsharp::kelvin_celsius(kelvin);
    /// 
    /// assert_eq!(0.0, answer);
    /// ```

    pub fn kelvin_celsius(kelvin: f32) -> f32 {
        let temp_final = kelvin - 273.15;

        temp_final
        
    }

}

pub mod length {

    /// Converts Inches to Feet.
    ///
    /// #Examples
    ///
    /// ```
    /// let inches = 36.0;
    /// let answer = mathsharp::inches_feet(inches);
    ///
    /// assert_eq!(3.0, answer);
    /// ```
    pub fn inches_feet(inches: f32) -> f32 {
        let feet = inches / 12.0;
        feet
    }
}
