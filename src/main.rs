use mathsharp::celsius_fahrenheit;
use mathsharp::fahrenheit_celsius;


fn main() {


    let from_fahrenheit = 32.0;
    let to_celsius = fahrenheit_celsius(from_fahrenheit);
    
    println!("{} fahrenheit is equal to {} Celsius", from_fahrenheit, to_celsius);   

    ////

    let from_celsius = 0.0;
    let to_fahrenheit = celsius_fahrenheit(from_celsius);
    
    println!("{} Celsius is equal to {} fahrenheit", from_celsius, to_fahrenheit);
   
}
