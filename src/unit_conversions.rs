

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

    pub fn inches_mile(inches: f32) -> f32 {
        let mile = inches / 63360.0;
        mile
    }

    pub fn feet_inches(feet: f32) -> f32 {
        let inches = feet * 12.0;
        inches
    }

    pub fn feet_yards(feet: f32) -> f32 {
        let yards = feet / 3.0;
        yards
    }

    pub fn yards_feet(yards: f32) -> f32 {
        let feet = yards / 3.0;
        feet
    }

    pub fn yards_inches(yards: f32) -> f32 {
        let inches = yards / 36.0;
        inches
    }

    pub fn yards_mile(yards: f32) -> f32 {
        let mile = yards / 1760.0;
        mile
    }

    pub fn miles_yards(miles: f32) -> f32 {
        let yards = miles * 1760.0;
        yards
    }

    pub fn feet_mile(feet: f32) -> f32 {
        let mile = feet / 5280.0;
        mile
    }

    pub fn mile_feet(mile: f32) -> f32 {
        let feet = mile * 5280.0;
        feet
    }

    pub fn mile_inches(mile: f32) -> f32 {
        let inches = mile * 63360.0;
        inches
    }
    //still need to do imperial to metric and vice versa conversions



    //

    // Metric
    pub fn millimeter_centimeter(millimeter: f32) -> f32 {
        let centimeter = millimeter / 10.0;
        centimeter
    } 

    pub fn millimeter_meter(millimeter: f32) -> f32 {
        let meter = millimeter / 1000.0;
        meter
    } 

    pub fn millimeter_kilometer(millimeter: f32) -> f32 {
        let kilometer = millimeter / 1000000.0;
        kilometer
    }

    pub fn centimeter_millimeter(centimeter: f32) -> f32 {
        let millimeter = centimeter * 10.0;
        millimeter
    } 

    pub fn centimeter_meter(centimeter: f32) -> f32 {
        let meter = centimeter / 100.0;
        meter
    }

    pub fn centimeter_kilometer(centimeter: f32) -> f32 {
        let kilometer = centimeter / 100000.0;
        kilometer
    }

    pub fn meters_centimeter(meter: f32) -> f32 {
        let centimeter = meter * 100.0;
        centimeter
    }

    pub fn meters_millimeter(meter: f32) -> f32 {
        let millimeter = meter * 1000.0;
        millimeter
    }

    pub fn meters_kilometer(meter: f32) -> f32 {
        let kilometer = meter / 1000.0;
        kilometer
    }

    pub fn kilometer_meters(kilometer: f32) -> f32 {
        let meters = kilometer * 1000.0;
        meters
    }

    pub fn kilometer_centimeters(kilometer: f32) -> f32 {
        let centimeters = kilometer * 100000.0;
        centimeters
    }

    pub fn kilometer_millimeters(kilometer: f32) -> f32 {
        let millimeters = kilometer * 1000000.0;
        millimeters
    }


}

pub mod mass {

}

pub mod volume {

}

pub mod area {

}

pub mod time {

}

pub mod pressure {

}

pub mod energy {

}

pub mod power {
    
}


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