use mathsharp::celsius_fahrenheit;
use mathsharp::fahrenheit_celsius;
use mathsharp::length::inches_feet;
use mathsharp::perimeter::*;


fn main() {

    let circle_test = perimeter_circle(4.0);
    println!("{circle_test} units");

    //
    
    let triangle_perimeter = perimeter_triangle(4.0,2.0,3.0);
    println!("{triangle_perimeter} units");
    //
    let inches = 12.0;
    let to_feet = inches_feet(inches);
    println!("Feet equals: {}", to_feet);

    //

    let from_fahrenheit = 32.0;
    let to_celsius = fahrenheit_celsius(from_fahrenheit);
    
    println!("{} fahrenheit is equal to {} Celsius", from_fahrenheit, to_celsius);   

    ////

    let from_celsius = 0.0;
    let to_fahrenheit = celsius_fahrenheit(from_celsius);
    
    println!("{} Celsius is equal to {} fahrenheit", from_celsius, to_fahrenheit);
   
}
