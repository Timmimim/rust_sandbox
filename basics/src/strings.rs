// Primitive Strings str: Immutable fixed-length string somewhere in memory (hard-coded)
// String = Growable, heap-allocated data structure - use when modification wanted/needed

pub fn run()
{
    let mut hello = String::from("Hello");

    // Get length
    println!("{}\nLength: {}", hello, hello.len());

    // Push chars (needs to be mutable AND String type)
    hello.push(' ');
    hello.push('W');
    // push entire string
    hello.push_str("orld");

    println!("{}\nNew Length: {}", hello, hello.len());

    // Capacity in Bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' Substring: {}", hello.contains("World"));
    
    // Replace
    println!("Replace 'World' with 'There': {}", hello.replace("World", "There"));

    // Loop over string by white spaces
    for word in hello.split_whitespace()
    {
        println!("{}", word);
    }

    // Create String with certain initial capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion Testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}