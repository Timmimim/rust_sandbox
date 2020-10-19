pub fn get_ascii_hangman_logo() -> String
{
    HANGMAN_LOGO.to_string()
}

pub fn get_hangman(status: u8) -> String
{
    match status 
    {
        0 => return HANGMAN_0.to_string(),
        1 => return HANGMAN_1.to_string(),
        2 => return HANGMAN_2.to_string(),
        3 => return HANGMAN_3.to_string(),
        4 => return HANGMAN_4.to_string(),
        5 => return HANGMAN_5.to_string(),
        6 => return HANGMAN_6.to_string(),
        7 => return HANGMAN_7.to_string(),
        8 => return HANGMAN_8.to_string(),
        9 => return HANGMAN_FULL.to_string(),
        _ => panic!("Invalid numeric input, nothing can be returned. Input was: {}", status),
    }
}

const HANGMAN_LOGO : &str = "
 _                                             
| |                                            
| |__   __ _ _ __   __ _ _ __ ___   __ _ _ __  
| '_ \\ / _` | '_ \\ / _` | '_ ` _ \\ / _` | '_ \\ 
| | | | (_| | | | | (_| | | | | | | (_| | | | |
|_| |_|\\__,_|_| |_|\\__, |_| |_| |_|\\__,_|_| |_|
                    __/ |                      
                   |___/                       
";

const HANGMAN_0 : &str = "







_________________
=================
Lösungswort :    ";

const HANGMAN_1 : &str = "






|=============
|___________||___
=================
Lösungswort :    ";

const HANGMAN_2 : &str = "

|
|
|
|
|
|=============
|___________||___
=================
Lösungswort :    ";

const HANGMAN_3 : &str = "
__________
|       
| 
|
|
|
|=============
|___________||___
=================
Lösungswort :    ";

const HANGMAN_4 : &str = "
__________
|  /
| /
|/
|
|    
|=============
|___________||___
=================
Lösungswort :    ";

const HANGMAN_5 : &str = "
__________
|  /     |
| /      |
|/      
|       
|     
|=============
|___________||___
=================
Lösungswort :    ";

const HANGMAN_6 : &str = "
__________
|  /     |
| /      |
|/       |
|        |
|       
|=============
|___________||___
=================
Lösungswort :    ";

const HANGMAN_7 : &str = "
__________
|  /     |
| /      |
|/      \\|/
|        |
|      
|=============
|___________||___
=================
Lösungswort :    ";

const HANGMAN_8 : &str = "
__________
|  /     |
| /      |
|/      \\|/
|        |
|       / \\
|=============
|___________||___
=================
Lösungswort :    ";

const HANGMAN_FULL : &str = "
_________
|  /     |
| /     (_)
|/      \\|/
|        |
|       / \\
|======\\   ===
|_______\\___||___
=================
Lösungswort :     ";


// ====================================================
// ====================================================
// ====================================================
/*
let alternate_design = "
___________.._______
| .__________))______|
| | / /      ||
| |/ /       ||
| | /        ||.-''.
| |/         |/  _  \
| |          ||  `/,|
| |          (\\`_.'
| |         .-`--'.
| |        /Y . . Y\
| |       // |   | \\
| |      //  | . |  \\
| |     ')   |   |   (`
| |          ||'||
| |          || ||
| |          || ||
| |          || ||
| |         / | | \
""""""""""|_`-' `-' |"""|
|"|"""""""\ \       '"|"|
| |        \ \        | |
: :         \ \       : :  sk
. .          `'       . .
"
*/