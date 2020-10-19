mod io_prompts;
mod logic;

fn main() {
    println!("{}\n", io_prompts::hangmen::get_ascii_hangman_logo());
    println!("Hey, let's play a friendly round of Hangmman!");
    println!("(Hope you don't die...)\n\n");

    // TODO: 
    // - Build up game logic.
    // - Make the game failsafe, no matter the input.

    loop {
        println!("Write the word to be guessed. No special symbols, use the standard 26 letter alphabet only.");

        let mut chosen_word: String = String::new();
        let _ = std::io::stdin().read_line(&mut chosen_word).expect("Failed to read line.");

        let _state = logic::state::init(&chosen_word);

        println!("{}", io_prompts::hangmen::get_hangman(0));

        /*
        (https://stackoverflow.com/questions/34837011/how-to-clear-terminal-screen-in-rust-after-new-line-is-printing)
        You can send a control character to clear the terminal screen.

        fn main() {
            print!("{}[2J", 27 as char);
        }

        Or to also positions the cursor at row 1, column 1:
        */
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        //print!("\x1B[2J\x1B[1;1H");

        println!("{}", io_prompts::hangmen::get_hangman(9));

        if !io_prompts::another_round::wanna_play() 
        {
            break;
        }
    }

    println!("Thanks for playing, have a nice day! :-)");
}
