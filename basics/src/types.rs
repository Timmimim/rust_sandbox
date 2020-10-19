/* 
    Primitive Types:
        Integers:   u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
        Floats:     f32, f64
        Boolean:    bool
        Charactes:  char
        Tuples
        Arrays
*/

// RustCompiler can (most of the time) infer required data types at compile time.
// Must be hand-specified to fit specific needs (size)

pub fn run()
{
    let x = 1;  // default: i32
    let y = 2.5; // default: f64

    // Add explicit type decl:
    let z : i64 = 123456789;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // get Boolean from Expression
    let is_greater: bool = 10 > 5;

    // use chars
    let a1: char = 'a';
    let face: char = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}