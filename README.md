# mathsharp
Rust Crate for Working with Mathematical Operations 

The Goal of mathsharp is to provide a fast and friendly experience for
anyone needing to use math in their projects, from unit converisons to 
calculus and beyond. 

## Example

Below example demonstrates using a temperature conversion function.

```
use mathsharp::fahrenheit_celsius;

fn main() {
    let from_fahrenheit = 32.0;
    let to_celsius = fahrenheit_celsius(from_fahrenheit);
    
    println!("{} fahrenheit is equal to {} Celsius", from_fahrenheit, to_celsius);   
}
```

## Roadmap
 

MathSharp aims to add areas of mathematics in blocks each with consistent monthly updates.

### BLOCK 1:  Unit Conversions

```
    March 1st Update:  
                        length, mass, volume, area, time, pressure, energy, power
```

### BLOCK 2: Geometric Calculations

```
    April 1st Update: 
                        perimeter, area, volume, triangles, circles, quadrilaterals
                        
    May 1st Update:
                        trigonometry
```

### BLOCK 3: Algebra 

```
    June 1st Update: TBD
                        
```


#### More Blocks will be added and existing ones updated as MathSharp progresses towards its goal

### Website On its way! 
Around the time of BLOCK 3 MathSharp website will be launched using the Dioxus framework to create a place for more documentation, math resources and a special surprise!



