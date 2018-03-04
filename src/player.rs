pub struct Player {
    pub name: String,
    pub guesses: u8,
    pub points: u8,
    pub word: String,
    pub guess: String
}


pub fn new_player(name: String) -> Player {
    Player {
        name,
        guesses: 0,
        points: 0,
        word: String::from(""),
        guess: String::from("")
    }
}

