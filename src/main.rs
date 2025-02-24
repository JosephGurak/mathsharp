use mathsharp::temperature::celsius_fahrenheit;
use mathsharp::temperature::fahrenheit_celsius;
use mathsharp::length::inches_feet;
use mathsharp::perimeter::triangle;


fn main() {

    //
    
    triangle();

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
