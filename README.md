Rust Crate for Working with Mathematical Operations 

The Goal of mathsharp is to provide a fast and friendly experience for
anyone needing to use math in their projects, from unit converisons to 
calculus and beyond. 

## Example

Below example demonstrates using a temperature conversion function.

```rust
use mathsharp::fahrenheit_celsius;

fn main() {
    //declare value you want to convert
    let from_fahrenheit = 32.0;

    let to_celsius = fahrenheit_celsius(from_fahrenheit);
    
    println!("{} fahrenheit is equal to {} Celsius", from_fahrenheit, to_celsius);   
}
```
## Latest Release
    Version 0.6.0 brings 
    -standardized on f64
    -our start to trigonometric solving
    -Basic Matrix operations for 2x2 and 3x3
    
    Bug Fixes:
    - 3 fucntions were incorrectly using a division operation instead of multiplication
    length::yards_feet
    length::yards_inches
    volume::gallon_to_fluid_ounces

                       

### Website On its way! 
MathSharp website will be launched to create a place for more documentation, math resources and a special surprise!



