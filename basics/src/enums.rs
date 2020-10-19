// Pretend: Game, with Avatar able to move in certain ways
enum Movement
{
    // Variants
    Up,
    Down, 
    Left,
    Right,
}

fn move_avatar(m: Movement)
{
    // Perform Action depending on Info
    match m     // similar to switch-case
    {
        Movement::Up => println!("Avatar moving up."),
        Movement::Down => println!("Avatar moving down."),
        Movement::Left => println!("Avatar moving left."),
        Movement::Right => println!("Avatar moving right."),
    }
}

pub fn run()
{
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    // move_avatar(avatar1)  will NOT work a second time; fn takes ownership of data, avatar1 is moved and invalid later
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}