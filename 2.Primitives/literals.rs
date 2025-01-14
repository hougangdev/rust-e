// Rust has a wide variety of literals.
// Samples include signed integers, unsigned integers, floating point, char scalar values, boolean, the unit type (), who's only possible value is and empty tuple.

// Defaults to i32, i64, u32, u64, f32, f64, bool, char, and ()

fn main() {
    // Signed integers (i8, i16, i32, i64, i128)
    let x: i32 = 10;
    let y: i64 = 100;
    let z: i128 = 1000;
    println!("Signed integers: x = {}, y = {}, z = {}", x, y, z);

    // Unsigned integers (u8, u16, u32, u64, u128)
    let a: u8 = 10;
    let b: u16 = 100;
    let c: u32 = 1000;
    let d: u64 = 10000;
    let e: u128 = 100000;
    println!("Unsigned integers: a = {}, b = {}, c = {}, d = {}, e = {}", 
             a, b, c, d, e);

    // Floating point numbers
    let f: f32 = 10.0;
    let g: f64 = 100.0;
    println!("Floating point numbers: f = {}, g = {}", f, g);

    // Boolean
    let h: bool = true;
    println!("Boolean value: h = {}", h);

    // Character
    let i: char = 'R';
    println!("Character: i = {}", i);

    // Unit type
    let j: () = ();
    println!("Unit type exists but prints nothing: {:?}", j);

    // String slice
    let k: &str = "Hello, world!";
    println!("String slice: k = {}", k);

    // Tuple
    let l: (i32, i32) = (10, 20);
    println!("Tuple: l = {:?}", l);
    println!("Tuple values: l.0 = {}, l.1 = {}", l.0, l.1);

    // Tuple is a collection of values of different types 
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);
    println!("Tuple: my_tuple = {:?}", my_tuple);
    println!("Tuple values: my_tuple.0 = {}, my_tuple.1 = {}, my_tuple.2 = {}, my_tuple.3 = {}", 
             my_tuple.0, my_tuple.1, my_tuple.2, my_tuple.3);  
}
