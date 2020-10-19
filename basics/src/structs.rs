// Structs - used to create custom data (structures) (optionally with own functionality)

// Traditional Struct
struct Colour 
{
    red : u8,
    green : u8,
    blue : u8,
}

// Tuple Struct
struct Colour2 (u8, u8, u8);

struct Person 
{
    first_name : String,
    last_name : String,
}

impl Person 
{
    // Construct Person
    fn new(first: &str, last: &str) -> Person 
    {
        Person {
            first_name : first.to_string(),
            last_name : last.to_string()
        }
    }

    // Get Full Name
    fn get_full_name(&self) -> String
    {
        format!("{} {}", self.first_name, self.last_name)
    }
    
    // Change First Name
    fn set_first_name(&mut self, new_first: &str)
    {
        self.first_name = new_first.to_string();
    }
    
    // Change Last Name
    fn set_last_name(&mut self, new_last: &str)
    {
        self.last_name = new_last.to_string();
    }

    fn to_tuple(self) -> (String, String)
    {
        (self.first_name, self.last_name)
    }
}

pub fn run()
{
    let mut c = Colour {
        red : 255,
        green : 0,
        blue : 0
    };

    c.red = 200;

    println!("Colour: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Colour2(255, 0, 0);
    c2.0 = 200;

    println!("Colour Tuple: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Junk", "Spearow");
    println!("Person: {}", p.get_full_name());

    p.set_first_name("Jack");
    p.set_last_name("Sparrow");
    println!("But actually: {}", p.get_full_name());

    println!("Person Tuple: {:?}", p.to_tuple());


}