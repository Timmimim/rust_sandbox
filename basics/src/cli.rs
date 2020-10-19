use std::env;

pub fn run()
{
    let args: Vec<String> = env::args().collect();

    if args.len() > 1
    {
        let command = args[1].clone();

        println!("Args: {:?}", args);
        println!("Args: {}", command);
        
        let name = "Funny Face";
        let status = "100%";

        if command == "hello"
        {
            println!("Hey back, {}, how are you?", name);
        }
        else if command == "status"
        {
            println!("Status is {}", status);
        }
        else
        {
            println!("That is not a valid input, silly Billy.");
        }
    }
    else
    {
        println!("Enter one of these extra commands: hello | status");
    }
}