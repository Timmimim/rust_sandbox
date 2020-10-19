pub fn run()
{
    // Variables are IMMUTABLE by default
    let name = "Jimmy";     // cannot be changed
    let mut age = 69;       // this can, keyword: mut (MUTable)

    println!("My name is {} and I am {} years old.", name, age);
    age += 1;
    println!("Next year I'll be {}.", age);

    // Define constant (usually upper case, type must be assigned explicitely)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables at once
    let ( my_name, my_age ) = ("Timmothy", 27);
    println!("{} is {}", my_name, my_age);
}