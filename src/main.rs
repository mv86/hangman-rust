mod player;

fn main() {
        
    let mut player1 = player::new_player(String::from("Max"));
    let mut player2 = player::new_player(String::from("Computer"));
    player2.word = String::from("Secret");

    println!("{}", player1.name);
    println!("{}", player2.name);
    println!("{}", player2.word);
}
