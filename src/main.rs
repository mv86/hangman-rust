struct Player {
    name: String,
    guesses: u8,
    points: u8,
    word: String,
    guess: String
}


fn new_player(name: String) -> Player {
    Player {
        name,
        guesses: 0,
        points: 0,
        word: String::from(""),
        guess: String::from("")
    }
}

//fn add_word_choice(player: &Player, word_choice: String) {
//    player.word = word_choice
//}

fn main() {
        
    let mut player1 = new_player(String::from("Max"));
    let mut player2 = new_player(String::from("Computer"));
    player2.word = String::from("Secret");

    println!("{}", player1.name);
    println!("{}", player2.name);
    println!("{}", player2.word);
}
