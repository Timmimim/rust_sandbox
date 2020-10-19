// Fixed-length list of elements of same type

// shortcut -> "It's always good to get rid of STDs." :D
use std::mem;

pub fn run()
{
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value -> MUST BE mut
    numbers[2] = 30;

    //  get all vals
    println!("{:?}", numbers);

    // get single value
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack-allocated
    println!("Array occupies {} bytes.", mem::size_of_val(&numbers));

    // Get you a Slice (1..4 returns items at index 1, 2, 3 -> Slice ends before 4)
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);
}