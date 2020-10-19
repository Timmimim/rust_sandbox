pub fn run() 
{
    // Print to Console
    println!("Hello from the Other Side (print.rs file)!");

    // Basic Formatting
    println!("Number: {}", 1);

    // Positional Arguments
    println!("{0} is from {1} and {0} is also {2}.", 
        "Bobby", "Mars", "a funny boy haha"
    );

    // Named Arguments
    println!("{name} likes to play {activity}.", 
        name = "John", 
        activity = "hide the pen"
    );

    // Placeholder Traits
    println!("Binary: {:b}\nHex: {:x}\nOctal: {:o}", 10, 10, 10);

    // Placeholder for Debug Trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10+10);
}