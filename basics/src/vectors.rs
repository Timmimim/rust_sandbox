// Fixed-length list of elements of same type

// shortcut -> "It's always good to get rid of STDs." :D
use std::mem;

pub fn run()
{
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value -> MUST BE mut
    numbers[2] = 30;

    // add on to Vector
    numbers.push(6);
    numbers.push(7);

    // remove last entry
    numbers.pop();

    //  get all vals
    println!("{:?}", numbers);

    // get single value
    println!("Single Value: {}", numbers[0]);

    // Get Vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack-allocated
    println!("Vector occupies {} bytes.", mem::size_of_val(&numbers));

    // Get you a Slice (1..4 returns items at index 1, 2, 3 -> Slice ends before 4)
    let slice: &[i32] = &numbers[1..5];
    println!("Slice: {:?}", slice);

    // Loop & Print
    for x in numbers.iter()
    {
        println!("Number: {}", x);
    }
    
    // Loop & mutate values
    for x in numbers.iter_mut()
    {
        *x *= 2;
    }
    println!("Numbers Vec Mutated: {:?}", numbers);
}