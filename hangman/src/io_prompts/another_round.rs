pub fn wanna_play() -> bool 
{
    let mut input = String::new();
    let response: bool;

    println!("Would you like to play another round?");
    println!("Yes : Y   ||   No : N");
    
    loop {
        // read line and store in input variable
        let _ = std::io::stdin().read_line(&mut input).expect("Failed to read line.");

        // NOTE: .trim() is important to chave of line-break characters from input formatting
        if input.trim() == "Y" 
        {
            response = true;
            break;
        }
        else if input.trim() == "N"
        {
            response = false;
            break;
        }
        else
        {
            println!("Your response was invalid. Sorry, please try again.");
            println!("Yes : Y   ||   No : N");
        }
    }
    response
}
