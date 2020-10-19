pub struct LettersGuessed {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
    i: bool,
    j: bool,
    k: bool,
    l: bool,
    m: bool,
    n: bool,
    o: bool,
    p: bool,
    q: bool,
    r: bool,
    s: bool,
    t: bool,
    u: bool,
    v: bool,
    w: bool,
    x: bool,
    y: bool,
    z: bool,
}

pub struct StateOfGame {
    target_word: String,
    word_progress: String,
    num_wrong_guesses: i8,
    letters: LettersGuessed,
}

impl StateOfGame {
    pub fn check_letter(&mut self, letter: char) -> bool
    {
        let &mut char_guessed_before = {
            match letter {
                'a' => &mut self.letters.a,
                'b' => &mut self.letters.b,
                'c' => &mut self.letters.c,
                'd' => &mut self.letters.d,
                'e' => &mut self.letters.e,
                'f' => &mut self.letters.f,
                'g' => &mut self.letters.g,
                'h' => &mut self.letters.h,
                'i' => &mut self.letters.i,
                'j' => &mut self.letters.j,
                'k' => &mut self.letters.k,
                'l' => &mut self.letters.l,
                'm' => &mut self.letters.m,
                'n' => &mut self.letters.n,
                'o' => &mut self.letters.o,
                'p' => &mut self.letters.p,
                'q' => &mut self.letters.q,
                'r' => &mut self.letters.r,
                's' => &mut self.letters.s,
                't' => &mut self.letters.t,
                'u' => &mut self.letters.u,
                'v' => &mut self.letters.v,
                'w' => &mut self.letters.w,
                'x' => &mut self.letters.x,
                'y' => &mut self.letters.y,
                'z' => &mut self.letters.z,
                _ => panic!("That's not a letter in the allowed 26 letter (lower case) alphabet.")
            }
        };
        
        if char_guessed_before 
        {
            return false;
        }
        else 
        {
            let char_in_word = self.target_word.contains(letter);
            // TODO: Finish method
            // Idea: feedback-system based on booleans. 
            // Use extra step via turn.rs to realise a loop only breaking after successful or missed guess 
            if !char_in_word {}
        }
        true
    }
}

pub fn init(word: &str) -> StateOfGame
{
    let mut state = StateOfGame {
        target_word: word.to_string(), 
        word_progress: String::new(),
        num_wrong_guesses: 0, 
        letters: LettersGuessed {
            a: false,
            b: false,
            c: false,
            d: false,
            e: false,
            f: false,
            g: false,
            h: false,
            i: false,
            j: false,
            k: false,
            l: false,
            m: false,
            n: false,
            o: false,
            p: false,
            q: false,
            r: false,
            s: false,
            t: false,
            u: false,
            v: false,
            w: false,
            x: false,
            y: false,
            z: false,
        }
    };
    for _ in word.chars() 
    {
        state.word_progress.push('_');
    }
    state
}