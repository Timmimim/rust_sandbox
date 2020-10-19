pub fn run()
{
    let age: i8 = 27;
    let id_checked: bool = true;

    // If/Else
    if age >= 21 && id_checked
    {
        println!("Bartender: What would you like to drink?");
    }
    else if age < 21 && id_checked
    {
        println!("Bartender: I'm sorry, you need to leave.");
    }
    else
    {
        println!("Bartender: I need to see some ID, please.");
    }

    // Shorthand If 
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}