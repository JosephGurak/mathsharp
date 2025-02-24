

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

    //imperial to metric
    pub fn inches_millimeters(inches: f32) -> f32 {
        let millimeters = inches * 25.4;
        millimeters
    }

    //metric to imperial
    pub fn millimeters_inches(millimeter: f32) -> f32 {
        let inches = millimeter / 25.4;
        inches
    }

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


pub mod time {
    pub fn seconds_minutes(seconds: f32) -> f32 {
        let minutes = seconds / 60.0;
        minutes
    }

    pub fn seconds_hours(seconds: f32) -> f32 {
        let hours = seconds / 3600.0;
        hours
    }

    pub fn seconds_days(seconds: f32) -> f32 {
        let days = seconds / 86400.0;
        days
    }

    //
    pub fn minutes_seconds(minutes: f32) -> f32 {
        let seconds = minutes * 60.0;
        seconds
    }

    pub fn minutes_hours(minutes: f32) -> f32 {
        let hours = minutes / 60.0;
        hours
    }

    pub fn minutes_days(minutes: f32) -> f32 {
        let days = minutes / 1440.0;
        days
    }
    //

    pub fn hours_seconds(hours: f32) -> f32 {
        let seconds = hours * 3600.0;
        seconds
    }

    pub fn hours_minutes(hours: f32) -> f32 {
        let minutes = hours * 60.0;
        minutes
    }

    pub fn hours_days(hours: f32) -> f32 {
        let days = hours / 24.0;
        days
    }

    //

    pub fn days_seconds(days: f32) -> f32 {
        let seconds = days * 86400.0;
        seconds
    }

    pub fn days_minutes(days: f32) -> f32 {
        let minutes = days * 1440.0;
        minutes
    }

    pub fn days_hours(days: f32) -> f32 {
        let hours = days * 24.0;
        hours
    }
}


pub mod power {
    pub fn horsepower_watts(horsepower: f32) -> f32 {
        let watts = horsepower * 746.0;
        watts
    }

    pub fn watts_horsepower(watts: f32) -> f32 {
        let horsepower = watts / 746.0;
        horsepower
    }

    pub fn watts_kilowatt(watts: f32) -> f32 {
        let kilowatt = watts / 1000.0;
        kilowatt
    }

    pub fn watts_megawatt(watts: f32) -> f32 {
        let megawatt = watts / 1000000.0;
        megawatt
    }

    pub fn watts_gigawatt(watts: f32) -> f32 {
        let gigawatt = watts / 1000000000.0;
        gigawatt
    }

    pub fn kilowatt_watts(kilowatt: f32) -> f32 {
        let watts = kilowatt * 1000.0;
        watts
    }

    pub fn kilowatt_megawatts(kilowatt: f32) -> f32 {
        let megawatts = kilowatt / 1000.0;
        megawatts
    }

    pub fn kilowatt_gigawatts(kilowatt: f32) -> f32 {
        let gigawatts = kilowatt / 1000000.0;
        gigawatts
    }

    pub fn megawatts_watts(megawatts: f32) -> f32 {
        let watts = megawatts * 1000000.0;
        watts
    }

    pub fn megawatts_kilowatts(megawatts: f32) -> f32 {
        let kilowatts = megawatts * 1000.0;
        kilowatts
    }

    pub fn megawatts_gigawatts(megawatts: f32) -> f32 {
        let gigawatts = megawatts * 1000.0;
        gigawatts
    }

    pub fn gigawatts_watts(gigawatts: f64) -> f64 {
        let watts = gigawatts * 1000000000.0;
        watts
    }
    
    pub fn gigawatts_kilowatts(gigawatts: f64) -> f64 {
        let kilowatts = gigawatts * 1000000.0;
        kilowatts
    }

    pub fn gigawatts_megawatts(gigawatts: f64) -> f64 {
        let megawatts = gigawatts * 1000.0;
        megawatts
    }
}


pub mod energy {
    pub fn joule_kilojoules(joules: f32) -> f32 {
        let kilojoules = joules / 1000.0;
        kilojoules
    }

    pub fn kilojoules_joules(kilojoules: f32) -> f32 {
        let joules = kilojoules * 1000.0;
        joules
    }

}

pub mod pressure {
    pub fn pascal_atm(pascal: f32) -> f32 {
        let atm = pascal / 101325.0;
        atm
    }

    pub fn pascal_bar(pascal: f32) -> f32 {
        let bar = pascal / 100000.0;
        bar
    }

    pub fn atm_pascal(atm: f32) -> f32 {
        let pascal = atm * 101325.0;
        pascal
    }

    pub fn atm_bar(atm: f32) -> f32 {
        let bar = atm * 1.01325;
        bar
    }
    
    pub fn atm_psi(atm: f32) -> f32 {
        let psi = atm * 14.69594;
        psi
    }

    pub fn psi_atm(psi: f32) -> f32 {
        let atm = psi / 14.69594;
        atm
    }

    pub fn psi_bar(psi: f32) -> f32 {
        let bar = psi * 0.0689476;
        bar
    }

    pub fn bar_atm(bar: f32) -> f32 {
        let atm = bar * 0.98692327;
        atm
    }

}

pub mod area {

    // imperial 
    pub fn sq_inches_sq_feet(sq_inches: f32) -> f32 {
        let sq_feet = sq_inches / 144.0;
        sq_feet
    }

    // metric
    pub fn sq_millimeters_sq_centimeters(sq_millimeters: f32) -> f32 {
        let sq_centimeters = sq_millimeters / 100.0;
        sq_centimeters
    }
    //conversion of imperial to metric, vice versa
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