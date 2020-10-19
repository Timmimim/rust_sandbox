pub fn run()
{
    greeting("Hello", "Charlize");

    // Bind function values to variable
    let get_sum = add (5, 6);
    println!("Sum of 5 & 6: {}", get_sum);

    // Closure
    let add_nums = |n1 : i32, n2 : i32| n1 + n2;
    println!("Closure Sum: {}", add_nums(3,4));
    println!("Other Closure Sum: {}", add_nums(46,23));

    let n3: i32 = 10;
    println!("Closure Sum w/ outside variable: {}", add_nums(3,4) + n3);
}

fn greeting(greet: &str, name: &str)
{
    println!("{} {}, nice to meet you!", greet, name);
}

fn add (n1: i32, n2: i32) -> i32
{
    n1 + n2     // no semi-colon means: this is what we wanna return!
}