use dialoguer::{
    Select,
    Input
};
use std::collections::HashMap;
use crate::{Player, PlayerType};
use console::{
    Term
};

pub fn print_welcome() {
    let term = Term::stdout();
    term.write_line(
        "Welcome to zanzibar dice game simulator.\n\
        Rules are simple, to understand gameplay I recommend to just try it.\n\
        All players start with some amount of tokens (default 20).\n\
        First player rolls the dice up to three times (but can decide to end earlier.\n\
        Rest of players try (in as many rolls as first player) to get higher score.\n\
        After each player rolls the dice, each player's score is counted.\n\
        The person with the least points get from each player some number of tokens, depending on highest score of any player.\n\
        \n\
        Available commands:\n\
        exit - to exit game\n\
        scoring - to display rules for counting scores\n\
        players - to display table of players with amount of tokens each currently have\n").unwrap();
}

pub fn get_players() -> HashMap<String, Player> {
    let tokens: usize = Input::new()
        .with_prompt("Enter amount of tokens that each player have on start")
        .with_initial_text("20")
        .interact_text().unwrap();

    let mut players: HashMap<String, Player> = HashMap::new();
    let player_types = vec!["PC", "person"];

    loop {
        let name: String = Input::new()
            .with_prompt("Enter next player name (f to finish choosing players)")
            .with_initial_text("Player")
            .interact_text().unwrap();

        if name == "f".to_string() {
            break;
        }

        let index: usize = Select::new()
            .with_prompt("Choose player's type")
            .default(0)
            .items(&player_types)
            .interact().unwrap();

        let player_type = match player_types[index] {
            "PC" => PlayerType::PC,
            "person" => PlayerType::Person,
            _ => PlayerType::PC
        };

        players.insert(name.clone(), Player::new(name, tokens, player_type));
    };

    players
}