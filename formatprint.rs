fn main() {
    // {} is a placeholder that will automatically replace any arguments
    println!("{} days", 31);

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Carter", "Texas", "code");

    // Named arguments
    println!("{subject} {verb} {object}", object = "the lazy dog", subject = "the quick brown fox", verb = "jumps over");

    // Different formatting can be invoked by specifying a format character
    // after a `:`.
    println!("Base 10: {}", 69420);
    println!("Base 2: {:b}", 69420); // 10001001101011100
    println!("Base 8: {:o}", 69420); // 21154   
    println!("Base 16: {:X}", 69420); // 1195C

    // Padding with zeros
    println!("{number:0>width$}", number = 1, width = 6);

    // Left adjust by flipping the sign, ,  This will output: "10000".
    println!("{number:>width$}", number = 1, width = 6);

    // Rust will also check if the type of the placeholder matches the type of the argument.
    // This will fail because we're trying to print a string but the placeholder expects a number.
    // println!("{} of {:b} people know binary, the other half doesn't", "one", 2);

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
